use rust_decimal::Decimal;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct AccountEntity {
    pub id: i32,
    pub user_id: i32,
    pub balance: Decimal, // Usamos Decimal para dinero, ¡nunca f64!
}
