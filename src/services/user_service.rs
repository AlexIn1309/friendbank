use sqlx::{Postgres, Transaction};

use crate::db;
use crate::errors::user_error::UserError;

use crate::models::dto::auth_dto::RegisterRequest;
use crate::models::dto::user_response::UserResponse;

use crate::repositories::{
    user_repository,
    account_repository,
};

pub async fn register(
    req: &RegisterRequest,
) -> Result<UserResponse, UserError> {
    let pool = db::get_pool().await.map_err(|_| UserError::DatabaseError)?;
    let mut tx = pool.begin().await.map_err(|_| UserError::DatabaseError)?;

    if user_repository::find_by_username(&mut tx, &req.username)
        .await?
        .is_some()
    {
        tx.rollback().await.ok();
        return Err(UserError::UserAlreadyExists);
    }

    let hashed = bcrypt::hash(&req.password, 10)
        .map_err(|_| UserError::InternalError)?;

    let user_id = user_repository::insert_user(
        &mut tx,
        &req.username,
        &hashed,
        2,
    )
    .await?;

    account_repository::create_account(&mut tx, user_id).await?;

    tx.commit().await.map_err(|_| UserError::DatabaseError)?;

    Ok(UserResponse {
        id: user_id,
        username: req.username.clone(),
        role_id: 2,
    })
}

pub async fn register_with_tx(
    tx: &mut Transaction<'_, Postgres>,
    req: &RegisterRequest,
) -> Result<UserResponse, UserError> {
    if user_repository::find_by_username(tx, &req.username)
        .await?
        .is_some()
    {
        return Err(UserError::UserAlreadyExists);
    }

    let hashed = bcrypt::hash(&req.password, 10)
        .map_err(|_| UserError::InternalError)?;

    let user_id = user_repository::insert_user(
        tx,
        &req.username,
        &hashed,
        2,
    )
    .await?;

    account_repository::create_account(tx, user_id).await?;

    
    Ok(UserResponse {
        id: user_id,
        username: req.username.clone(),
        role_id: 2,
    })

}

