# Server

Needs postgres server (use docker ;))

## Endpoints

- /agents               GET     --> Vec<Agent>
- /agents               POST    --> uuid
- /jobs                 POST    agent_id, command --> id
- /jobs                 GET     --> Vec<Jobs>
- /jobs/:agent_id       GET     --> Job
- /jobs/result/:job_id  POST    String

## ToDo

[ ] Long polling

## Ressources

- 