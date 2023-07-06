import requests


def test_health():
    response = requests.get("http://localhost:3000/health_check")
    assert response.status_code == 200