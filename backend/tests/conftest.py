import pytest
from httpx import AsyncClient
from fastapi.testclient import TestClient
from app.main import app

@pytest.fixture(scope="module")
async def async_client():
    async with AsyncClient(app=app, base_url="http://test") as ac:
        yield ac
