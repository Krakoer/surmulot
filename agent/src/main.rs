use ureq;
use std::{time::Duration, str::FromStr};
use uuid::Uuid;
use std::thread;

fn main() {
    let agent: ureq::Agent = ureq::AgentBuilder::new()
      .timeout_read(Duration::from_secs(5))
      .timeout_write(Duration::from_secs(5))
      .build();

    let id = agent.post("http://127.0.0.1:3000/agents").call().unwrap().into_string().unwrap();
    let id = Uuid::from_str(&id).unwrap();

    // Long pooling
    loop{
        // Try to get a job 
        let jobs = agent.get(format!("http://127.0.0.1:3000/jobs/{}", id).as_str()).call().unwrap().into_string().unwrap();
        println!("Jobs: {}", jobs);
        thread::sleep(Duration::from_secs(5));
    }
}
