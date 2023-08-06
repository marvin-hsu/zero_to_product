import pytest


@pytest.fixture(autouse=True)
def host_name():
    return "http://localhost:3000"
