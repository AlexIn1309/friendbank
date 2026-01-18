use jsonwebtoken::{encode, Header, EncodingKey};
use crate::errors::user_error::UserError;

pub fn generate_access_token(username: &str, role: &str,
    ) -> Result<String, UserError> {
    let claims = crate::models::JwtClaims {
        sub: username.to_string(),
        role: role.to_string(),
        exp: chrono::Utc::now()
            .checked_add_signed(chrono::Duration::minutes(15))
            .unwrap()
            .timestamp() as usize,
    };

    encode (
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"secret"),
        )
        .map_error(|_| UserError::InternalError)
}
