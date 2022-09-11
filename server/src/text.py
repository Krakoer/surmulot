import requests

res = requests.post('http://127.0.0.1:3000/agents')
agent_id = res.text
print(res.text)

res = requests.post("http://127.0.0.1:3000/jobs", json={"agent_id": agent_id, "command": "coucouc je suis une teub"})
print(res.text)
res = requests.post("http://127.0.0.1:3000/jobs", json={"agent_id": agent_id, "command": "coucouc je suis une teub"})
print(res.text)
res = requests.post("http://127.0.0.1:3000/jobs", json={"agent_id": agent_id, "command": "coucouc je suis une teub"})
print(res.text)


res = requests.get(f"http://127.0.0.1:3000/jobs/{agent_id}")
print(res.text)