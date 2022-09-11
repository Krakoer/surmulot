use std::str::FromStr;

use serde::Deserialize;
use uuid::Uuid;
use axum::{Extension, extract::{Json, Path}};

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

pub async fn get_all_jobs(service: Extension<Service>) -> String{
    match service.list_all_jobs().await{
        Ok(jobs) => format!("{:#?}", jobs),
        Err(e) => format!("Error getting jobs: {}", e)
    }
}

pub async fn get_jobs(service: Extension<Service>, Path(agent_id): Path<String>) -> String{
    let agent_id = match Uuid::from_str(&agent_id){
        Ok(a) => a,
        Err(e) => {return format!("Error getting jobs: {}", e)}
    };
    
    match service.list_jobs(agent_id).await{
        Ok(jobs) => format!("{:#?}", jobs),
        Err(e) => format!("Error getting jobs: {}", e)
    }
}