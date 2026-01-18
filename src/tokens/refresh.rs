use uuid::Uuid;
use crate::errors::auth_error::AuthError;

pub fn generate_refresh_token() -> Result<String, AuthError> {
    Ok(Uuid::new_v4().to_string())
}
