use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // `init` does call `set_logger`, so this is all we need to do.
    // We are falling back to printing all logs at info-level or above
    // if the RUST_LOG environment variable has not been set
    let subscriber = get_subscriber(
        "zero2prod".into(),
        "info".into(),
        || std::io::stdout()
    );
    init_subscriber(subscriber);

    //init configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
        // .expect("Failed to connect to Postgres.");

    let address = format!(
        "{}:{}",
        configuration.application.host ,
        configuration.application.port
    );
    println!("Finance service running {}", address);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}