use serde::{Deserialize, Serialize};

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

