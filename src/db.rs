use sqlx::{Pool, Postgres};

pub async fn get_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        // ⚠️ SOLO PARA DEBUG
        "postgres://USER:PASSWORD@HOST:5432/DBNAME?sslmode=require".to_string()
    });

    println!("DB URL usada: {}", db_url.starts_with("postgres://"));

    Pool::<Postgres>::connect(&db_url).await
}

