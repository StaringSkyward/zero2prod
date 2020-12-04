use crate::routes::{health_check, subscribe};

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
  // Wrap the connection_pool in an Atomic Reference Counter
  let connection_pool = web::Data::new(connection_pool);

  let server = HttpServer::new(move || {
    App::new()
      .route("/health_check", web::get().to(health_check))
      .route("/subscriptions", web::post().to(subscribe))
      // Register the connection as part of the application state
      .app_data(connection_pool.clone())
  })
  .listen(listener)?
  .run();

  Ok(server)
}
