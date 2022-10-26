use log::error;
use crate::error;
use sqlx::{self, postgres::PgPoolOptions, Pool, Postgres};

pub async fn connect(db_url: &str) -> Result<Pool<Postgres>, error::MyError>{
    PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .map_err(|err| {
            error!("Cannot connect to db {}: {}", db_url, err);
            err.into()
        })
}

pub async fn migrate(db: &Pool<Postgres>) -> Result<(), error::MyError> {
    match sqlx::migrate!("./db/migrations").run(db).await {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("db::migrate: migrating: {}", &err);
            Err(err)
        }
    }?;
    Ok(())
}