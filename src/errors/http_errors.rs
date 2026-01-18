
use vercel_runtime::StatusCode;
use crate::errors::user_error::UserError;

pub fn map_auth_error(err: &UserError) -> StatusCode {
    match err {
        UserError::InvalidToken => StatusCode::UNAUTHORIZED,
        UserError::NoTokenFound => StatusCode::UNAUTHORIZED,
        UserError::UserNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

