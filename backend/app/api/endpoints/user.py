from fastapi import APIRouter, Depends
from sqlalchemy.ext.asyncio import AsyncSession
from app.db.database import get_db
from app.schemas.user import UserCreate, UserOut
from app.models.user import User

router = APIRouter()

@router.post("/", response_model=UserOut)
async def create_user(user: UserCreate, db: AsyncSession = Depends(get_db)):
    new_user = User(email=user.email, hashed_password=user.password)  # Здесь надо будет хешировать пароль
    db.add(new_user)
    await db.commit()
    await db.refresh(new_user)
    return new_user

