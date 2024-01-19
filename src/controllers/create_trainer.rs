use actix_web::{post, web, HttpResponse, Responder};
use sqlx::postgres::PgPool;

use crate::server::AppState;
use crate::models::trainer::{Trainer, TrainerDto};

#[post("/trainer")]
pub async fn create_trainer(
    app_state: web::Data<AppState>,
    trainer: web::Json<TrainerDto>,
) -> impl Responder {
    let trainer_dto = trainer.into_inner();

    // postgres smallint is i16, save cast u8 to i16
    let trainer = Trainer::new(trainer_dto.name, trainer_dto.level as i16);

    let query_result = save_trainer(&app_state.db, &trainer).await;

    // for name unique constraint: reactive approach:
    // try to save trainer, if error is unique constraint violation, return bad request
    // another approach would be to check if trainer name exists before saving
    match query_result {
        Ok(trainer) => HttpResponse::Created().json(trainer),
        Err(e) => {
            if let Some(db_error) = e.as_database_error() {
                if let Some(constraint) = db_error.constraint() {
                    if constraint == "trainers_name_key" {
                        return HttpResponse::BadRequest().body("Trainer name already exists");
                    }
                }
            }
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn save_trainer(pool: &PgPool, trainer: &Trainer) -> Result<Trainer, sqlx::Error> {
    let result = sqlx::query_as!(
        Trainer, "insert into trainers (id, name, level, created_at, updated_at) values ($1, $2, $3, $4, $5) returning id, name, level, created_at, updated_at",
        trainer.id, trainer.name, trainer.level, trainer.created_at, trainer.updated_at
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(trainer) => Ok(trainer),
        Err(e) => Err(e),
    }
}