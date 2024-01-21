use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};

use sqlx::postgres::PgPool;

use crate::controllers::create_trainer::create_trainer;

pub struct AppState {
    pub db: PgPool,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
        println!("Starting server at http://{}:{}", self.host, self.port);

        let pool = PgPool::connect(
            &std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set: export DATABASE_URL=..."),
        )
        .await
        .expect("Failed to create pool.");

        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(web::Data::new(AppState { db: pool.clone() }))
                .service(index)
                .service(create_trainer)
        })
        .bind(format!("{}:{}", self.host, self.port))?
        .run()
        .await
    }
}
