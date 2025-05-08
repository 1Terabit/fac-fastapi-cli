use std::path::Path;
use std::fs;
use crate::utils::fs::append_to_init;
use crate::utils::testing::*;

pub fn create_route(name: &str, method: Option<String>) {
    let path = Path::new("app").join("routes").join(format!("{}.py", name));
    fs::create_dir_all("app/routes").ok();

    let http_method = method.unwrap_or_else(|| "GET".to_string());

    fs::write(
        &path,
        format!(
            r#"from fastapi import APIRouter, Depends
from app.infrastructure.example_impl import ExampleImpl

router = APIRouter()

@router.{0}("/{1}")
def read_{1}(service: ExampleImpl = Depends()):
    return {{"message": service.get_data()}}
"#,
            http_method.to_lowercase(),
            name
        ),
    )
    .expect("❌ Failed to write route file");

    append_to_init("app/routes/__init__.py", name);
    println!("✅ Route created at '{}'", path.display());

    let test_content = generate_route_test(name, &http_method);
    create_test_file("route", name, &test_content);
}

pub fn create_model(name: &str) {
    let path = Path::new("app").join("models").join(format!("{}.py", name));
    fs::create_dir_all("app/models").ok();

    fs::write(
        &path,
        format!(
            r#"# Model: {0}

def {0}_example():
    return "🚀 Model '{0}' is ready!"
"#,
            name
        ),
    )
    .expect("❌ Failed to write model file");

    append_to_init("app/models/__init__.py", name);
    println!("✅ Model created at '{}'", path.display());

    let test_content = generate_model_test(name);
    create_test_file("model", name, &test_content);
}

pub fn create_component(folder: &str, name: &str, comment: &str, suffix: &str) {
    let dir = Path::new("app").join(folder);
    let path = dir.join(format!("{}.py", name));
    fs::create_dir_all(&dir).ok();

    fs::write(
        &path,
        format!(
            r#"# {2}: {0}
            # Archivo generado en carpeta: {1}

def {0}_{3}():
    return "🚀 {2} '{0}' is ready!"
"#,
            name, folder, comment, suffix
        ),
    )
    .expect(&format!("❌ Failed to write {}", suffix));

    append_to_init(&format!("app/{}/__init__.py", folder), name);
    println!("✅ {} created at '{}'", suffix.to_uppercase(), path.display());

    let test_content = match suffix {
        "service" => generate_service_test(name),
        "core" => generate_core_test(name),
        _ => panic!("Unknown component type: {}", suffix)
    };
    create_test_file(suffix, name, &test_content);
}
