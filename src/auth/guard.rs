use crate::errors::user_error::UserError;
use crate::auth::jwt::decode_jwt;
use crate::auth::context::AuthContext;

pub fn authenticate_bearer(
    auth_header: Option<&str>,
) -> Result<AuthContext, UserError> {
    let header = auth_header.ok_or(UserError::Unauthorized)?;

    if !header.starts_with("Bearer ") {
        return Err(UserError::Unauthorized);
    }

    let token = header.trim_start_matches("Bearer ");

    let claims = decode_jwt(token)
        .map_err(|_| UserError::InvalidToken)?;

    Ok(AuthContext {
        user_id: claims.sub,
        role_id: claims.role_id,
    })
}

pub fn require_accountant(ctx: &AuthContext) -> Result<(), UserError> {
    if ctx.role_id != 1 {
        return Err(UserError::Forbidden);
    }
    Ok(())
}

