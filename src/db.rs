use sqlx::{Pool, Postgres};

pub async fn get_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL no configurada en el entorno");
    Pool::<Postgres>::connect(&db_url).await
}
