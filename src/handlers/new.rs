use std::path::Path;
use std::fs;
use crate::cli::NewOptions;
use crate::handlers::features::Features;
use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;

pub fn scaffold_project(
    name: &str,
    dependencies: Option<String>,
    options: &NewOptions,
) {
    println!("🛠️  Creating new FastAPI app: {}", name);

    let root = Path::new(name);
    let app = root.join("app");
    
    // Estructura principal
    fs::create_dir_all(&app).unwrap();
    fs::write(app.join("__init__.py"), "").unwrap();

    // Configuración
    let config = app.join("config");
    fs::create_dir_all(&config).unwrap();
    fs::write(config.join("__init__.py"), "").unwrap();
    fs::write(config.join("settings.py"), "# Configuración del entorno\n").unwrap();

    // Core (Dominio)
    let core = app.join("core");
    fs::create_dir_all(&core.join("entities")).unwrap();
    fs::create_dir_all(&core.join("value_objects")).unwrap();
    fs::create_dir_all(&core.join("interfaces")).unwrap();
    fs::write(core.join("exceptions.py"), "# Excepciones de dominio\n").unwrap();
    fs::write(core.join("types.py"), "# Tipos comunes\n").unwrap();
    create_init_files(&core);

    // Application (Casos de Uso)
    let application = app.join("application");
    fs::create_dir_all(&application.join("use_cases")).unwrap();
    fs::create_dir_all(&application.join("services")).unwrap();
    create_init_files(&application);

    // Infrastructure (Implementaciones)
    let infrastructure = app.join("infrastructure");
    fs::create_dir_all(&infrastructure.join("database/models")).unwrap();
    fs::create_dir_all(&infrastructure.join("database/repositories")).unwrap();
    fs::create_dir_all(&infrastructure.join("external_apis")).unwrap();
    fs::create_dir_all(&infrastructure.join("utils")).unwrap();
    create_init_files(&infrastructure);

    // Interfaces (Adaptadores de entrada)
    let interfaces = app.join("interfaces");
    fs::create_dir_all(&interfaces.join("api/v1/endpoints")).unwrap();
    fs::create_dir_all(&interfaces.join("api/v1/schemas")).unwrap();
    fs::create_dir_all(&interfaces.join("api/v1/dependencies")).unwrap();
    create_init_files(&interfaces);

    // Tests
    let tests = root.join("tests");
    fs::create_dir_all(&tests.join("core")).unwrap();
    fs::create_dir_all(&tests.join("application")).unwrap();
    fs::create_dir_all(&tests.join("infrastructure")).unwrap();
    fs::create_dir_all(&tests.join("interfaces")).unwrap();
    create_init_files(&tests);

    create_main_app(&app);
    create_example_files(&app);
    create_project_files(root, dependencies, options);

    println!("✅ Project '{}' created with clean hexagonal architecture!", name);
}

fn create_init_files(dir: &Path) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            fs::write(path.join("__init__.py"), "").unwrap();
            create_init_files(&path);
        }
    }
}

fn create_main_app(app: &Path) {
    let main_content = r#"from fastapi import FastAPI
from app.interfaces.api.v1.endpoints import router as api_router
from app.config.settings import Settings

def create_app() -> FastAPI:
    settings = Settings()
    app = FastAPI(title=settings.PROJECT_NAME)
    
    app.include_router(api_router, prefix="/api/v1")
    
    return app

app = create_app()

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("main:app", host="0.0.0.0", port=8000, reload=True)
"#;
    fs::write(app.join("main.py"), main_content).unwrap();
}

fn create_example_files(app: &Path) {
    // Ejemplo de entidad en el dominio
    let entity_content = r#"from dataclasses import dataclass
from app.core.types import ID

@dataclass(frozen=True)
class ExampleEntity:
    id: ID
    name: str
    description: str"#;
    fs::write(app.join("core/entities/example.py"), entity_content).unwrap();

    // Ejemplo de caso de uso
    let usecase_content = r#"from app.core.interfaces.example_repository import ExampleRepository
from app.core.entities.example import ExampleEntity

class GetExampleUseCase:
    def __init__(self, repository: ExampleRepository):
        self.repository = repository

    def execute(self, example_id: str) -> ExampleEntity:
        return self.repository.get_by_id(example_id)"#;
    fs::write(app.join("application/use_cases/get_example.py"), usecase_content).unwrap();

    // Ejemplo de endpoint
    let endpoint_content = r#"from fastapi import APIRouter, Depends
from app.application.use_cases.get_example import GetExampleUseCase
from app.interfaces.api.v1.schemas.example import ExampleResponse
from app.interfaces.api.v1.dependencies.repositories import get_example_repository

router = APIRouter(prefix="/examples", tags=["examples"])

@router.get("/{example_id}", response_model=ExampleResponse)
def get_example(
    example_id: str,
    use_case: GetExampleUseCase = Depends(lambda: GetExampleUseCase(get_example_repository()))
):
    example = use_case.execute(example_id)
    return ExampleResponse.from_entity(example)"#;
    fs::write(app.join("interfaces/api/v1/endpoints/example.py"), endpoint_content).unwrap();
}

fn create_project_files(root: &Path, dependencies: Option<String>, options: &NewOptions) {
    fs::write(root.join("README.md"), "# 🚀 FastAPI App\n").ok();
    
    let mut env_vars = String::from("# 🔐 Environment variables\n");

    if options.sql {
        env_vars.push_str("\n# Database Configuration\nDATABASE_URL=postgresql://user:password@localhost/dbname\n");
    }

    if options.supabase {
        env_vars.push_str("\n# Supabase Configuration\nSUPABASE_URL=your-project-url\nSUPABASE_KEY=your-anon-key\nSUPABASE_SECRET_KEY=your-service-role-key\n");
    }

    if options.auth {
        env_vars.push_str("\n# JWT Authentication\nSECRET_KEY=your-secret-key\nALGORITHM=HS256\nACCESS_TOKEN_EXPIRE_MINUTES=30\n");
    }

    if options.cache {
        env_vars.push_str("\n# Redis Cache Configuration\nREDIS_URL=redis://localhost:6379\n");
    }

    if options.tasks {
        env_vars.push_str("\n# Celery Configuration\nCELERY_BROKER_URL=redis://localhost:6379/0\nCELERY_RESULT_BACKEND=redis://localhost:6379/0\n");
    }

    fs::write(root.join(".env.example"), env_vars).ok();
    fs::write(
        root.join(".gitignore"),
        r#"
__pycache__/
*.pyc
venv/
.env
"#,
    ).ok();
    
    let mut requirements = String::from("fastapi==0.115.12\nuvicorn==0.34.2\npydantic==2.11.4\npython-dotenv==1.1.0\n");

    if options.sql {
        requirements.push_str("sqlalchemy==2.0.0\nalembic==1.13.0\npsycopg2-binary==2.9.9\n");
    }

    if options.supabase {
        requirements.push_str("supabase==2.3.4\nsupabase-client==2.15.1\n");
    }

    if options.auth {
        requirements.push_str("python-jose[cryptography]==3.3.0\npasslib[bcrypt]==1.7.4\n");
    }

    if options.cors {
        requirements.push_str("fastapi-cors==0.0.6\n");
    }

    if options.cache {
        requirements.push_str("redis==5.0.1\nfastapi-cache2==0.2.1\n");
    }

    if options.tasks {
        requirements.push_str("celery==5.3.6\nflower==2.0.1\n");
    }
    if let Some(deps) = dependencies {
        requirements.push_str(&deps);
    }
    fs::write(root.join("requirements.txt"), requirements).ok();
}

pub struct NewCommand {
    sql: bool,
    supabase: bool,
    auth: bool,
    cors: bool,
    cache: bool,
    tasks: bool,
}

impl NewCommand {
    pub fn new(options: &NewOptions) -> Self {
        NewCommand {
            sql: options.sql,
            supabase: options.supabase,
            auth: options.auth,
            cors: options.cors,
            cache: options.cache,
            tasks: options.tasks,
        }
    }

    pub fn execute(&self, name: &str, dependencies: Option<String>, no_interactive: bool) -> Result<(), Box<dyn std::error::Error>> {
        println!("🛠️  Creating new FastAPI app: {}", name);
    
        let root = Path::new(name);
        let app = root.join("app");
        
        let options = self.get_interactive_options(no_interactive)?;
        scaffold_project(name, dependencies, &options);
    
        self.setup_features(&root, &app)?;
        Ok(())
    }

    fn get_interactive_options(&self, no_interactive: bool) -> Result<NewOptions, Box<dyn std::error::Error>> {
        if no_interactive {
            return Ok(NewOptions {
                sql: self.sql,
                supabase: self.supabase,
                auth: self.auth,
                cors: self.cors,
                cache: self.cache,
                tasks: self.tasks,
            });
        }

        println!("📦 Let's configure your FastAPI project:");
        
        let options = NewOptions {
            sql: Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to include SQLAlchemy ORM support?")
                .default(false)
                .interact()?,
            supabase: Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to include Supabase integration?")
                .default(false)
                .interact()?,
            auth: Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to include JWT authentication?")
                .default(false)
                .interact()?,
            cors: Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to include CORS middleware?")
                .default(false)
                .interact()?,
            cache: Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to include Redis and FastAPI-Cache support?")
                .default(false)
                .interact()?,
            tasks: Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to include Celery task queue and Flower monitoring?")
                .default(false)
                .interact()?,
        };

        Ok(options)
    }

    fn setup_features(&self, root: &Path, app: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let features = Features::new(root, app);

        if self.sql { features.setup_database()?; }
        if self.supabase { features.setup_supabase()?; }
        if self.auth { features.setup_auth()?; }
        if self.cors { features.setup_cors()?; }
        if self.cache { features.setup_cache()?; }
        if self.tasks { features.setup_tasks()?; }

        Ok(())
    }
}
