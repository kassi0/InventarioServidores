use crate::models::{
    AuditLog, CreateServidorInput, Servidor, ServidorQuery, ServidorStats, ServidoresResponse,
    StatItem, UpdateServidorInput,
};
use sqlx::{
    postgres::PgPoolOptions,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    PgPool, Row, SqlitePool,
};
use std::str::FromStr;
use tracing::info;

const SEED_DATA: &str = include_str!("../../seed_servidores.json");

#[derive(Clone)]
pub enum DbPool {
    Sqlite(SqlitePool),
    Postgres(PgPool),
}

impl DbPool {
    pub async fn init(database_url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
            info!("Conectando ao banco de dados PostgreSQL...");
            let pool = PgPoolOptions::new()
                .max_connections(20)
                .connect(database_url)
                .await?;
            let db = DbPool::Postgres(pool);
            db.run_migrations().await?;
            db.seed_if_empty().await?;
            Ok(db)
        } else {
            info!("Conectando ao banco de dados SQLite (dev): {}", database_url);
            let opts = SqliteConnectOptions::from_str(database_url)?
                .create_if_missing(true);
            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect_with(opts)
                .await?;
            let db = DbPool::Sqlite(pool);
            db.run_migrations().await?;
            db.seed_if_empty().await?;
            Ok(db)
        }
    }

    pub async fn run_migrations(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self {
            DbPool::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS servidores (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        item INTEGER,
                        nome TEXT NOT NULL,
                        state TEXT NOT NULL DEFAULT 'POWER ON',
                        hostname TEXT NOT NULL,
                        servico TEXT NOT NULL,
                        equipe_resp TEXT NOT NULL,
                        ambiente TEXT NOT NULL,
                        ip TEXT NOT NULL,
                        memoria_gb TEXT,
                        disco_gb TEXT,
                        cpus TEXT,
                        sistema_operacional TEXT,
                        observacoes TEXT,
                        admin_user TEXT,
                        admin_password TEXT,
                        created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
                        updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
                    );
                    CREATE INDEX IF NOT EXISTS idx_servidores_nome ON servidores(nome);
                    CREATE INDEX IF NOT EXISTS idx_servidores_ip ON servidores(ip);
                    CREATE INDEX IF NOT EXISTS idx_servidores_ambiente ON servidores(ambiente);
                    CREATE INDEX IF NOT EXISTS idx_servidores_equipe ON servidores(equipe_resp);
                    CREATE INDEX IF NOT EXISTS idx_servidores_state ON servidores(state);

                    CREATE TABLE IF NOT EXISTS audit_logs (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        username TEXT NOT NULL,
                        action TEXT NOT NULL,
                        servidor_id INTEGER,
                        servidor_nome TEXT,
                        detalhes TEXT,
                        created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
                    );
                    CREATE INDEX IF NOT EXISTS idx_audit_created_at ON audit_logs(created_at);
                    CREATE INDEX IF NOT EXISTS idx_audit_username ON audit_logs(username);
                    CREATE INDEX IF NOT EXISTS idx_audit_action ON audit_logs(action);
                    "#,
                )
                .execute(pool)
                .await?;

                // Garante que colunas administrativas existam mesmo em bases pré-criadas
                let _ = sqlx::query("ALTER TABLE servidores ADD COLUMN admin_user TEXT").execute(pool).await;
                let _ = sqlx::query("ALTER TABLE servidores ADD COLUMN admin_password TEXT").execute(pool).await;

                info!("Migração SQLite executada com sucesso.");
            }
            DbPool::Postgres(pool) => {
                // 1. Tabela de servidores
                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS servidores (
                        id BIGSERIAL PRIMARY KEY,
                        item INTEGER,
                        nome VARCHAR(255) NOT NULL,
                        state VARCHAR(50) NOT NULL DEFAULT 'POWER ON',
                        hostname VARCHAR(255) NOT NULL,
                        servico VARCHAR(255) NOT NULL,
                        equipe_resp VARCHAR(255) NOT NULL,
                        ambiente VARCHAR(255) NOT NULL,
                        ip VARCHAR(255) NOT NULL,
                        memoria_gb VARCHAR(100),
                        disco_gb VARCHAR(100),
                        cpus VARCHAR(50),
                        sistema_operacional VARCHAR(255),
                        observacoes TEXT,
                        admin_user VARCHAR(255),
                        admin_password VARCHAR(255),
                        created_at VARCHAR(100) NOT NULL DEFAULT (to_char(CURRENT_TIMESTAMP, 'YYYY-MM-DD HH24:MI:SS')),
                        updated_at VARCHAR(100) NOT NULL DEFAULT (to_char(CURRENT_TIMESTAMP, 'YYYY-MM-DD HH24:MI:SS'))
                    )
                    "#,
                )
                .execute(pool)
                .await?;

                // Índices da tabela servidores
                sqlx::query("CREATE INDEX IF NOT EXISTS idx_servidores_nome ON servidores(nome)").execute(pool).await?;
                sqlx::query("CREATE INDEX IF NOT EXISTS idx_servidores_ip ON servidores(ip)").execute(pool).await?;
                sqlx::query("CREATE INDEX IF NOT EXISTS idx_servidores_ambiente ON servidores(ambiente)").execute(pool).await?;
                sqlx::query("CREATE INDEX IF NOT EXISTS idx_servidores_equipe ON servidores(equipe_resp)").execute(pool).await?;
                sqlx::query("CREATE INDEX IF NOT EXISTS idx_servidores_state ON servidores(state)").execute(pool).await?;

                // 2. Tabela de logs de auditoria
                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS audit_logs (
                        id BIGSERIAL PRIMARY KEY,
                        username VARCHAR(255) NOT NULL,
                        action VARCHAR(100) NOT NULL,
                        servidor_id BIGINT,
                        servidor_nome VARCHAR(255),
                        detalhes TEXT,
                        created_at VARCHAR(100) NOT NULL DEFAULT (to_char(CURRENT_TIMESTAMP, 'YYYY-MM-DD HH24:MI:SS'))
                    )
                    "#,
                )
                .execute(pool)
                .await?;

                // Índices da tabela audit_logs
                sqlx::query("CREATE INDEX IF NOT EXISTS idx_audit_created_at ON audit_logs(created_at)").execute(pool).await?;
                sqlx::query("CREATE INDEX IF NOT EXISTS idx_audit_username ON audit_logs(username)").execute(pool).await?;
                sqlx::query("CREATE INDEX IF NOT EXISTS idx_audit_action ON audit_logs(action)").execute(pool).await?;

                // Garante que colunas administrativas existam mesmo se a tabela já foi criada anteriormente
                let _ = sqlx::query("ALTER TABLE servidores ADD COLUMN IF NOT EXISTS admin_user VARCHAR(255)").execute(pool).await;
                let _ = sqlx::query("ALTER TABLE servidores ADD COLUMN IF NOT EXISTS admin_password VARCHAR(255)").execute(pool).await;

                info!("Migração PostgreSQL executada com sucesso.");
            }
        }
        Ok(())
    }

    pub async fn seed_if_empty(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let count: i64 = match self {
            DbPool::Sqlite(pool) => {
                let row = sqlx::query("SELECT COUNT(*) as count FROM servidores")
                    .fetch_one(pool)
                    .await?;
                row.get("count")
            }
            DbPool::Postgres(pool) => {
                let row = sqlx::query("SELECT COUNT(*) as count FROM servidores")
                    .fetch_one(pool)
                    .await?;
                row.get("count")
            }
        };

        if count == 0 {
            info!("Banco de dados vazio. Populando dados iniciais da planilha DETRAN...");
            let raw_records: Vec<serde_json::Value> = serde_json::from_str(SEED_DATA)
                .unwrap_or_default();

            for item in raw_records {
                let nome = item.get("Nome").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
                if nome.is_empty() {
                    continue;
                }
                let item_num: Option<i32> = item.get("Item").and_then(|v| v.as_str()).and_then(|s| s.parse().ok());
                let state = item.get("STATE").and_then(|v| v.as_str()).unwrap_or("POWER ON").trim().to_string();
                let hostname = item.get("Hostname").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
                let servico = item.get("Serviço").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
                let equipe_resp = item.get("Equipe-Resp").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
                let ambiente = item.get("Ambiente").and_then(|v| v.as_str()).unwrap_or("Produção").trim().to_string();
                let ip = item.get("IP").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
                let memoria_gb = item.get("Memoria (GB)").and_then(|v| v.as_str()).map(|s| s.trim().to_string());
                let disco_gb = item.get("Disco (GB)").and_then(|v| v.as_str()).map(|s| s.trim().to_string());
                let cpus = item.get("CPUs").and_then(|v| v.as_str()).map(|s| s.trim().to_string());
                let sistema_operacional = item.get("Sistema Operacional").and_then(|v| v.as_str()).map(|s| s.trim().to_string());

                match self {
                    DbPool::Sqlite(pool) => {
                        sqlx::query(
                            r#"
                            INSERT INTO servidores (item, nome, state, hostname, servico, equipe_resp, ambiente, ip, memoria_gb, disco_gb, cpus, sistema_operacional)
                            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
                            "#,
                        )
                        .bind(item_num)
                        .bind(&nome)
                        .bind(&state)
                        .bind(&hostname)
                        .bind(&servico)
                        .bind(&equipe_resp)
                        .bind(&ambiente)
                        .bind(&ip)
                        .bind(&memoria_gb)
                        .bind(&disco_gb)
                        .bind(&cpus)
                        .bind(&sistema_operacional)
                        .execute(pool)
                        .await?;
                    }
                    DbPool::Postgres(pool) => {
                        sqlx::query(
                            r#"
                            INSERT INTO servidores (item, nome, state, hostname, servico, equipe_resp, ambiente, ip, memoria_gb, disco_gb, cpus, sistema_operacional)
                            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                            "#,
                        )
                        .bind(item_num)
                        .bind(&nome)
                        .bind(&state)
                        .bind(&hostname)
                        .bind(&servico)
                        .bind(&equipe_resp)
                        .bind(&ambiente)
                        .bind(&ip)
                        .bind(&memoria_gb)
                        .bind(&disco_gb)
                        .bind(&cpus)
                        .bind(&sistema_operacional)
                        .execute(pool)
                        .await?;
                    }
                }
            }
            info!("Carga inicial concluída com sucesso!");
        } else {
            info!("Banco de dados já contém {} servidores cadastrados.", count);
        }
        Ok(())
    }

    pub async fn list_servidores(
        &self,
        query: &ServidorQuery,
    ) -> Result<ServidoresResponse, Box<dyn std::error::Error + Send + Sync>> {
        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.per_page.unwrap_or(20).clamp(1, 200);
        let offset = (page - 1) * per_page;

        let q_pattern = query.q.as_deref().map(|s| format!("%{}%", s.trim().to_lowercase()));
        let ambiente_filter = query.ambiente.as_deref().filter(|s| !s.trim().is_empty());
        let equipe_filter = query.equipe.as_deref().filter(|s| !s.trim().is_empty());
        let state_filter = query.state.as_deref().filter(|s| !s.trim().is_empty());

        let sort_col = match query.sort_by.as_deref() {
            Some("nome") => "nome",
            Some("hostname") => "hostname",
            Some("ambiente") => "ambiente",
            Some("equipe_resp") => "equipe_resp",
            Some("state") => "state",
            Some("item") => "item",
            Some("ip") => "ip",
            _ => "id",
        };
        let sort_order = if query.sort_dir.as_deref() == Some("desc") { "DESC" } else { "ASC" };

        match self {
            DbPool::Sqlite(pool) => {
                let count_sql = format!(
                    r#"
                    SELECT COUNT(*) as total FROM servidores
                    WHERE (
                        ?1 IS NULL OR
                        LOWER(nome) LIKE ?1 OR
                        LOWER(hostname) LIKE ?1 OR
                        LOWER(ip) LIKE ?1 OR
                        LOWER(servico) LIKE ?1 OR
                        LOWER(sistema_operacional) LIKE ?1
                    )
                    AND (?2 IS NULL OR ambiente = ?2)
                    AND (?3 IS NULL OR equipe_resp = ?3)
                    AND (?4 IS NULL OR state = ?4)
                    "#
                );

                let total_row = sqlx::query(&count_sql)
                    .bind(&q_pattern)
                    .bind(ambiente_filter)
                    .bind(equipe_filter)
                    .bind(state_filter)
                    .fetch_one(pool)
                    .await?;
                let total: i64 = total_row.get("total");

                let select_sql = format!(
                    r#"
                    SELECT id, item, nome, state, hostname, servico, equipe_resp, ambiente, ip,
                           memoria_gb, disco_gb, cpus, sistema_operacional, observacoes,
                           admin_user, admin_password, created_at, updated_at
                    FROM servidores
                    WHERE (
                        ?1 IS NULL OR
                        LOWER(nome) LIKE ?1 OR
                        LOWER(hostname) LIKE ?1 OR
                        LOWER(ip) LIKE ?1 OR
                        LOWER(servico) LIKE ?1 OR
                        LOWER(sistema_operacional) LIKE ?1
                    )
                    AND (?2 IS NULL OR ambiente = ?2)
                    AND (?3 IS NULL OR equipe_resp = ?3)
                    AND (?4 IS NULL OR state = ?4)
                    ORDER BY {} {}
                    LIMIT ?5 OFFSET ?6
                    "#,
                    sort_col, sort_order
                );

                let rows = sqlx::query_as::<_, Servidor>(&select_sql)
                    .bind(&q_pattern)
                    .bind(ambiente_filter)
                    .bind(equipe_filter)
                    .bind(state_filter)
                    .bind(per_page as i64)
                    .bind(offset as i64)
                    .fetch_all(pool)
                    .await?;

                let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;

                Ok(ServidoresResponse {
                    items: rows,
                    total,
                    page,
                    per_page,
                    total_pages,
                })
            }
            DbPool::Postgres(pool) => {
                let count_sql = format!(
                    r#"
                    SELECT COUNT(*) as total FROM servidores
                    WHERE (
                        $1 IS NULL OR
                        LOWER(nome) LIKE $1 OR
                        LOWER(hostname) LIKE $1 OR
                        LOWER(ip) LIKE $1 OR
                        LOWER(servico) LIKE $1 OR
                        LOWER(sistema_operacional) LIKE $1
                    )
                    AND ($2 IS NULL OR ambiente = $2)
                    AND ($3 IS NULL OR equipe_resp = $3)
                    AND ($4 IS NULL OR state = $4)
                    "#
                );

                let total_row = sqlx::query(&count_sql)
                    .bind(&q_pattern)
                    .bind(ambiente_filter)
                    .bind(equipe_filter)
                    .bind(state_filter)
                    .fetch_one(pool)
                    .await?;
                let total: i64 = total_row.get("total");

                let select_sql = format!(
                    r#"
                    SELECT id, item, nome, state, hostname, servico, equipe_resp, ambiente, ip,
                           memoria_gb, disco_gb, cpus, sistema_operacional, observacoes,
                           admin_user, admin_password, created_at, updated_at
                    FROM servidores
                    WHERE (
                        $1 IS NULL OR
                        LOWER(nome) LIKE $1 OR
                        LOWER(hostname) LIKE $1 OR
                        LOWER(ip) LIKE $1 OR
                        LOWER(servico) LIKE $1 OR
                        LOWER(sistema_operacional) LIKE $1
                    )
                    AND ($2 IS NULL OR ambiente = $2)
                    AND ($3 IS NULL OR equipe_resp = $3)
                    AND ($4 IS NULL OR state = $4)
                    ORDER BY {} {}
                    LIMIT $5 OFFSET $6
                    "#,
                    sort_col, sort_order
                );

                let rows = sqlx::query_as::<_, Servidor>(&select_sql)
                    .bind(&q_pattern)
                    .bind(ambiente_filter)
                    .bind(equipe_filter)
                    .bind(state_filter)
                    .bind(per_page as i64)
                    .bind(offset as i64)
                    .fetch_all(pool)
                    .await?;

                let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;

                Ok(ServidoresResponse {
                    items: rows,
                    total,
                    page,
                    per_page,
                    total_pages,
                })
            }
        }
    }

    pub async fn get_servidor_by_id(&self, id: i64) -> Result<Option<Servidor>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            DbPool::Sqlite(pool) => {
                let res = sqlx::query_as::<_, Servidor>(
                    "SELECT id, item, nome, state, hostname, servico, equipe_resp, ambiente, ip,
                            memoria_gb, disco_gb, cpus, sistema_operacional, observacoes,
                            admin_user, admin_password, created_at, updated_at
                     FROM servidores WHERE id = ?1",
                )
                .bind(id)
                .fetch_optional(pool)
                .await?;
                Ok(res)
            }
            DbPool::Postgres(pool) => {
                let res = sqlx::query_as::<_, Servidor>(
                    "SELECT id, item, nome, state, hostname, servico, equipe_resp, ambiente, ip,
                            memoria_gb, disco_gb, cpus, sistema_operacional, observacoes,
                            admin_user, admin_password, created_at, updated_at
                     FROM servidores WHERE id = $1",
                )
                .bind(id)
                .fetch_optional(pool)
                .await?;
                Ok(res)
            }
        }
    }

    pub async fn create_servidor(&self, input: &CreateServidorInput) -> Result<Servidor, Box<dyn std::error::Error + Send + Sync>> {
        let state = input.state.as_deref().unwrap_or("POWER ON").trim().to_uppercase();
        match self {
            DbPool::Sqlite(pool) => {
                let id = sqlx::query(
                    r#"
                    INSERT INTO servidores (item, nome, state, hostname, servico, equipe_resp, ambiente, ip, memoria_gb, disco_gb, cpus, sistema_operacional, observacoes, admin_user, admin_password)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
                    "#,
                )
                .bind(input.item)
                .bind(&input.nome)
                .bind(&state)
                .bind(&input.hostname)
                .bind(&input.servico)
                .bind(&input.equipe_resp)
                .bind(&input.ambiente)
                .bind(&input.ip)
                .bind(&input.memoria_gb)
                .bind(&input.disco_gb)
                .bind(&input.cpus)
                .bind(&input.sistema_operacional)
                .bind(&input.observacoes)
                .bind(&input.admin_user)
                .bind(&input.admin_password)
                .execute(pool)
                .await?
                .last_insert_rowid();

                self.get_servidor_by_id(id).await?.ok_or_else(|| "Erro ao obter servidor inserido".into())
            }
            DbPool::Postgres(pool) => {
                let row = sqlx::query(
                    r#"
                    INSERT INTO servidores (item, nome, state, hostname, servico, equipe_resp, ambiente, ip, memoria_gb, disco_gb, cpus, sistema_operacional, observacoes, admin_user, admin_password)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
                    RETURNING id
                    "#,
                )
                .bind(input.item)
                .bind(&input.nome)
                .bind(&state)
                .bind(&input.hostname)
                .bind(&input.servico)
                .bind(&input.equipe_resp)
                .bind(&input.ambiente)
                .bind(&input.ip)
                .bind(&input.memoria_gb)
                .bind(&input.disco_gb)
                .bind(&input.cpus)
                .bind(&input.sistema_operacional)
                .bind(&input.observacoes)
                .bind(&input.admin_user)
                .bind(&input.admin_password)
                .fetch_one(pool)
                .await?;

                let id: i64 = row.get("id");
                self.get_servidor_by_id(id).await?.ok_or_else(|| "Erro ao obter servidor inserido".into())
            }
        }
    }

    pub async fn update_servidor(
        &self,
        id: i64,
        input: &UpdateServidorInput,
    ) -> Result<Option<Servidor>, Box<dyn std::error::Error + Send + Sync>> {
        let current = match self.get_servidor_by_id(id).await? {
            Some(s) => s,
            None => return Ok(None),
        };

        let item = input.item.or(current.item);
        let nome = input.nome.as_deref().unwrap_or(&current.nome);
        let state = input.state.as_deref().unwrap_or(&current.state).trim().to_uppercase();
        let hostname = input.hostname.as_deref().unwrap_or(&current.hostname);
        let servico = input.servico.as_deref().unwrap_or(&current.servico);
        let equipe_resp = input.equipe_resp.as_deref().unwrap_or(&current.equipe_resp);
        let ambiente = input.ambiente.as_deref().unwrap_or(&current.ambiente);
        let ip = input.ip.as_deref().unwrap_or(&current.ip);
        let memoria_gb = input.memoria_gb.as_ref().or(current.memoria_gb.as_ref());
        let disco_gb = input.disco_gb.as_ref().or(current.disco_gb.as_ref());
        let cpus = input.cpus.as_ref().or(current.cpus.as_ref());
        let sistema_operacional = input.sistema_operacional.as_ref().or(current.sistema_operacional.as_ref());
        let observacoes = input.observacoes.as_ref().or(current.observacoes.as_ref());
        let admin_user = input.admin_user.as_ref().or(current.admin_user.as_ref());
        let admin_password = input.admin_password.as_ref().or(current.admin_password.as_ref());

        match self {
            DbPool::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    UPDATE servidores SET
                        item = ?1, nome = ?2, state = ?3, hostname = ?4, servico = ?5,
                        equipe_resp = ?6, ambiente = ?7, ip = ?8, memoria_gb = ?9,
                        disco_gb = ?10, cpus = ?11, sistema_operacional = ?12,
                        observacoes = ?13, admin_user = ?14, admin_password = ?15,
                        updated_at = datetime('now', 'localtime')
                    WHERE id = ?16
                    "#,
                )
                .bind(item)
                .bind(nome)
                .bind(&state)
                .bind(hostname)
                .bind(servico)
                .bind(equipe_resp)
                .bind(ambiente)
                .bind(ip)
                .bind(memoria_gb)
                .bind(disco_gb)
                .bind(cpus)
                .bind(sistema_operacional)
                .bind(observacoes)
                .bind(admin_user)
                .bind(admin_password)
                .bind(id)
                .execute(pool)
                .await?;
            }
            DbPool::Postgres(pool) => {
                sqlx::query(
                    r#"
                    UPDATE servidores SET
                        item = $1, nome = $2, state = $3, hostname = $4, servico = $5,
                        equipe_resp = $6, ambiente = $7, ip = $8, memoria_gb = $9,
                        disco_gb = $10, cpus = $11, sistema_operacional = $12,
                        observacoes = $13, admin_user = $14, admin_password = $15,
                        updated_at = to_char(CURRENT_TIMESTAMP, 'YYYY-MM-DD HH24:MI:SS')
                    WHERE id = $16
                    "#,
                )
                .bind(item)
                .bind(nome)
                .bind(&state)
                .bind(hostname)
                .bind(servico)
                .bind(equipe_resp)
                .bind(ambiente)
                .bind(ip)
                .bind(memoria_gb)
                .bind(disco_gb)
                .bind(cpus)
                .bind(sistema_operacional)
                .bind(observacoes)
                .bind(admin_user)
                .bind(admin_password)
                .bind(id)
                .execute(pool)
                .await?;
            }
        }

        self.get_servidor_by_id(id).await
    }

    pub async fn update_credentials(
        &self,
        id: i64,
        admin_user: Option<String>,
        admin_password: Option<String>,
    ) -> Result<Option<Servidor>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            DbPool::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    UPDATE servidores SET
                        admin_user = ?1,
                        admin_password = ?2,
                        updated_at = datetime('now', 'localtime')
                    WHERE id = ?3
                    "#,
                )
                .bind(&admin_user)
                .bind(&admin_password)
                .bind(id)
                .execute(pool)
                .await?;
            }
            DbPool::Postgres(pool) => {
                sqlx::query(
                    r#"
                    UPDATE servidores SET
                        admin_user = $1,
                        admin_password = $2,
                        updated_at = to_char(CURRENT_TIMESTAMP, 'YYYY-MM-DD HH24:MI:SS')
                    WHERE id = $3
                    "#,
                )
                .bind(&admin_user)
                .bind(&admin_password)
                .bind(id)
                .execute(pool)
                .await?;
            }
        }
        self.get_servidor_by_id(id).await
    }

    pub async fn create_audit_log(
        &self,
        username: &str,
        action: &str,
        servidor_id: Option<i64>,
        servidor_nome: Option<&str>,
        detalhes: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self {
            DbPool::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO audit_logs (username, action, servidor_id, servidor_nome, detalhes)
                    VALUES (?1, ?2, ?3, ?4, ?5)
                    "#,
                )
                .bind(username)
                .bind(action)
                .bind(servidor_id)
                .bind(servidor_nome)
                .bind(detalhes)
                .execute(pool)
                .await?;
            }
            DbPool::Postgres(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO audit_logs (username, action, servidor_id, servidor_nome, detalhes)
                    VALUES ($1, $2, $3, $4, $5)
                    "#,
                )
                .bind(username)
                .bind(action)
                .bind(servidor_id)
                .bind(servidor_nome)
                .bind(detalhes)
                .execute(pool)
                .await?;
            }
        }
        Ok(())
    }

    pub async fn list_audit_logs(
        &self,
        limit: i64,
    ) -> Result<Vec<AuditLog>, Box<dyn std::error::Error + Send + Sync>> {
        let lim = limit.clamp(1, 200);
        match self {
            DbPool::Sqlite(pool) => {
                let rows = sqlx::query_as::<_, AuditLog>(
                    "SELECT id, username, action, servidor_id, servidor_nome, detalhes, created_at
                     FROM audit_logs
                     ORDER BY id DESC
                     LIMIT ?1",
                )
                .bind(lim)
                .fetch_all(pool)
                .await?;
                Ok(rows)
            }
            DbPool::Postgres(pool) => {
                let rows = sqlx::query_as::<_, AuditLog>(
                    "SELECT id, username, action, servidor_id, servidor_nome, detalhes, created_at
                     FROM audit_logs
                     ORDER BY id DESC
                     LIMIT $1",
                )
                .bind(lim)
                .fetch_all(pool)
                .await?;
                Ok(rows)
            }
        }
    }

    pub async fn delete_servidor(&self, id: i64) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let rows_affected = match self {
            DbPool::Sqlite(pool) => {
                sqlx::query("DELETE FROM servidores WHERE id = ?1")
                    .bind(id)
                    .execute(pool)
                    .await?
                    .rows_affected()
            }
            DbPool::Postgres(pool) => {
                sqlx::query("DELETE FROM servidores WHERE id = $1")
                    .bind(id)
                    .execute(pool)
                    .await?
                    .rows_affected()
            }
        };
        Ok(rows_affected > 0)
    }

    pub async fn get_all_for_export(&self) -> Result<Vec<Servidor>, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            DbPool::Sqlite(pool) => {
                let rows = sqlx::query_as::<_, Servidor>(
                    "SELECT id, item, nome, state, hostname, servico, equipe_resp, ambiente, ip,
                            memoria_gb, disco_gb, cpus, sistema_operacional, observacoes,
                            admin_user, admin_password, created_at, updated_at
                     FROM servidores ORDER BY id ASC",
                )
                .fetch_all(pool)
                .await?;
                Ok(rows)
            }
            DbPool::Postgres(pool) => {
                let rows = sqlx::query_as::<_, Servidor>(
                    "SELECT id, item, nome, state, hostname, servico, equipe_resp, ambiente, ip,
                            memoria_gb, disco_gb, cpus, sistema_operacional, observacoes,
                            admin_user, admin_password, created_at, updated_at
                     FROM servidores ORDER BY id ASC",
                )
                .fetch_all(pool)
                .await?;
                Ok(rows)
            }
        }
    }

    pub async fn get_stats(&self) -> Result<ServidorStats, Box<dyn std::error::Error + Send + Sync>> {
        let servers = self.get_all_for_export().await?;

        let total_servidores = servers.len() as i64;
        let mut total_power_on = 0i64;
        let mut total_power_off = 0i64;
        let mut total_cpus = 0i64;
        let mut total_memoria_gb = 0.0f64;
        let mut total_disco_gb = 0.0f64;

        let mut ambiente_counts: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        let mut equipe_counts: std::collections::HashMap<String, i64> = std::collections::HashMap::new();

        for s in &servers {
            let state_upper = s.state.trim().to_uppercase();
            if state_upper.contains("OFF") {
                total_power_off += 1;
            } else {
                total_power_on += 1;
            }

            let amb = if s.ambiente.trim().is_empty() { "Outros" } else { s.ambiente.trim() };
            *ambiente_counts.entry(amb.to_string()).or_insert(0) += 1;

            let eq = if s.equipe_resp.trim().is_empty() { "Não Definida" } else { s.equipe_resp.trim() };
            *equipe_counts.entry(eq.to_string()).or_insert(0) += 1;

            // Parse CPUs
            if let Some(cpus_str) = &s.cpus {
                let num_str: String = cpus_str.chars().filter(|c| c.is_ascii_digit()).collect();
                if let Ok(val) = num_str.parse::<i64>() {
                    total_cpus += val;
                }
            }

            // Parse RAM (e.g. "8", "32 GB")
            if let Some(mem_str) = &s.memoria_gb {
                let clean: String = mem_str
                    .chars()
                    .filter(|c| c.is_ascii_digit() || *c == '.' || *c == ',')
                    .collect::<String>()
                    .replace(',', ".");
                if let Ok(val) = clean.parse::<f64>() {
                    total_memoria_gb += val;
                }
            }

            // Parse Disk (e.g. "64.11", "160")
            if let Some(disk_str) = &s.disco_gb {
                let clean: String = disk_str
                    .chars()
                    .filter(|c| c.is_ascii_digit() || *c == '.' || *c == ',')
                    .collect::<String>()
                    .replace(',', ".");
                if let Ok(val) = clean.parse::<f64>() {
                    total_disco_gb += val;
                }
            }
        }

        let mut por_ambiente: Vec<StatItem> = ambiente_counts
            .into_iter()
            .map(|(name, count)| StatItem { name, count })
            .collect();
        por_ambiente.sort_by(|a, b| b.count.cmp(&a.count));

        let mut por_equipe: Vec<StatItem> = equipe_counts
            .into_iter()
            .map(|(name, count)| StatItem { name, count })
            .collect();
        por_equipe.sort_by(|a, b| b.count.cmp(&a.count));

        Ok(ServidorStats {
            total_servidores,
            total_power_on,
            total_power_off,
            total_cpus,
            total_memoria_gb: (total_memoria_gb * 10.0).round() / 10.0,
            total_disco_gb: (total_disco_gb * 10.0).round() / 10.0,
            por_ambiente,
            por_equipe,
        })
    }
}
