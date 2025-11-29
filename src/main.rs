use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;

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
    let conn_pool = PgPoolOptions::new()
        .idle_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.db.with_db());

    let addr = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(addr)?;

    println!(
        "Server running on http://{}:{}",
        config.application.host, config.application.port
    );
    run(listener, conn_pool)?.await
}
