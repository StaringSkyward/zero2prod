use std::net::TcpListener;
use zero2prod::run;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind("0.0.0.0:8080")
        .expect("Failed to bind to port 8080");
    run(listener)?.await
}
