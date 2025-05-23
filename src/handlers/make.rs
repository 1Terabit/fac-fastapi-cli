use crate::utils::fs::append_to_init;
use crate::utils::testing::*;
use std::fs;
use std::path::Path;

fn capitalize_name(name: &str) -> String {
    name.chars()
        .next()
        .unwrap()
        .to_uppercase()
        .chain(name.chars().skip(1))
        .collect::<String>()
}

fn create_route(name: &str) {
    let capitalized_name = capitalize_name(name);
    let path = Path::new("app")
        .join("interfaces")
        .join("api")
        .join("v1")
        .join("endpoints")
        .join(format!("{}.py", name));
    fs::create_dir_all("app/interfaces/api/v1/endpoints").ok();

    fs::write(
        &path,
        format!(
            r#"from fastapi import APIRouter, Depends, HTTPException
from app.application.use_cases.{0}_use_case import Get{1}UseCase, GetAll{1}UseCase, Create{1}UseCase, Update{1}UseCase, Delete{1}UseCase
from app.infrastructure.repositories.{0}_repository import {1}Repository
from app.interfaces.api.v1.schemas.{0} import {1}Create, {1}Update, {1}Response

router = APIRouter(prefix="/{0}s", tags=["{0}s"])

@router.get("/", response_model=list[{1}Response])
async def get_all_{0}s():
    repository = {1}Repository()
    use_case = GetAll{1}UseCase(repository)
    return use_case.execute()

@router.get("/{{id}}", response_model={1}Response)
async def get_{0}(id: str):
    repository = {1}Repository()
    use_case = Get{1}UseCase(repository)
    return use_case.execute(id)

@router.post("/", response_model={1}Response, status_code=201)
async def create_{0}(data: {1}Create):
    repository = {1}Repository()
    use_case = Create{1}UseCase(repository)
    return use_case.execute(data.dict())

@router.put("/{{id}}", response_model={1}Response)
async def update_{0}(id: str, data: {1}Update):
    repository = {1}Repository()
    use_case = Update{1}UseCase(repository)
    return use_case.execute(id, data.dict())

@router.delete("/{{id}}")
async def delete_{0}(id: str):
    repository = {1}Repository()
    use_case = Delete{1}UseCase(repository)
    return {{"success": use_case.execute(id)}}
"#,
            name,
            capitalized_name
        ),
    )
    .expect("❌ Error al crear el archivo de ruta");

    append_to_init("app/interfaces/api/v1/endpoints/__init__.py", name);
    println!("✅ Ruta creada en '{}'", path.display());

    let test_content = generate_route_test(name, "GET");
    create_test_file("route", name, &test_content);
}

pub fn create_model(name: &str) {
    let path = Path::new("app")
        .join("infrastructure")
        .join("database")
        .join("models")
        .join(format!("{}.py", name));
    fs::create_dir_all("app/infrastructure/database/models").ok();

    fs::write(
        &path,
        format!(
            r#"from sqlalchemy import Column, Integer, String
from app.infrastructure.database.base import Base

class {0}Model(Base):
    __tablename__ = "{1}s"

    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, index=True)
    description = Column(String)
"#,
            name.chars()
                .next()
                .unwrap()
                .to_uppercase()
                .chain(name.chars().skip(1))
                .collect::<String>(),
            name.to_lowercase()
        ),
    )
    .expect("❌ Failed to write model file");

    append_to_init("app/infrastructure/database/models/__init__.py", name);
    println!("✅ Model created at '{}'", path.display());
}

pub fn create_entity(name: &str) {
    let capitalized_name = capitalize_name(name);

    // 1. Crear entidad (Core)
    create_core_entity(name, &capitalized_name);

    // 2. Crear interfaz del repositorio (Core)
    create_repository_interface(name, &capitalized_name);

    // 3. Crear caso de uso (Application)
    create_use_case(name, &capitalized_name);

    // 4. Crear implementación del repositorio (Infrastructure)
    create_repository_impl(name, &capitalized_name);

    // 5. Crear modelo de base de datos (Infrastructure)
    create_model(name);

    // 6. Crear schema/DTO (Interface)
    create_schema(name, &capitalized_name);

    // 7. Crear endpoint (Interface)
    create_route(name);

    println!(
        "✨ Entidad '{}' creada exitosamente con todos sus componentes!",
        name
    );
}

fn create_core_entity(name: &str, capitalized_name: &str) {
    let path = Path::new("app")
        .join("core")
        .join("entities")
        .join(format!("{}.py", name));
    fs::create_dir_all("app/core/entities").ok();

    fs::write(
        &path,
        format!(
            r#"from dataclasses import dataclass
from app.core.types import ID

@dataclass(frozen=True)
class {0}:
    id: ID
    name: str
    description: str"#,
            capitalized_name
        ),
    )
    .expect("❌ Error al crear la entidad");

    append_to_init("app/core/entities/__init__.py", name);
    println!("✅ Entidad core creada en '{}'", path.display());
}

fn create_repository_interface(name: &str, capitalized_name: &str) {
    let path = Path::new("app")
        .join("core")
        .join("interfaces")
        .join(format!("{}_repository.py", name));
    fs::create_dir_all("app/core/interfaces").ok();

    fs::write(
        &path,
        format!(
            r#"from abc import ABC, abstractmethod
from ..entities.{0} import {1}

class {1}Repository(ABC):
    @abstractmethod
    def get_by_id(self, id: str) -> {1}:
        pass
        
    @abstractmethod
    def get_all(self) -> list[{1}]:
        pass
        
    @abstractmethod
    def create(self, data: dict) -> {1}:
        pass
        
    @abstractmethod
    def update(self, id: str, data: dict) -> {1}:
        pass
        
    @abstractmethod
    def delete(self, id: str) -> bool:
        pass"#,
            name, capitalized_name
        ),
    )
    .expect("❌ Error al crear la interfaz del repositorio");

    append_to_init("app/core/interfaces/__init__.py", name);
    println!("✅ Interfaz del repositorio creada en '{}'", path.display());
}

fn create_use_case(name: &str, capitalized_name: &str) {
    let path = Path::new("app")
        .join("application")
        .join("use_cases")
        .join(format!("{}_use_case.py", name));
    fs::create_dir_all("app/application/use_cases").ok();

    fs::write(
        &path,
        format!(
            r#"from app.core.interfaces.{0}_repository import {1}Repository
from app.core.entities.{0} import {1}

class Get{1}UseCase:
    def __init__(self, repository: {1}Repository):
        self.repository = repository

    def execute(self, {0}_id: str) -> {1}:
        return self.repository.get_by_id({0}_id)

class GetAll{1}UseCase:
    def __init__(self, repository: {1}Repository):
        self.repository = repository

    def execute(self) -> list[{1}]:
        return self.repository.get_all()

class Create{1}UseCase:
    def __init__(self, repository: {1}Repository):
        self.repository = repository

    def execute(self, data: dict) -> {1}:
        return self.repository.create(data)

class Update{1}UseCase:
    def __init__(self, repository: {1}Repository):
        self.repository = repository

    def execute(self, {0}_id: str, data: dict) -> {1}:
        return self.repository.update({0}_id, data)

class Delete{1}UseCase:
    def __init__(self, repository: {1}Repository):
        self.repository = repository

    def execute(self, {0}_id: str) -> bool:
        return self.repository.delete({0}_id)"#,
            name, capitalized_name
        ),
    )
    .expect("❌ Error al crear el caso de uso");

    append_to_init("app/application/use_cases/__init__.py", name);
    println!("✅ Casos de uso creados en '{}'", path.display());
}

fn create_repository_impl(name: &str, capitalized_name: &str) {
    let path = Path::new("app")
        .join("infrastructure")
        .join("repositories")
        .join(format!("{}_repository.py", name));
    fs::create_dir_all("app/infrastructure/repositories").ok();

    fs::write(
        &path,
        format!(
            r#"from app.core.interfaces.{0}_repository import {1}Repository as {1}RepositoryInterface
from app.core.entities.{0} import {1}
from app.infrastructure.database.models.{0} import {1}Model

class {1}Repository({1}RepositoryInterface):
    def __init__(self):
        self.model = {1}Model

    def get_by_id(self, id: str) -> {1}:
        model = self.model.get_by_id(id)
        return {1}(
            id=model.id,
            name=model.name,
            description=model.description
        )

    def get_all(self) -> list[{1}]:
        models = self.model.get_all()
        return [
            {1}(
                id=model.id,
                name=model.name,
                description=model.description
            )
            for model in models
        ]

    def create(self, data: dict) -> {1}:
        model = self.model.create(data)
        return {1}(
            id=model.id,
            name=model.name,
            description=model.description
        )

    def update(self, id: str, data: dict) -> {1}:
        model = self.model.update(id, data)
        return {1}(
            id=model.id,
            name=model.name,
            description=model.description
        )

    def delete(self, id: str) -> bool:
        return self.model.delete(id)"#,
            name, capitalized_name
        ),
    ).expect("❌ Error al crear la implementación del repositorio");

    append_to_init("app/infrastructure/repositories/__init__.py", name);
    println!(
        "✅ Implementación del repositorio creada en '{}'",
        path.display()
    );
}

fn create_schema(name: &str, capitalized_name: &str) {
    let path = Path::new("app")
        .join("interfaces")
        .join("api")
        .join("v1")
        .join("schemas")
        .join(format!("{}.py", name));
    fs::create_dir_all("app/interfaces/api/v1/schemas").ok();

    fs::write(
        &path,
        format!(
            r#"from pydantic import BaseModel

class {0}Base(BaseModel):
    name: str
    description: str

class {0}Create({0}Base):
    pass

class {0}Update({0}Base):
    pass

class {0}Response({0}Base):
    id: str

    class Config:
        from_attributes = True"#,
            capitalized_name
        ),
    )
    .expect("❌ Error al crear el schema");

    append_to_init("app/interfaces/api/v1/schemas/__init__.py", name);
    println!("✅ Schema creado en '{}'", path.display());
}
