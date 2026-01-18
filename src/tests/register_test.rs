use friendbank::db;
use friendbank::services::user_service;
use friendbank::models::RegisterRequest;

#[tokio::test]
async fn register_creates_user_and_account() {
    let pool = db::get_pool().await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    let req = RegisterRequest {
        username: "test_user_123".into(),
        password: "password123".into(),
    };
    let result = user_service::register_with_tx(&mut tx, &req).await;
    assert!(result.is_ok());
    tx.rollback().await.unwrap();
}
