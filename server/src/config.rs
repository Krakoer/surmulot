use crate::error;
use dotenv;

#[derive(Debug, Clone)]
pub struct Config{
    pub port:u16,
    pub database_url: String,
}

const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_PORT: &str = "PORT";
const DEFAULT_PORT:u16 = 3000;


impl Config{
    pub fn load() -> Result<Config, error::Error>{
        dotenv::dotenv().ok();

        let port = std::env::var(ENV_PORT)
            .ok()
            .map_or(Ok(DEFAULT_PORT), |p| p.parse::<u16>())?;

        let database_url = std::env::var(ENV_DATABASE_URL)
            .map_err(|_| error::Error::Internal(format!("Config: env {} not found", ENV_DATABASE_URL)))?;
        Ok(Config{port, database_url})
    }
}
