use api::routes;
use tokio;
use anyhow;

mod config;
mod db;
pub mod entities;
mod error;
mod repository;
mod service;
mod api;

pub use error::Error;
pub use repository::Repository;
pub use service::Service;


#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), anyhow::Error> {
    std::env::set_var("RUST_LOG", "server=info");
    std::env::set_var("DATABASE_URL", "postgres://postgres:example@127.0.0.1:5432/server");
    env_logger::init();
    let config = config::Config::load()?;
    let pool = db::connect(&config.database_url).await?;
    db::migrate(&pool).await?;

    let s = Service::new(pool);
    
    let app = routes(s);
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
