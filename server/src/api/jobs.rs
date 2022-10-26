use std::str::FromStr;

use serde::Deserialize;
use uuid::Uuid;
use axum::{Extension, extract::{Json, Path}};

use crate::{Service, MyError, entities::Job};

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

pub async fn get_all_jobs(service: Extension<Service>) -> Result<Json<Vec<Job>>, MyError>{
    match service.list_all_jobs().await{
        Ok(jobs) => Ok(Json(jobs)),
        Err(e) => Err(e)
    }
}

pub async fn get_jobs(service: Extension<Service>, Path(agent_id): Path<String>) -> Result<Json<Job>, MyError>{
    let agent_id = match Uuid::from_str(&agent_id){
        Ok(a) => a,
        Err(e) => {return Err(e.into())}
    };
    
    match service.list_jobs(agent_id).await{
        Ok(job) => Ok(Json(job)),
        Err(e) => Err(e)
    }
}