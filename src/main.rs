use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Panic if config can't be read
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("0.0.0.0:{}", configuration.application_port);

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind(address).expect("Failed to bind to port");
    run(listener, connection_pool)?.await
}
