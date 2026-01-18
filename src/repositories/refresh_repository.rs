use sqlx::{Postgres, Transaction};
use uuid::Uuid;
use crate::errors::user_error::UserError;
use crate::models::entities::refresh_token_entity::RefreshTokenEntity;

pub async fn find_by_token(
    tx: &mut Transaction<'_, Postgres>,
    token: &str,
) -> Result<Option<RefreshTokenEntity>, UserError> {
    let uuid_parsed = Uuid::parse_str(token)
        .map_err(|_| UserError::NoTokenFound)?;

    sqlx::query_as!(
        RefreshTokenEntity,
        r#"
        SELECT id, user_id, expires_at, revoked, created_at
        FROM "RefreshToken"
        WHERE id = $1
        "#,
        uuid_parsed
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(|_| UserError::DatabaseError)
}

pub async fn revoke_token(
    tx: &mut Transaction<'_, Postgres>,
    token: &Uuid,
) -> Result<(), UserError> {
    sqlx::query!(
        r#"
        UPDATE "RefreshToken"
        SET revoked = true
        WHERE id = $1
        "#,
        token
    )
    .execute(&mut **tx)
    .await
    .map_err(|_| UserError::DatabaseError)?;

    Ok(())
}

pub async fn create_token(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i32,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> Result<String, UserError> {
    let new_token = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO "RefreshToken" (id, user_id, expires_at, revoked)
        VALUES ($1, $2, $3, false)
        "#,
        new_token,
        user_id,
        expires_at.naive_utc()
    )
    .execute(&mut **tx)
    .await
    .map_err(|_| UserError::DatabaseError)?;

    Ok(new_token.to_string())
}

