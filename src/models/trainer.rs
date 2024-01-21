use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct TrainerDto {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(range(min = 1, max = 50))]
    pub level: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trainer {
    pub id: Uuid,
    pub name: String,
    pub level: i16,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
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
}
