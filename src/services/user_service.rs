use sqlx::Row;
use crate::models::{UserData, User, LoginResponse};
use crate::db;
use crate::auth::{verify_password, generate_jwt};

pub async fn login(user_data: &UserData) -> Result<LoginResponse, String> {
    let pool = db::get_pool().await.map_err(|_| "DB error")?;

    let row = sqlx::query("SELECT id, username, password_hash, role FROM \"User\" WHERE username = $1")
        .bind(&user_data.username)
        .fetch_one(&pool)
        .await
        .map_err(|_| "Usuario no encontrado")?;

    let user = User {
        id: row.get("id"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        role: row.get("role"),
    };

    if !verify_password(&user_data.password, &user.password_hash) {
        return Err("Contraseña incorrecta".into());
    }

    let token = generate_jwt(&user.username, &user.role).map_err(|_| "Error al generar JWT")?;

    Ok(LoginResponse { token, role: user.role })
}

