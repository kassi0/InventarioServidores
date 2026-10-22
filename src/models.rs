use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Servidor {
    pub id: i64,
    pub item: Option<i32>,
    pub nome: String,
    pub state: String,
    pub hostname: String,
    pub servico: String,
    pub equipe_resp: String,
    pub ambiente: String,
    pub ip: String,
    pub memoria_gb: Option<String>,
    pub disco_gb: Option<String>,
    pub cpus: Option<String>,
    pub sistema_operacional: Option<String>,
    pub observacoes: Option<String>,
    pub admin_user: Option<String>,
    pub admin_password: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateServidorInput {
    pub item: Option<i32>,
    pub nome: String,
    pub state: Option<String>,
    pub hostname: String,
    pub servico: String,
    pub equipe_resp: String,
    pub ambiente: String,
    pub ip: String,
    pub memoria_gb: Option<String>,
    pub disco_gb: Option<String>,
    pub cpus: Option<String>,
    pub sistema_operacional: Option<String>,
    pub observacoes: Option<String>,
    pub admin_user: Option<String>,
    pub admin_password: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateServidorInput {
    pub item: Option<i32>,
    pub nome: Option<String>,
    pub state: Option<String>,
    pub hostname: Option<String>,
    pub servico: Option<String>,
    pub equipe_resp: Option<String>,
    pub ambiente: Option<String>,
    pub ip: Option<String>,
    pub memoria_gb: Option<String>,
    pub disco_gb: Option<String>,
    pub cpus: Option<String>,
    pub sistema_operacional: Option<String>,
    pub observacoes: Option<String>,
    pub admin_user: Option<String>,
    pub admin_password: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateCredentialsInput {
    pub admin_user: Option<String>,
    pub admin_password: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuditLog {
    pub id: i64,
    pub username: String,
    pub action: String,
    pub servidor_id: Option<i64>,
    pub servidor_nome: Option<String>,
    pub detalhes: Option<String>,
    pub created_at: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct ServidorQuery {
    pub q: Option<String>,
    pub ambiente: Option<String>,
    pub equipe: Option<String>,
    pub state: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ServidoresResponse {
    pub items: Vec<Servidor>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatItem {
    pub name: String,
    pub count: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServidorStats {
    pub total_servidores: i64,
    pub total_power_on: i64,
    pub total_power_off: i64,
    pub total_cpus: i64,
    pub total_memoria_gb: f64,
    pub total_disco_gb: f64,
    pub por_ambiente: Vec<StatItem>,
    pub por_equipe: Vec<StatItem>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
    pub name: String,
    pub grupos: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub grupos: Vec<String>,
    pub exp: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CurrentUser {
    pub username: String,
    pub name: String,
    pub grupos: Vec<String>,
}
