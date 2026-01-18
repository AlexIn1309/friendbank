use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("El usuario ya existe")]
    UserAlreadyExists,

    #[error("Usuario no encontrado")]
    UserNotFound,

    #[error("Contrasena Incorrecta")]
    InvalidPassword,

    #[error("Error de base de datos")]
    DatabaseError,

    #[error("Error Interno")]
    InternalError,

    #[error("No existe ese token refresh")]
    NoTokenFound,

    #[error("Este token es invalido")]
    InvalidToken,

    #[error("Error al generar el token")]
    TokenGenerationError,

    #[error("Credenciales no validas")]
    InvalidCredentials,

    #[error("No tienes acceso a este recuros")]
    Forbidden,

    #[error("No tienes acceso a este recuros")]
    Unauthorized,
}
