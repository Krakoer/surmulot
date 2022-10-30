use chrono::Utc;
use sqlx::{Pool, Postgres};
use log::error;
use uuid::Uuid;

use crate::{Repository, entities::Job, MyError};

impl Repository{
    pub async fn add_job(&self, db: &Pool<Postgres>, job: &Job) -> Result<(), MyError>{
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

    pub async fn get_all_jobs(&self, db: &Pool<Postgres>) -> Result<Vec<Job>, MyError>{
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

    pub async fn get_job(&self, db: &Pool<Postgres>, agent_id: Uuid) -> Result<Job, MyError>{
        const QUERY: &str = "SELECT * FROM jobs WHERE agent_id = $1 AND output IS NULL LIMIT 1";
        match sqlx::query_as::<_, Job>(QUERY)
            .bind(agent_id)
            .fetch_optional(db)
            .await{
                Ok(Some(res)) => Ok(res),
                Ok(None) => Err(MyError::NotFound(format!("No jobs found for agent {}", agent_id))),
                Err(err) => {
                    error!("get_job: {}", err);
                    Err(err.into())
                }
            }
    }

    pub async fn post_result(&self, db: &Pool<Postgres>, job_id: Uuid, output: String) -> Result<(), MyError>{
        const QUERY: &str = "UPDATE jobs SET output = $1, executed_at = $2 WHERE id = $3";
        match sqlx::query(QUERY)
            .bind(output)
            .bind(Utc::now())
            .bind(job_id)
            .execute(db)
            .await{
                Ok(_) => Ok(()),
                Err(err) => {
                    error!("post_results: {}", &err);
                    Err(err.into())
                }
            }
    }
}