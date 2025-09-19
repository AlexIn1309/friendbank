use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String;
    pub role: String;
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub username: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
    pub balance: Decimal,
}

#[derive(Serialize, Deserialize)]
pub struct AccountantData {
    pub username: String,
    pub amount: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionData {
    pub recipient_username: String,
    pub amount: f64,
}
