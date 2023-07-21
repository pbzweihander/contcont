use migration::MigratorTrait;
use sea_orm::Database;

mod config;
mod entity;
mod handler;
mod utils;

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .init();

    let db = Database::connect(format!(
        "sqlite://{}",
        crate::config::CONFIG.database_file_path
    ))
    .await?;

    migration::Migrator::up(&db, None).await?;

    let router = crate::handler::create_router(db);

    let listen_addr = &crate::config::CONFIG.listen_addr;
    tracing::info!(%listen_addr, "starting http server...");
    axum::Server::bind(&listen_addr.parse()?)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
