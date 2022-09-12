use sqlx::{Pool, Postgres};
use log::error;
use uuid::Uuid;

use crate::{Repository, entities::Job, Error};

impl Repository{
    pub async fn add_job(&self, db: &Pool<Postgres>, job: &Job) -> Result<(), Error>{
        const QUERY: &str = "INSERT INTO jobs
            (id, created_at, executed_at, command, args, output, agent_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)";

        match sqlx::query(QUERY)
            .bind(job.id)
            .bind(job.created_at)
            .bind(job.executed_at)
            .bind(&job.command)
            .bind(&job.args)
            .bind(&job.output)
            .bind(job.agent_id)
            .execute(db)
            .await{
                Ok(_) => Ok(()),
                Err(err) => {
                    error!("add_jobs: Inserting job: {}", &err);
                    Err(err.into())
                }
            }
    }

    pub async fn get_all_jobs(&self, db: &Pool<Postgres>) -> Result<Vec<Job>, Error>{
        const QUERY: &str = "SELECT * FROM jobs ORDER BY created_at";
        match sqlx::query_as::<_, Job>(QUERY)
            .fetch_all(db)
            .await{
                Ok(res) => Ok(res),
                Err(err) => {
                    error!("get_all_jobs: {}", err);
                    Err(err.into())
                }
            }
    }

    pub async fn get_jobs(&self, db: &Pool<Postgres>, agent_id: Uuid) -> Result<Vec<Job>, Error>{
        const QUERY: &str = "SELECT * FROM jobs WHERE agent_id = $1 ORDER BY created_at";
        match sqlx::query_as::<_, Job>(QUERY)
            .bind(agent_id)
            .fetch_all(db)
            .await{
                Ok(res) => Ok(res),
                Err(err) => {
                    error!("get_jobs: {}", err);
                    Err(err.into())
                }
            }
    }
}