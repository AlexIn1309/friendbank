use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct UserEntity {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub role_id: i32,
    pub created_at: NaiveDateTime,
}

