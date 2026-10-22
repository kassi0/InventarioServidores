use crate::{
    auth::AuthManager,
    config::Config,
    models::{CurrentUser, LoginRequest},
};
use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

pub struct AppState {
    pub db: crate::db::DbPool,
    pub auth: Arc<AuthManager>,
    #[allow(dead_code)]
    pub config: Config,
}

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let username = payload.username.clone();
    match state.auth.authenticate(payload).await {
        Ok(resp) => {
            let cookie_val = format!(
                "auth_token={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=86400",
                resp.token
            );
            let mut headers = HeaderMap::new();
            if let Ok(hv) = HeaderValue::from_str(&cookie_val) {
                headers.insert(header::SET_COOKIE, hv);
            }

            let detalhes = format!("Usuário '{}' autenticado com sucesso no sistema", username);
            let _ = state
                .db
                .create_audit_log(&username, "LOGIN_LDAP", None, None, Some(&detalhes))
                .await;
            tracing::info!("AUDIT: Usuário '{}' realizou login com sucesso", username);

            (StatusCode::OK, headers, Json(json!(resp))).into_response()
        }
        Err((code, msg)) => {
            tracing::warn!("AUDIT: Falha de autenticação para usuário '{}': {}", username, msg);
            (code, Json(json!({ "error": msg }))).into_response()
        }
    }
}

pub async fn logout_handler() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    let cookie_val = "auth_token=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0";
    if let Ok(hv) = HeaderValue::from_str(cookie_val) {
        headers.insert(header::SET_COOKIE, hv);
    }
    (StatusCode::OK, headers, Json(json!({ "message": "Logout realizado com sucesso" }))).into_response()
}

pub async fn me_handler(user_res: Result<CurrentUser, (StatusCode, &'static str)>) -> impl IntoResponse {
    match user_res {
        Ok(u) => (StatusCode::OK, Json(json!(u))).into_response(),
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Não autenticado" })),
        )
            .into_response(),
    }
}
