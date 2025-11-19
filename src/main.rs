use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;

use zero2prod::{
    configuration::get_config,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuration");
    let conn_pool = PgPool::connect(&config.db.connection_string().expose_secret())
        .await
        .expect("Error connecting to pool");

    let addr = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(addr)?;

    println!("Server running on http://127.0.0.1:8000");
    run(listener, conn_pool)?.await
}
