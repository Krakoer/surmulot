use super::Repository;
use crate::{entities::Agent, MyError};
use log::error;
use sqlx::{Pool, Postgres};

impl Repository{
    pub async fn create_agent(&self, db: &Pool<Postgres>, agent: &Agent) -> Result<(), MyError>{
        const QUERY: &str = "INSERT INTO agents
            (id, created_at, last_seen_at, username, hostname)
            VALUES ($1, $2, $3, $4, $5)";

        match sqlx::query(QUERY)
            .bind(agent.id)
            .bind(agent.created_at)
            .bind(agent.last_seen_at)
            .bind(&agent.username)
            .bind(&agent.hostname)
            .execute(db)
            .await{
                Ok(_) => Ok(()),
                Err(err) => {
                    error!("create_agent: Inserting agent: {}", &err);
                    Err(err.into())
                }
            }
    }

    pub async fn get_agents(&self, db: &Pool<Postgres>) -> Result<Vec<Agent>, MyError>{
        const QUERY: &str = "SELECT * FROM agents ORDER BY created_at";
        match sqlx::query_as::<_, Agent>(QUERY)
            .fetch_all(db)
            .await{
                Ok(res) => Ok(res),
                Err(err) => {
                    error!("find all jobs: {}", err);
                    Err(err.into())
                }
            }
    }


}