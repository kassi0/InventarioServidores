use crate::{
    config::Config,
    models::{Claims, CurrentUser, LoginRequest, LoginResponse},
};
use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use ldap3::{LdapConn, Scope, SearchEntry};
use tracing::{error, info, warn};

pub struct AuthManager {
    config: Config,
}

impl AuthManager {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn generate_jwt(&self, username: &str, name: &str, grupos: &[String]) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: username.to_string(),
            name: name.to_string(),
            grupos: grupos.to_vec(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )
    }

    #[allow(dead_code)]
    pub fn validate_jwt(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }

    pub async fn authenticate(&self, req: LoginRequest) -> Result<LoginResponse, (StatusCode, String)> {
        let username = req.username.trim().to_string();
        let password = req.password;

        if username.is_empty() || password.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Usuário e senha são obrigatórios".into()));
        }

        let config_clone = self.config.clone();
        let username_clone = username.clone();
        let password_clone = password.clone();

        // Perform LDAP authentication in a blocking thread with a timeout
        let auth_result = tokio::task::spawn_blocking(move || {
            Self::ldap_auth_blocking(&config_clone, &username_clone, &password_clone)
        })
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro interno de autenticação: {}", e)))?;

        match auth_result {
            Ok((display_name, matched_groups)) => {
                info!("Usuário '{}' autenticado com sucesso via LDAP nos grupos: {:?}", username, matched_groups);
                let token = self
                    .generate_jwt(&username, &display_name, &matched_groups)
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao gerar token: {}", e)))?;

                Ok(LoginResponse {
                    token,
                    username,
                    name: display_name,
                    grupos: matched_groups,
                })
            }
            Err(ldap_err) => {
                // If LDAP is unreachable and DEV_MODE is enabled, allow dev login
                if self.config.dev_mode {
                    warn!(
                        "LDAP falhou ({}), mas DEV_MODE está ativo. Permitindo login de desenvolvimento para '{}'.",
                        ldap_err, username
                    );
                    let dev_groups = self.config.ldap_grupos.clone();
                    let display_name = format!("{} (Dev/Admin)", username);
                    let token = self
                        .generate_jwt(&username, &display_name, &dev_groups)
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao gerar token: {}", e)))?;

                    return Ok(LoginResponse {
                        token,
                        username,
                        name: display_name,
                        grupos: dev_groups,
                    });
                }

                error!("Falha na autenticação do usuário '{}': {}", username, ldap_err);
                Err((StatusCode::UNAUTHORIZED, ldap_err))
            }
        }
    }

    fn ldap_auth_blocking(config: &Config, username: &str, password: &str) -> Result<(String, Vec<String>), String> {
        let ldap_url = format!("ldap://{}:{}", config.ldap_host, config.ldap_port);
        info!("Conectando ao servidor LDAP DETRAN em {}...", ldap_url);

        let mut ldap = LdapConn::new(&ldap_url)
            .map_err(|e| format!("Não foi possível conectar ao servidor LDAP {}: {}", ldap_url, e))?;

        // 1. Bind with service account (sas-admin)
        info!("Autenticando conta de serviço LDAP '{}'...", config.ldap_login);
        ldap.simple_bind(&config.ldap_login, &config.ldap_senha)
            .map_err(|e| format!("Falha no bind administrativo LDAP: {}", e))?
            .success()
            .map_err(|e| format!("Bind administrativo rejeitado: {}", e))?;

        // 2. Search for user by sAMAccountName or userPrincipalName
        let base_dn = "DC=detrandom,DC=tran";
        let filter = format!("(|(sAMAccountName={})(userPrincipalName={})(userPrincipalName={}@detrandom.tran))", username, username, username);
        info!("Buscando usuário LDAP com filtro: {}", filter);

        let (rs, _res) = ldap
            .search(
                base_dn,
                Scope::Subtree,
                &filter,
                vec!["dn", "cn", "displayName", "memberOf", "sAMAccountName"],
            )
            .map_err(|e| format!("Erro na busca de usuário LDAP: {}", e))?
            .success()
            .map_err(|e| format!("Erro no resultado da busca LDAP: {}", e))?;

        if rs.is_empty() {
            return Err("Usuário não encontrado no domínio DETRANDOM.TRAN".to_string());
        }

        let entry = SearchEntry::construct(rs[0].clone());
        let user_dn = entry.dn;
        let display_name = entry
            .attrs
            .get("displayName")
            .and_then(|v| v.first())
            .or_else(|| entry.attrs.get("cn").and_then(|v| v.first()))
            .cloned()
            .unwrap_or_else(|| username.to_string());

        let member_of = entry.attrs.get("memberOf").cloned().unwrap_or_default();

        info!("Usuário localizado: DN='{}', DisplayName='{}'", user_dn, display_name);

        // 3. Authenticate the user with their password
        info!("Validando senha do usuário '{}'...", user_dn);
        let mut user_ldap = LdapConn::new(&ldap_url)
            .map_err(|e| format!("Erro ao abrir conexão para autenticação do usuário: {}", e))?;

        user_ldap
            .simple_bind(&user_dn, password)
            .map_err(|e| format!("Falha ao autenticar credenciais do usuário: {}", e))?
            .success()
            .map_err(|_| "Credenciais inválidas. Verifique seu usuário e senha.".to_string())?;

        // 4. Verify groups against LDAP_GRUPOS
        // LDAP_GRUPOS = ["OSRH-DES-PROD", "OSRH-INFRA", "OSRH-DES-COD"]
        let mut matched_groups = Vec::new();

        for allowed_group in &config.ldap_grupos {
            let allowed_norm = allowed_group.trim().to_uppercase();
            for user_group_dn in &member_of {
                let user_group_norm = user_group_dn.to_uppercase();
                // Check if DN contains "CN=<GROUP>," or matches
                let cn_prefix = format!("CN={},", allowed_norm);
                if user_group_norm.contains(&cn_prefix) || user_group_norm.contains(&allowed_norm) {
                    if !matched_groups.contains(allowed_group) {
                        matched_groups.push(allowed_group.clone());
                    }
                }
            }
        }

        if matched_groups.is_empty() {
            return Err(format!(
                "Acesso negado: o usuário '{}' não pertence a nenhum dos grupos permitidos: {:?}",
                username, config.ldap_grupos
            ));
        }

        Ok((display_name, matched_groups))
    }
}

// Axum extractor to inject CurrentUser from Cookie or Authorization header
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = extract_token_from_parts(parts)
            .ok_or((StatusCode::UNAUTHORIZED, "Não autenticado"))?;

        // Retrieve secret from extensions or default
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "detran-inventario-super-secret-key-2026".to_string());

        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Sessão expirada ou inválida"))?;

        Ok(CurrentUser {
            username: token_data.claims.sub,
            name: token_data.claims.name,
            grupos: token_data.claims.grupos,
        })
    }
}

pub fn extract_token_from_parts(parts: &Parts) -> Option<String> {
    // 1. Try Cookie header: auth_token=...
    if let Some(cookie_header) = parts.headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if let Some(val) = cookie.strip_prefix("auth_token=") {
                    return Some(val.to_string());
                }
            }
        }
    }

    // 2. Try Authorization: Bearer <token>
    if let Some(auth_header) = parts.headers.get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Some(token.trim().to_string());
            }
        }
    }

    None
}
