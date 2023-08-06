import threading
import time

import pytest
import requests
import random
import string
import os
from sqlalchemy import text, create_engine

from utility.email_mock_server import app


def test_subscribe_returns_a_200_for_valid_form_data(host_name, email_server):
    username = generate_username(8)

    response = requests.post(
        f"{host_name}/subscriptions",
        headers={"Content-Type": "application/x-www-form-urlencoded"},
        data={"name": username, "email": f"{username}@gmail.com"},
    )
    assert response.status_code == 200

    connection_string = os.environ.get('DATABASE_URL')

    engine = create_engine(connection_string)
    with engine.connect() as conn:
        result = conn.execute(text("SELECT * FROM subscriptions WHERE email = :email"),
                              {"email": f"{username}@gmail.com"})
        assert result.fetchone() is not None

    engine.dispose()


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
        f"{host_name}/subscriptions",
        headers={"Content-Type": "application/x-www-form-urlencoded"},
        data=test_case[0],
    )
    assert response.status_code == 400, test_case[1]


def generate_username(length):
    letters = string.ascii_lowercase
    return "".join(random.choice(letters) for i in range(length))


def start_email_server():
    app.run()


@pytest.fixture(scope="module")
def email_server():
    stop_event = threading.Event()
    server_thread = threading.Thread(target=start_email_server, args=(stop_event,))
    server_thread.start()
    time.sleep(1)
    response = requests.get('http://localhost:5000')

    if response.status_code != 200:
        pytest.fail("mock_email_start_fail")

    yield

    stop_event.set()
    server_thread.join()
