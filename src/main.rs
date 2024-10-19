use axum::routing::get;
use tokio::net::TcpListener;

type Pool = sqlx::SqlitePool;

#[derive(Debug, Clone, axum::extract::FromRef)]
struct App {
    pool: Pool,
}

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("chat=trace")
        .pretty()
        .init();
    tracing::info!("tracing initialized");
    tokio::runtime::Runtime::new()?.block_on(run_server())?;
    Ok(())
}

async fn run_server() -> eyre::Result<()> {
    dotenvy::dotenv().expect("missing .env file");
    let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let address = std::env::var("ADDRESS").expect("missing ADDRESS");

    tracing::info!(address, database_url);

    let pool = sqlx::SqlitePool::connect(&database_url).await?;

    let router = axum::Router::new()
        .route("/", get(|| async move { "Hello, World" }))
        .with_state(App { pool });
    let tcp = TcpListener::bind(&address).await?;

    axum::serve(tcp, router).await?;
    Ok(())
}
