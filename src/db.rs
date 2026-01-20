use sqlx::{Pool, Postgres};

pub async fn get_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    match std::env::var("DATABASE_URL") {
        Ok(db_url) => {
            println!("DATABASE_URL presente: true");
            Pool::<Postgres>::connect(&db_url).await
        }
        Err(e) => {
            println!("DATABASE_URL presente: false -> {:?}", e);
            Err(sqlx::Error::Configuration(Box::new(e)))
        }
    }
}

