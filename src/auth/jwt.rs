
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration, UNIX_EPOCH};

const SECRET: &[u8] = b"super-secret-key"; // TODO: env en prod

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,       // user_id
    pub role_id: i32,  // role_id
    pub exp: usize,
}

pub fn generate_jwt(
    user_id: i32,
    role_id: i32,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(3600)) // 1 hora
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: user_id,
        role_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    Ok(
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(SECRET),
            &Validation::default(),
        )?
        .claims
    )
}

