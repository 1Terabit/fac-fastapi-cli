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
    let routes = app.join("routes");
    let models = app.join("models");
    let services = app.join("services");
    let core = app.join("core");
    let ports = app.join("ports");
    let infrastructure = app.join("infrastructure");
    let tests = root.join("tests");

    fs::create_dir_all(&routes).unwrap();
    fs::create_dir_all(&models).unwrap();
    fs::create_dir_all(&services).unwrap();
    fs::create_dir_all(&core).unwrap();
    fs::create_dir_all(&ports).unwrap();
    fs::create_dir_all(&infrastructure).unwrap();
    fs::create_dir_all(&tests).unwrap();

    for dir in [&routes, &models, &services, &core, &ports, &infrastructure] {
        fs::write(dir.join("__init__.py"), format!("# 📦 {} package\n", dir.display())).ok();
    }

    create_main_app(&app);
    create_example_port(&app);
    create_example_impl(&app);
    create_project_files(root, dependencies, options);

    println!("✅ Project '{}' created with hexagonal architecture!", name);
}

fn create_main_app(app: &Path) {
    let main_py = app.join("main.py");
    fs::write(
        &main_py,
        r#"from fastapi import FastAPI
from app.routes import example_router

app = FastAPI()

app.include_router(example_router)

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
"#,
    ).unwrap();
}

fn create_example_port(app: &Path) {
    fs::write(
        app.join("ports").join("example_port.py"),
        r#"from abc import ABC, abstractmethod

class ExamplePort(ABC):
    @abstractmethod
    def get_data(self) -> str:
        pass
"#,
    ).ok();
}

fn create_example_impl(app: &Path) {
    fs::write(
        app.join("infrastructure").join("example_impl.py"),
        r#"from app.ports.example_port import ExamplePort

class ExampleImpl(ExamplePort):
    def get_data(self) -> str:
        return "✅ Hexagonal architecture works!"
"#,
    ).ok();
}

fn create_project_files(root: &Path, dependencies: Option<String>, options: &NewOptions) {
    fs::write(root.join("README.md"), "# 🚀 FastAPI App\n").ok();
    fs::write(root.join(".env.example"), 
        r#"# 🔐 Environment variables
SUPABASE_URL=your-project-url
SUPABASE_KEY=your-anon-key
SUPABASE_SECRET_KEY=your-service-role-key
"#).ok();
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
        
        let mut options = NewOptions {
            sql: self.sql,
            supabase: self.supabase,
            auth: self.auth,
            cors: self.cors,
            cache: self.cache,
            tasks: self.tasks,
        };
    
        if !no_interactive {
            println!("📦 Let's configure your FastAPI project:");

            options.sql = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to include SQLAlchemy ORM support?")
                .default(false)
                .interact()?;

            options.supabase = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to include Supabase integration?")
                .default(false)
                .interact()?;

            options.auth = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to include JWT authentication?")
                .default(false)
                .interact()?;

            options.cors = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to include CORS middleware?")
                .default(false)
                .interact()?;

            options.cache = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to include Redis and FastAPI-Cache support?")
                .default(false)
                .interact()?;

            options.tasks = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to include Celery task queue and Flower monitoring?")
                .default(false)
                .interact()?;
        }
        
        scaffold_project(name, dependencies, &options);
    
        let features = Features::new(&root, &app);  // Ahora root y app están definidas
    
        if self.sql {
            features.setup_database()?;
        }
    
        if self.supabase {
            features.setup_supabase()?;
        }
    
        if self.auth {
            features.setup_auth()?;
        }
    
        if self.cors {
            features.setup_cors()?;
        }
    
        if self.cache {
            features.setup_cache()?;
        }
    
        if self.tasks {
            features.setup_tasks()?;
        }
    
        Ok(())
    }
}
