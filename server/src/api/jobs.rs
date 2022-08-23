use serde::Deserialize;
use uuid::Uuid;
use axum::{Extension, extract::Json};

use crate::Service;

#[derive(Deserialize, Debug)]
pub struct CreateJob{
    pub agent_id: String,
    pub command: String
}


pub async fn post_jobs(service: Extension<Service>, Json(payload): Json<CreateJob>) -> String{
    let agent_id = match Uuid::parse_str(&payload.agent_id){
        Ok(id) => id,
        Err(e) => {
            return e.to_string()
        }
    };
    let job = match service.create_job(&agent_id, payload.command).await{
        Ok(job) => job,
        Err(r) => {
            return r.to_string()
        }
    };

    job.id.to_string()
}