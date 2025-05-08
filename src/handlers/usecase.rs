use std::path::Path;
use std::fs;
use crate::utils::fs::append_to_init;
use crate::utils::testing::*;

pub fn create_usecase(name: &str) {
    let path = Path::new("app").join("usecases").join(format!("{}.py", name));
    fs::create_dir_all("app/usecases").ok();

    fs::write(
        &path,
        format!(
            r#"""Use case for connecting ports, services, and implementations.

from app.services.example_service import ExampleService
from app.ports.example_port import ExamplePort

class {0}:
    def __init__(self, port: ExamplePort):
        self.port = port

    def execute(self):
        return self.port.get_data()
"#,
            name
        ),
    )
    .expect("❌ Failed to write usecase file");

    append_to_init("app/usecases/__init__.py", name);
    println!("✅ Usecase created at '{}'", path.display());

    let test_content = generate_usecase_test(name);
    create_test_file("usecase", name, &test_content);
}
