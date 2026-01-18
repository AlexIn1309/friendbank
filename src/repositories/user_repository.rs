use sqlx::{Postgres, Transaction};
use crate::models::entities::user_entity::UserEntity;
use crate::errors::user_error::UserError;

pub async fn find_by_username(
    tx: &mut Transaction<'_, Postgres>,
    username: &str,
) -> Result<Option<UserEntity>, UserError> {
    sqlx::query_as!(
        UserEntity,
        r#"
        SELECT id, username, password_hash, role_id, created_at
        FROM "User"
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(|_| UserError::DatabaseError)
}



pub async fn insert_user(
    tx: &mut Transaction<'_, Postgres>,
    username: &str,
    password_hash: &str,
    role_id: i32,
) -> Result<i32, UserError> {
    let record = sqlx::query!(
        r#"
        INSERT INTO "User" (username, password_hash, role_id)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        username,
        password_hash,
        role_id
    )
    .fetch_one(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("? DB ERROR insert_user: {:?}", e);
        UserError::DatabaseError
    })?;

    Ok(record.id)
}


pub async fn find_by_id(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i32,
) -> Result<Option<UserEntity>, UserError> {
    sqlx::query_as!(
        UserEntity,
        r#"
        SELECT id, username, password_hash, role_id, created_at
        FROM "User"
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(|_| UserError::DatabaseError)
}
