import requests

url = 'http://localhost:8000/stats'
number = 42

response = requests.post(url, json={'number': number})
for key, value in response.json().items():
    print(f'{key}: {value}')
