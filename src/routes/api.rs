use crate::{
    models::{
        CreateServidorInput, CurrentUser, ServidorQuery, UpdateCredentialsInput,
        UpdateServidorInput,
    },
    routes::auth::AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;
use tracing::info;

pub async fn list_servidores_handler(
    State(state): State<Arc<AppState>>,
    _user: CurrentUser,
    Query(query): Query<ServidorQuery>,
) -> impl IntoResponse {
    match state.db.list_servidores(&query).await {
        Ok(resp) => (StatusCode::OK, Json(json!(resp))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Erro ao listar servidores: {}", e) })),
        )
            .into_response(),
    }
}

pub async fn get_servidor_handler(
    State(state): State<Arc<AppState>>,
    _user: CurrentUser,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match state.db.get_servidor_by_id(id).await {
        Ok(Some(s)) => (StatusCode::OK, Json(json!(s))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Servidor não encontrado" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Erro ao buscar servidor: {}", e) })),
        )
            .into_response(),
    }
}

pub async fn create_servidor_handler(
    State(state): State<Arc<AppState>>,
    user: CurrentUser,
    Json(input): Json<CreateServidorInput>,
) -> impl IntoResponse {
    if input.nome.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "O campo Nome é obrigatório" })),
        )
            .into_response();
    }

    match state.db.create_servidor(&input).await {
        Ok(s) => {
            let detalhes = format!(
                "Servidor '{}' criado no ambiente '{}' com IP '{}' (Equipe: {})",
                s.nome, s.ambiente, s.ip, s.equipe_resp
            );
            let _ = state
                .db
                .create_audit_log(
                    &user.username,
                    "CRIAR_SERVIDOR",
                    Some(s.id),
                    Some(&s.nome),
                    Some(&detalhes),
                )
                .await;
            info!(
                "AUDIT: Usuário '{}' CRIOU servidor '{}' (id: {})",
                user.username, s.nome, s.id
            );
            (StatusCode::CREATED, Json(json!(s))).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Erro ao criar servidor: {}", e) })),
        )
            .into_response(),
    }
}

pub async fn update_servidor_handler(
    State(state): State<Arc<AppState>>,
    user: CurrentUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateServidorInput>,
) -> impl IntoResponse {
    match state.db.update_servidor(id, &input).await {
        Ok(Some(s)) => {
            let detalhes = format!(
                "Dados do servidor '{}' (IP: {}, Ambiente: {}) foram atualizados",
                s.nome, s.ip, s.ambiente
            );
            let _ = state
                .db
                .create_audit_log(
                    &user.username,
                    "EDITAR_SERVIDOR",
                    Some(s.id),
                    Some(&s.nome),
                    Some(&detalhes),
                )
                .await;
            info!(
                "AUDIT: Usuário '{}' EDITOU servidor '{}' (id: {})",
                user.username, s.nome, s.id
            );
            (StatusCode::OK, Json(json!(s))).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Servidor não encontrado" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Erro ao atualizar servidor: {}", e) })),
        )
            .into_response(),
    }
}

pub async fn update_credentials_handler(
    State(state): State<Arc<AppState>>,
    user: CurrentUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateCredentialsInput>,
) -> impl IntoResponse {
    match state.db.update_credentials(id, input.admin_user, input.admin_password).await {
        Ok(Some(s)) => {
            let detalhes = format!(
                "Credenciais administrativas do servidor '{}' foram cadastradas/atualizadas",
                s.nome
            );
            let _ = state
                .db
                .create_audit_log(
                    &user.username,
                    "ATUALIZAR_CREDENCIAIS",
                    Some(s.id),
                    Some(&s.nome),
                    Some(&detalhes),
                )
                .await;
            info!(
                "AUDIT: Usuário '{}' ATUALIZOU CREDENCIAIS do servidor '{}' (id: {})",
                user.username, s.nome, s.id
            );
            (StatusCode::OK, Json(json!(s))).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Servidor não encontrado" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Erro ao atualizar credenciais: {}", e) })),
        )
            .into_response(),
    }
}

pub async fn list_audit_logs_handler(
    State(state): State<Arc<AppState>>,
    _user: CurrentUser,
) -> impl IntoResponse {
    match state.db.list_audit_logs(100).await {
        Ok(logs) => (StatusCode::OK, Json(json!(logs))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Erro ao listar logs de auditoria: {}", e) })),
        )
            .into_response(),
    }
}

pub async fn delete_servidor_handler(
    State(state): State<Arc<AppState>>,
    user: CurrentUser,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let server_name = state
        .db
        .get_servidor_by_id(id)
        .await
        .ok()
        .flatten()
        .map(|s| s.nome);
    let server_name_str = server_name.as_deref().unwrap_or("Desconhecido");

    match state.db.delete_servidor(id).await {
        Ok(true) => {
            let detalhes = format!(
                "Servidor '{}' (id: {}) foi excluído permanentemente do inventário",
                server_name_str, id
            );
            let _ = state
                .db
                .create_audit_log(
                    &user.username,
                    "EXCLUIR_SERVIDOR",
                    Some(id),
                    Some(server_name_str),
                    Some(&detalhes),
                )
                .await;
            info!(
                "AUDIT: Usuário '{}' EXCLUIU servidor '{}' (id: {})",
                user.username, server_name_str, id
            );
            (
                StatusCode::OK,
                Json(json!({ "message": "Servidor excluído com sucesso" })),
            )
                .into_response()
        }
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Servidor não encontrado" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Erro ao excluir servidor: {}", e) })),
        )
            .into_response(),
    }
}

pub async fn get_stats_handler(
    State(state): State<Arc<AppState>>,
    _user: CurrentUser,
) -> impl IntoResponse {
    match state.db.get_stats().await {
        Ok(stats) => (StatusCode::OK, Json(json!(stats))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Erro ao calcular estatísticas: {}", e) })),
        )
            .into_response(),
    }
}

pub async fn export_csv_handler(
    State(state): State<Arc<AppState>>,
    _user: CurrentUser,
) -> impl IntoResponse {
    match state.db.get_all_for_export().await {
        Ok(servers) => {
            let mut csv = String::new();
            // UTF-8 BOM so Excel opens with proper accents
            csv.push('\u{FEFF}');
            csv.push_str("Item;Nome;Status;Hostname;Serviço;Equipe Responsável;Ambiente;Endereço IP;Memória (GB);Disco (GB);CPUs;Sistema Operacional;Observações\n");

            for s in servers {
                let item = s.item.map(|i| i.to_string()).unwrap_or_default();
                let mem = s.memoria_gb.unwrap_or_default();
                let disco = s.disco_gb.unwrap_or_default();
                let cpus = s.cpus.unwrap_or_default();
                let so = s.sistema_operacional.unwrap_or_default();
                let obs = s.observacoes.unwrap_or_default().replace(';', ",");

                csv.push_str(&format!(
                    "{};\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\";\"{}\"\n",
                    item,
                    s.nome.replace('"', "\"\""),
                    s.state.replace('"', "\"\""),
                    s.hostname.replace('"', "\"\""),
                    s.servico.replace('"', "\"\""),
                    s.equipe_resp.replace('"', "\"\""),
                    s.ambiente.replace('"', "\"\""),
                    s.ip.replace('"', "\"\""),
                    mem.replace('"', "\"\""),
                    disco.replace('"', "\"\""),
                    cpus.replace('"', "\"\""),
                    so.replace('"', "\"\""),
                    obs.replace('"', "\"\""),
                ));
            }

            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/csv; charset=utf-8"),
            );
            headers.insert(
                header::CONTENT_DISPOSITION,
                HeaderValue::from_static("attachment; filename=\"inventario_servidores_detran.csv\""),
            );

            (StatusCode::OK, headers, csv).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Erro ao exportar dados: {}", e) })),
        )
            .into_response(),
    }
}
