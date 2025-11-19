use std::net::TcpListener;

use sqlx::PgPool;

use zero2prod::configuration::get_config;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read configuration");
    let conn_pool = PgPool::connect(&config.db.connection_string())
        .await
        .expect("Error connecting to pool");

    let addr = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(addr)?;

    println!("Server running on http://127.0.0.1:8000");
    run(listener, conn_pool)?.await
}
