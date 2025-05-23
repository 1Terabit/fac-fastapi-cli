use std::fs;
use std::path::Path;

pub fn create_test_file(component_type: &str, name: &str, test_content: &str) {
    fs::create_dir_all("tests").unwrap();
    let test_dir = match component_type {
        "api" => "tests/interfaces/api",
        "entity" => "tests/core/entities",
        "repository" => "tests/infrastructure/repositories",
        "usecase" => "tests/application/use_cases",
        _ => "tests",
    };

    fs::create_dir_all(test_dir).unwrap();
    let test_path = Path::new(test_dir).join(format!("test_{}.py", name));

    fs::write(&test_path, test_content)
        .unwrap_or_else(|_| panic!("❌ Failed to write test file for {}", name));

    println!("🧪 Test created at '{}'", test_path.display());
}

pub fn generate_route_test(name: &str, http_method: &str) -> String {
    format!(
        r#"# 🧪 Test for route: {0}
import pytest
from fastapi.testclient import TestClient
from app.main import app

client = TestClient(app)

def test_{0}_route_status():
    response = client.{1}("/{0}")
    assert response.status_code == 200

def test_{0}_route_response():
    response = client.{1}("/{0}")
    data = response.json()
    assert "message" in data
    assert isinstance(data["message"], str)

def test_{0}_route_headers():
    response = client.{1}("/{0}")
    assert response.headers["content-type"] == "application/json"
"#,
        name,
        http_method.to_lowercase()
    )
}

pub fn generate_usecase_test(name: &str) -> String {
    format!(
        r#"# 🧪 Test for use case: {0}
import pytest
from unittest.mock import Mock
from app.usecases.{0} import {0}
from app.ports.example_port import ExamplePort

class MockPort(ExamplePort):
    def get_data(self) -> str:
        return "test data"

def test_{0}_initialization():
    port = MockPort()
    usecase = {0}(port)
    assert isinstance(usecase.port, ExamplePort)

def test_{0}_execution():
    port = MockPort()
    usecase = {0}(port)
    result = usecase.execute()
    assert result == "test data"

def test_{0}_with_mock():
    mock_port = Mock(spec=ExamplePort)
    mock_port.get_data.return_value = "mocked data"
    usecase = {0}(mock_port)
    result = usecase.execute()
    assert result == "mocked data"
    mock_port.get_data.assert_called_once()
"#,
        name
    )
}
