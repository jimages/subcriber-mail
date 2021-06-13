use subcriber_mail::run;
use subcriber_mail::configuration::get_configuration;
use std::net;
use sqlx::PgPool;

// 在正式软件中启动服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let listener = net::TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
