use sqlx::{Postgres, Transaction};
use crate::errors::user_error::UserError;
use rust_decimal::Decimal;

pub async fn create_account(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i32,
) -> Result<(), UserError> {
    sqlx::query!(
        r#"
        INSERT INTO "Account" (user_id, balance)
        VALUES ($1, $2)
        "#,
        user_id,
        Decimal::ZERO
    )
    .execute(&mut **tx)
    .await
    .map_err(|_| UserError::DatabaseError)?;

    Ok(())
}

