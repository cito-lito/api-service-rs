mod controllers;
mod models;
mod server;
use server::Server;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    //std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    std::env::set_var(
        "DATABASE_URL",
        "postgres://postgres:postgres@localhost:5432/app_db",
    );

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // start http server
    let server = Server::new("127.0.0.1".to_string(), 3003);
    server.run().await
}
