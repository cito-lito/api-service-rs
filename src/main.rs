mod server;
use server::Server;


use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    // std::env::set_var("DATABASE_URL", "postgres://postgres:postgres@localhost:5432/app_db");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let server = Server::new("127.0.0.1".to_string(), 3003);
    server.run().await
}