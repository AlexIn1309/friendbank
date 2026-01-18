#[derive(Debug)]
pub enum AuthError {
    UserNotFound,
    InvalidCredentials,
    DatabaseError,
    InternalError,
}
