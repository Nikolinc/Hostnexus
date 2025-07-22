import pytest

@pytest.mark.asyncio
async def test_create_user(async_client):
    response = await async_client.post("/api/users/", json={
        "email": "test@example.com",
        "password": "test123"
    })
    assert response.status_code == 200
    data = response.json()
    assert data["email"] == "test@example.com"
    assert "id" in data
