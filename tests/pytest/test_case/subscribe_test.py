import pytest
import requests


def test_subscribe_returns_a_200_for_valid_form_data(host_name):
    response = requests.post(
        f"{host_name}/subscribe",
        headers={"Content-Type": "application/x-www-form-urlencoded"},
        data={"name": "Marvin Hsu", "email": "N26064074@gs.ncku.edu.tw"},
    )
    assert response.status_code == 200


@pytest.mark.parametrize(
    "test_case",
    [
        ({"name": "Marvin Hsu"}, "missing the email"),
        ({}, "missing both name the email"),
        ({"email": "N26064074@gs.ncku.edu.tw"}, "missing the name"),
    ],
)
def test_subscribe_returns_a_400_when_data_is_missing(host_name, test_case):
    response = requests.post(
        f"{host_name}/subscribe",
        headers={"Content-Type": "application/x-www-form-urlencoded"},
        data=test_case[0],
    )
    assert response.status_code == 400
    # assert test_case[1] in response.text
