use chrono::Utc;
use uuid::Uuid;

use crate::{Service, Error, entities::Job};

impl Service{
    pub async fn create_job(&self, agent_id: &Uuid, command: String) -> Result<Job, Error>{
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

    pub async fn list_jobs(&self) -> Result<Vec<Job>, Error>{
        self.repo.get_all_jobs(&self.db).await
    }
}