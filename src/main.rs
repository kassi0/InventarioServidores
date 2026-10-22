mod auth;
mod config;
mod db;
mod models;
mod routes;

use auth::AuthManager;
use config::Config;
use db::DbPool;
use routes::auth::AppState;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "inventario_servidores=debug,tower_http=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Iniciando DETRAN-BA - Sistema de Inventário de Servidores...");

    let config = Config::from_env();
    info!("Modo de Execução: Dev={}", config.dev_mode);
    info!("Banco de Dados: {}", if config.is_postgres { "PostgreSQL" } else { "SQLite (dev)" });
    info!("LDAP Host: {}:{}", config.ldap_host, config.ldap_port);
    info!("LDAP Admin Bind: {}", config.ldap_login);
    info!("LDAP Grupos Autorizados: {:?}", config.ldap_grupos);

    // Initialize Database & run auto-migration/seed
    let db = DbPool::init(&config.database_url).await?;

    let auth = Arc::new(AuthManager::new(config.clone()));

    let state = Arc::new(AppState {
        db,
        auth,
        config: config.clone(),
    });

    let app = routes::create_router(state);

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("🚀 Servidor web escutando em http://{}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Servidor finalizado com sucesso.");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Falha ao registrar listener de Ctrl+C");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Falha ao registrar listener de SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Sinal de interrupção Ctrl+C recebido, encerrando graciosamente...");
        },
        _ = terminate => {
            info!("Sinal SIGTERM recebido do OpenShift/Kubernetes, encerrando graciosamente...");
        },
    }
}
