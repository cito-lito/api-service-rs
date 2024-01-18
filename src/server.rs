
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use uuid::Uuid;

pub struct AppState {
    pub db: PgPool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trainer {
    pub id: Uuid,
    pub name: String,
    pub level: i16,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainerDto {
    pub name: String,
    pub level: u8,
}

impl Trainer {
    pub fn new(name: String, level: i16) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            level,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
    pub fn _update(&mut self, name: String, level: i16) {
        self.name = name;
        self.level = level;
        self.updated_at = chrono::Local::now().naive_local();
    }
}

#[post("/trainer")]
async fn create_trainer(
    app_state: web::Data<AppState>,
    trainer: web::Json<TrainerDto>,
) -> impl Responder {
    let trainer_dto = trainer.into_inner();
    let trainer = Trainer::new(trainer_dto.name, trainer_dto.level as i16);
    let result = sqlx::query_as!(
        Trainer, "insert into trainers (id, name, level, created_at, updated_at) values ($1, $2, $3, $4, $5) returning id, name, level, created_at, updated_at",
        trainer.id, trainer.name, trainer.level, trainer.created_at, trainer.updated_at
    )
    .fetch_one(&app_state.db)
    .await;

    match result {
        Ok(trainer) => HttpResponse::Ok().json(trainer),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
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

        let pool = PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set: export DATABASE_URL=..."))
            .await
            .expect("Failed to create pool.");

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(AppState { db: pool.clone() }))
                .service(index)
                .service(create_trainer)
        })
        .bind(format!("{}:{}", self.host, self.port))?
        .run()
        .await
    }
}
