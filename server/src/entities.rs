use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow)]
pub struct Agent{
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct Job{
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
    pub command: String,
    pub args: Vec<String>,
    pub output: Option<String>,
    pub agent_id: Uuid,
}