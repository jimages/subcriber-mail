use actix_web::{App, HttpResponse, Responder, HttpServer, web};
use actix_web::dev::Server;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();
    Ok(server)
}
