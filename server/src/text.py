import requests
url_base = "http://127.0.0.1:5000"
def get_agents():
    r = requests.get(f"{url_base}/agents")
    return r.content

print(get_agents())