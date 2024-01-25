mod controllers;
mod models;
mod routes;
mod server;

use server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_env();
    env_logger::init();

    // start http server
    let server = Server::new("127.0.0.1".to_string(), 3003);
    server.run().await
}

fn init_env() {
    std::env::set_var(
        "DATABASE_URL",
        "postgres://postgres:postgres@localhost:5432/app_db",
    );
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
}
