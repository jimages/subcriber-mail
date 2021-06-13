pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

use std::net::TcpListener;
pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .service(routes::subscriptions)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
