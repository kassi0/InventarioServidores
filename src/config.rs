use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub is_postgres: bool,
    pub ldap_host: String,
    pub ldap_port: u16,
    pub ldap_login: String,
    pub ldap_senha: String,
    pub ldap_grupos: Vec<String>,
    pub jwt_secret: String,
    pub dev_mode: bool,
}

impl Config {
    pub fn from_env() -> Self {
        // Load .env file if present
        let _ = dotenvy::dotenv();

        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);

        let use_sqlite = env::var("USE_SQLITE")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);

        let database_url = if use_sqlite {
            "sqlite://servidores.db".to_string()
        } else if let Ok(url) = env::var("DATABASE_URL") {
            url
        } else if let Ok(db) = env::var("POSTGRES_DB") {
            let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "postgres.detran-apps-internos.svc.cluster.local".to_string());
            let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
            let user = env::var("POSTGRES_USER").unwrap_or_else(|_| "inventario".to_string());
            let pwd = env::var("POSTGRES_PWD")
                .or_else(|_| env::var("POSTGRES_PASSWORD"))
                .unwrap_or_else(|_| "pwdinventario".to_string());
            format!("postgres://{}:{}@{}:{}/{}", user, pwd, host, port, db)
        } else {
            "sqlite://servidores.db".to_string()
        };
        let is_postgres = database_url.starts_with("postgres://") || database_url.starts_with("postgresql://");

        let ldap_host = env::var("LDAP").unwrap_or_else(|_| "10.21.246.3".to_string());
        let ldap_port = env::var("LDAP_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(389);
        let ldap_login = env::var("LDAP_LOGIN").unwrap_or_else(|_| "sas-admin@detrandom.tran".to_string());
        let ldap_senha = env::var("LDAP_SENHA").unwrap_or_else(|_| "Detr@nBA2025!".to_string());

        // Parse LDAP_GRUPOS: e.g. "OSRH-DES-PROD | OSRH-INFRA | OSRH-DES-COD"
        let grupos_raw = env::var("LDAP_GRUPOS")
            .unwrap_or_else(|_| "OSRH-DES-PROD | OSRH-INFRA | OSRH-DES-COD".to_string());
        let ldap_grupos: Vec<String> = grupos_raw
            .split('|')
            .map(|g| g.trim().to_string())
            .filter(|g| !g.is_empty())
            .collect();

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "detran-inventario-super-secret-key-2026".to_string());
        let dev_mode = env::var("DEV_MODE")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(!is_postgres);

        Config {
            port,
            database_url,
            is_postgres,
            ldap_host,
            ldap_port,
            ldap_login,
            ldap_senha,
            ldap_grupos,
            jwt_secret,
            dev_mode,
        }
    }
}
