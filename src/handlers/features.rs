use std::path::Path;
use std::fs;
use std::io::Result;

pub struct Features<'a> {
    root: &'a Path,
    app: &'a Path,
}

impl<'a> Features<'a> {
    pub fn new(root: &'a Path, app: &'a Path) -> Self {
        Features { root, app }
    }

    pub fn setup_database(&self) -> Result<()> {
        fs::write(
            self.app.join("core").join("database.py"),
            r#"from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker
from dotenv import load_dotenv
import os

load_dotenv()

SQLALCHEMY_DATABASE_URL = os.getenv("DATABASE_URL", "postgresql://user:password@localhost/dbname")

engine = create_engine(SQLALCHEMY_DATABASE_URL)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

Base = declarative_base()

def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()
"#,
        )?;

        fs::write(
            "alembic.ini",
            r#"[alembic]
script_location = migrations
sqlalchemy.url = driver://user:pass@localhost/dbname

[loggers]
keys = root,sqlalchemy,alembic

[handlers]
keys = console

[formatters]
keys = generic

[logger_root]
level = WARN
handlers = console
qualname =

[logger_sqlalchemy]
level = WARN
handlers =
qualname = sqlalchemy.engine

[logger_alembic]
level = INFO
handlers =
qualname = alembic

[handler_console]
class = StreamHandler
args = (sys.stderr,)
level = NOTSET
formatter = generic

[formatter_generic]
format = %(levelname)-5.5s [%(name)s] %(message)s
datefmt = %H:%M:%S
"#,
        )?;

        Ok(())
    }

    pub fn setup_auth(&self) -> Result<()> {
        let auth_dir = self.app.join("core").join("auth");
        fs::create_dir_all(&auth_dir)?;
        
        fs::write(
            auth_dir.join("jwt.py"),
            r#"from datetime import datetime, timedelta
from typing import Optional
from jose import JWTError, jwt
from passlib.context import CryptContext
from pydantic import BaseModel

SECRET_KEY = "your-secret-key-here"  # Cambiar en producción
ALGORITHM = "HS256"
ACCESS_TOKEN_EXPIRE_MINUTES = 30

pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")

class Token(BaseModel):
    access_token: str
    token_type: str

class TokenData(BaseModel):
    username: Optional[str] = None

def verify_password(plain_password: str, hashed_password: str) -> bool:
    return pwd_context.verify(plain_password, hashed_password)

def get_password_hash(password: str) -> str:
    return pwd_context.hash(password)

def create_access_token(data: dict, expires_delta: Optional[timedelta] = None) -> str:
    to_encode = data.copy()
    if expires_delta:
        expire = datetime.utcnow() + expires_delta
    else:
        expire = datetime.utcnow() + timedelta(minutes=15)
    to_encode.update({"exp": expire})
    encoded_jwt = jwt.encode(to_encode, SECRET_KEY, algorithm=ALGORITHM)
    return encoded_jwt

def verify_token(token: str, credentials_exception) -> TokenData:
    try:
        payload = jwt.decode(token, SECRET_KEY, algorithms=[ALGORITHM])
        username: str = payload.get("sub")
        if username is None:
            raise credentials_exception
        token_data = TokenData(username=username)
        return token_data
    except JWTError:
        raise credentials_exception"#,
        )?;

        fs::write(
            auth_dir.join("deps.py"),
            r#"from fastapi import Depends, HTTPException, status
from fastapi.security import OAuth2PasswordBearer
from .jwt import verify_token

oauth2_scheme = OAuth2PasswordBearer(tokenUrl="token")

async def get_current_user(token: str = Depends(oauth2_scheme)):
    credentials_exception = HTTPException(
        status_code=status.HTTP_401_UNAUTHORIZED,
        detail="Could not validate credentials",
        headers={"WWW-Authenticate": "Bearer"},
    )
    return verify_token(token, credentials_exception)"#,
        )?;

        fs::write(
            self.app.join("routes").join("auth.py"),
            r#"from fastapi import APIRouter, Depends, HTTPException, status
from fastapi.security import OAuth2PasswordRequestForm
from datetime import timedelta
from ..core.auth.jwt import Token, create_access_token, ACCESS_TOKEN_EXPIRE_MINUTES

router = APIRouter(tags=["auth"])

@router.post("/token", response_model=Token)
async def login(form_data: OAuth2PasswordRequestForm = Depends()):
    # Aquí deberías verificar las credenciales contra tu base de datos
    # Este es solo un ejemplo
    if form_data.username != "test" or form_data.password != "test":
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Incorrect username or password",
            headers={"WWW-Authenticate": "Bearer"},
        )

    access_token_expires = timedelta(minutes=ACCESS_TOKEN_EXPIRE_MINUTES)
    access_token = create_access_token(
        data={"sub": form_data.username}, expires_delta=access_token_expires
    )
    return {"access_token": access_token, "token_type": "bearer"}"#,
        )?;

        Ok(())
    }

    pub fn setup_cors(&self) -> Result<()> {
        fs::write(
            self.app.join("core").join("cors.py"),
            r#"from fastapi.middleware.cors import CORSMiddleware

def setup_cors(app):
    app.add_middleware(
        CORSMiddleware,
        allow_origins=["*"],  # Configura esto con tus dominios permitidos en producción
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
    )"#,
        )?;

        fs::write(
            self.app.join("main.py"),
            r#"from fastapi import FastAPI
from app.routes import example_router
from app.core.cors import setup_cors

app = FastAPI()

# Configurar CORS
setup_cors(app)

app.include_router(example_router)

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)"#,
        )?;

        Ok(())
    }

    pub fn setup_cache(&self) -> Result<()> {
        fs::write(
            self.app.join("core").join("cache.py"),
            r#"from fastapi_cache import FastAPICache
from fastapi_cache.backends.redis import RedisBackend
from redis import asyncio as aioredis
from dotenv import load_dotenv
import os

load_dotenv()

REDIS_URL = os.getenv("REDIS_URL", "redis://localhost:6379")

async def setup_cache():
    redis = aioredis.from_url(REDIS_URL, encoding="utf8", decode_responses=True)
    FastAPICache.init(RedisBackend(redis), prefix="fastapi-cache")

# Ejemplo de uso del cache:
'''
from fastapi_cache.decorator import cache

@router.get("/items")
@cache(expire=60)  # Cache por 60 segundos
async def get_items():
    return {"items": ["item1", "item2"]}
'''
"#,
        )?;

        fs::write(
            self.app.join("main.py"),
            r#"from fastapi import FastAPI
from app.routes import example_router
from app.core.cache import setup_cache

app = FastAPI()

@app.on_event("startup")
async def startup_event():
    await setup_cache()

app.include_router(example_router)

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
"#,
        )?;

        fs::write(
            self.root.join(".env.example"),
            r#"# 🔐 Environment variables
REDIS_URL=redis://localhost:6379
"#,
        )?;

        Ok(())
    }

    pub fn setup_tasks(&self) -> Result<()> {
        let tasks_dir = self.app.join("core").join("tasks");
        fs::create_dir_all(&tasks_dir)?;

        fs::write(
            tasks_dir.join("celery.py"),
            r#"from celery import Celery
from dotenv import load_dotenv
import os

load_dotenv()

REDIS_URL = os.getenv("REDIS_URL", "redis://localhost:6379")

celery = Celery(
    "app",
    broker=REDIS_URL,
    backend=REDIS_URL,
    include=["app.core.tasks.tasks"]
)

# Configuración opcional de Celery
celery.conf.update(
    task_serializer="json",
    accept_content=["json"],
    result_serializer="json",
    timezone="UTC",
    enable_utc=True,
)
"#,
        )?;

        fs::write(
            tasks_dir.join("tasks.py"),
            r#"from .celery import celery
from celery import Task
from typing import Dict, Any

class BaseTask(Task):
    abstract = True

    def on_failure(self, exc, task_id, args, kwargs, einfo):
        # Manejo de errores personalizado
        print(f"Task {task_id} failed: {exc}")

@celery.task(base=BaseTask)
def proceso_largo(data: Dict[str, Any]) -> Dict[str, Any]:
    """
    Ejemplo de tarea asíncrona
    """
    # Simular proceso largo
    import time
    time.sleep(5)
    
    return {
        "status": "completed",
        "data": data,
        "message": "Proceso completado exitosamente"
    }
"#,
        )?;

        fs::write(
            self.app.join("routes").join("tasks.py"),
            r#"from fastapi import APIRouter, BackgroundTasks
from ..core.tasks.tasks import proceso_largo
from typing import Dict, Any

router = APIRouter(prefix="/tasks", tags=["tasks"])

@router.post("/proceso")
async def iniciar_proceso(data: Dict[str, Any]):
    """
    Inicia un proceso asíncrono usando Celery
    """
    task = proceso_largo.delay(data)
    return {
        "task_id": task.id,
        "message": "Proceso iniciado correctamente"
    }

@router.get("/status/{task_id}")
async def estado_proceso(task_id: str):
    """
    Obtiene el estado de una tarea
    """
    task = proceso_largo.AsyncResult(task_id)
    return {
        "task_id": task_id,
        "status": task.status,
        "result": task.result if task.ready() else None
    }
"#,
        )?;

        fs::write(
            self.root.join("flower.py"),
            r#"from app.core.tasks.celery import celery

# Configuración de Flower
flower = celery
"#,
        )?;

        fs::write(
            self.root.join(".env.example"),
            r#"# 🔐 Environment variables
REDIS_URL=redis://localhost:6379
"#,
        )?;

        Ok(())
    }

    pub fn setup_supabase(&self) -> Result<()> {
        fs::write(
            self.app.join("core").join("supabase.py"),
            r#"from supabase import create_client
from dotenv import load_dotenv
import os

load_dotenv()

SUPABASE_URL = os.getenv("SUPABASE_URL")
SUPABASE_KEY = os.getenv("SUPABASE_KEY")

supabase = create_client(SUPABASE_URL, SUPABASE_KEY)

# Ejemplo de uso:
'''
# Consultar datos
response = supabase.table("users").select("*").execute()

# Insertar datos
data = {"name": "John Doe", "email": "john@example.com"}
response = supabase.table("users").insert(data).execute()

# Actualizar datos
response = supabase.table("users").update({"name": "Jane Doe"}).eq("id", 1).execute()

# Eliminar datos
response = supabase.table("users").delete().eq("id", 1).execute()
'''
"#,
        )?;

        // Actualizar .env.example con configuración de Supabase
        fs::write(
            self.root.join(".env.example"),
            r#"# 🔐 Environment variables
SUPABASE_URL=your-project-url
SUPABASE_KEY=your-anon-key
SUPABASE_SECRET_KEY=your-service-role-key
"#,
        )?;

        Ok(())
    }
}