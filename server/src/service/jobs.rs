use chrono::Utc;
use uuid::Uuid;

use crate::{Service, MyError, entities::Job};

impl Service{
    pub async fn create_job(&self, agent_id: &Uuid, command: String) -> Result<Job, MyError>{
        let command = command.trim();
        let mut args: Vec<String> = command.split_whitespace().map(|e| e.to_string()).collect();
        let command = args.remove(0);
        let new_job = Job{
            id: Uuid::new_v4(),
            agent_id: agent_id.clone(),
            command: command,
            args: args,
            created_at: Utc::now(),
            executed_at: None,
            output: None,
        };

        self.repo.add_job(&self.db, &new_job).await?;

        Ok(new_job)
    }

    pub async fn list_all_jobs(&self) -> Result<Vec<Job>, MyError>{
        self.repo.get_all_jobs(&self.db).await
    }

    pub async fn list_jobs(&self, agent_id: Uuid) -> Result<Job, MyError>{
        self.repo.get_job(&self.db, agent_id).await
    }

    pub async fn post_result(&self, job_id: Uuid, output: String) -> Result<(), MyError>{
        self.repo.post_result(&self.db, job_id, output).await
    }
}