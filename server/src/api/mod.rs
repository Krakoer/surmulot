mod agents;
mod jobs;

use axum::{Router, routing::{get, post}, Extension};

use crate::Service;

pub fn routes(s: Service) -> Router{
    let router = Router::new().route("/", get(|| async { "Hello, World!" }))
        .route(
            "/agents", 
            get(agents::get_agents)
            .post(agents::post_agents)
        )
        .route(
            "/jobs",
            post(jobs::post_jobs)
            .get(jobs::get_all_jobs)
        )
        .route(
            "/jobs/:agent_id",
            get(jobs::get_jobs)
        )
        .layer(Extension(s));

    router
}