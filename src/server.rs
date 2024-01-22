use actix_cors::Cors;
use actix_web::{middleware::{Logger, Compress}, web, App, HttpServer, Responder, get, HttpResponse};

use sqlx::postgres::PgPool;

use crate::routes::config_routes;

pub struct AppState {
    pub db: PgPool,
}

pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
    pub async fn run(&self) -> std::io::Result<()> {
        log::info!("Starting server at http://{}:{}", self.host, self.port);

        let pool = PgPool::connect(
            &std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set: export DATABASE_URL=..."),
        )
        .await
        .expect("Failed to create pool.");

        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .wrap(Cors::default())
                .wrap(Compress::default())
                .app_data(web::Data::new(AppState { db: pool.clone() }))
                .service(health_check)
                .configure(config_routes)
        })
        .bind(format!("{}:{}", self.host, self.port))?
        .run()
        .await
    }
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("Healthy")
}
