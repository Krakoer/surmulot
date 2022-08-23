use axum::Extension;

use crate::Service;

pub async fn get_agents(Extension(service): Extension<Service>) -> String{
    match service.get_agents().await{
        Ok(agents) => format!("{:#?}", agents),
        Err(e) => format!("Error getting agents: {}", e)
    }
}

pub async fn post_agents(Extension(service): Extension<Service>) -> String{
    match service.register_agent().await{
        Ok(id) => id.to_string(),
        Err(e) => format!("Error creating agent: {}", e)
    }
}