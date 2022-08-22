use chrono::Utc;
use uuid::Uuid;

use crate::{Service, entities::Agent, Error};

impl Service{
    pub async fn register_agent(&self) -> Result<Uuid, Error>{
        let now = Utc::now();
        let id = Uuid::new_v4();
        let agent = Agent{
            id,
            last_seen_at: now,
            created_at: now,
        };

        self.repo.create_agent(&self.db, &agent).await?;

        Ok(id)
    }

    pub async fn get_agents(&self) -> Result<Vec<Agent>, Error>{
        self.repo.get_agents(&self.db).await
    }
}