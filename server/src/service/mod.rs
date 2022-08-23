use sqlx::{Pool, Postgres};

use crate::{Repository};

mod agents;
mod jobs;

#[derive(Debug, Clone)]
pub struct Service{
    repo: Repository,
    db: Pool<Postgres>
}

impl Service{
    pub fn new(db: Pool<Postgres>) -> Service{
        let repo = Repository{};
        Service{
            repo,
            db
        }
    }
}