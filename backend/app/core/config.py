from pydantic import BaseSettings

class Settings(BaseSettings):
    PROJECT_NAME: str = "Hostnexus API"
    DB_HOST: str = "localhost"
    DB_PORT: int = 5432
    DB_USER: str = "postgres"
    DB_PASSWORD: str = "password"
    DB_NAME: str = "hostnexus"

    class Config:
        env_file = ".env"

settings = Settings()
