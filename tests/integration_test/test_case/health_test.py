import requests


def test_health(host_name):
    response = requests.get(f"{host_name}/health_check")
    assert response.status_code == 200
