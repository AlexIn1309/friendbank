use chrono::Utc;
use crate::db;
use crate::auth;

use crate::errors::user_error::UserError;

use crate::models::dto::auth_dto::{
    LoginRequest,
    LoginResponse,
    RefreshTokenRequest,
    RefreshTokenResponse,
};

use crate::repositories::{
    refresh_repository,
    user_repository,
};

use bcrypt;

pub async fn refresh(
    req: RefreshTokenRequest,
) -> Result<RefreshTokenResponse, UserError> {
    let pool = db::get_pool().await.map_err(|_| UserError::DatabaseError)?;
    let mut tx = pool.begin().await.map_err(|_| UserError::DatabaseError)?;

    let token_entity = refresh_repository::find_by_token(&mut tx, &req.refresh_token)
        .await?
        .ok_or(UserError::NoTokenFound)?;

    if token_entity.is_invalid() {
        return Err(UserError::InvalidToken);
    }

    let user = user_repository::find_by_id(&mut tx, token_entity.user_id)
        .await?
        .ok_or(UserError::UserNotFound)?;

    refresh_repository::revoke_token(&mut tx, &token_entity.id).await?;

    let expires_at = Utc::now() + chrono::Duration::days(7);
    let new_refresh_token =
        refresh_repository::create_token(&mut tx, token_entity.user_id, expires_at).await?;

    tx.commit().await.map_err(|_| UserError::DatabaseError)?;

    let new_access_token =
        auth::generate_jwt(user.id, user.role_id)
            .map_err(|_| UserError::TokenGenerationError)?;

    Ok(RefreshTokenResponse {
        token: new_access_token,
        refresh_token: new_refresh_token.to_string(),
    })
}



pub async fn login(req: LoginRequest) -> Result<LoginResponse, UserError> {
    let pool = db::get_pool().await.map_err(|_| UserError::DatabaseError)?;
    let mut tx = pool.begin().await.map_err(|_| UserError::DatabaseError)?;

    // 1. Buscar usuario
    let user = user_repository::find_by_username(&mut tx, &req.username)
        .await?
        .ok_or(UserError::UserNotFound)?;

    // 2. Verificar password
    let is_valid = bcrypt::verify(&req.password, &user.password_hash)
        .map_err(|_| UserError::InvalidCredentials)?;

    if !is_valid {
        return Err(UserError::InvalidCredentials);
    }

    // 3. Generar JWT (access token)
    let token = auth::generate_jwt(user.id, user.role_id)
    .map_err(|_| UserError::TokenGenerationError)?;

    // 4. Crear refresh token (7 dias)
    let expires_at = chrono::Utc::now() + chrono::Duration::days(7);

    let refresh_token = refresh_repository::create_token(
        &mut tx,
        user.id,
        expires_at,
    )
    .await?;

    // 5. Commit de la transaccion
    tx.commit().await.map_err(|_| UserError::DatabaseError)?;

    // 6. R
Ok(LoginResponse {
        token,
        refresh_token,
        role: user.role_id.to_string(),
    })
}
