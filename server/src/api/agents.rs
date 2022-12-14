use axum::Extension;
use axum::response::Json;

use crate::{Service, MyError};
use crate::entities::{Agent, AgentRegister};

pub async fn get_agents(Extension(service): Extension<Service>) -> Result<Json<Vec<Agent>>, MyError>{
    match service.get_agents().await{
        Ok(agents) => Ok(Json(agents)),
        Err(err) => Err(err)
    }
}                        

pub async fn post_agents(Extension(service): Extension<Service>,  Json(payload): Json<AgentRegister>) -> String{
    match service.register_agent(payload).await{
        Ok(id) => id.to_string(),
        Err(e) => format!("Error creating agent: {}", e)
    }
}