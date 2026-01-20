use sqlx::{Pool, Postgres};

pub async fn get_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        // ⚠️ SOLO PARA DEBUG
        "postgres://9bd497365146acc75bc0d9a699bb4ca56c7751229ab6121f63a0acc6585cdf72:sk_2CvELq6DOK-_LHhPkTtH2@db.prisma.io:5432/postgres?sslmode=require".to_string()
    });

    println!("DB URL usada: {}", db_url.starts_with("postgres://"));

    Pool::<Postgres>::connect(&db_url).await
}

