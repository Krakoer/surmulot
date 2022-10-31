use chrono::Utc;
use uuid::Uuid;

use crate::{Service, entities::{Agent, AgentRegister}, MyError};

impl Service{
    pub async fn register_agent(&self, new_agent: AgentRegister) -> Result<Uuid, MyError>{
        let now = Utc::now();
        let id = Uuid::new_v4();
        let agent = Agent{
            id,
            last_seen_at: now,
            created_at: now,
            username: new_agent.username,
            hostname: new_agent.hostname,
        };

        self.repo.create_agent(&self.db, &agent).await?;

        Ok(id)
    }

    pub async fn get_agents(&self) -> Result<Vec<Agent>, MyError>{
        self.repo.get_agents(&self.db).await
    }
}