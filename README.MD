<p align="center">
  <img src="./assets/faspi.png" alt="Faspi Logo" width="250">
</p>

<p align="center">
  <a href="https://github.com/1Terabit">
    <img src="https://img.shields.io/badge/Created%20by-1Terabit-FF8C00?style=flat-square&logoColor=white" />
  </a>
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Rust-E33616?style=flat-square&logo=rust&logoColor=white" />
  </a>
  <a href="https://fastapi.tiangolo.com/">
    <img src="https://img.shields.io/badge/FastAPI-005571?style=flat-square&logo=fastapi" />
  </a>
  <a href="https://www.python.org/">
    <img src="https://img.shields.io/badge/Python-3776AB?style=flat-square&logo=python&logoColor=white" />
  </a>
</p>

---
&nbsp;
<h1 align="center">🚀 FastAPI CLI in Rust </h1>


## 📝 Description
A powerful and efficient CLI for generating FastAPI projects with hexagonal architecture, written in Rust.

## ✨ Features
- 🏗️ Project generation with hexagonal architecture
- 🛠️ Route, model, and service creation
- 🧪 Automatic test generation
- 🔄 Automatic module updates
- 🎯 Python environment agnostic

## 📦 Installation and Usage

### 🛠️ Global Installation
```bash
cargo install faspi
```

### 💻 Basic Commands
```bash
# Create new project
faspi new myapp

# Generate components
faspi make route user
faspi make model user
faspi make service user
faspi make core user
faspi make ports user
faspi make infrastructure user
```
### 🎯 Optional Features
When creating a new project, you can include additional features:

```bash
# Create project with optional features
faspi new myapp --sql        # Include SQLAlchemy ORM support
faspi new myapp --supabase   # Include Supabase integration
faspi new myapp --auth       # Include JWT authentication
faspi new myapp --cors       # Include CORS middleware
faspi new myapp --cache      # Include Redis and FastAPI-Cache support
faspi new myapp --tasks      # Include Celery task queue and Flower monitoring

# Combine multiple features
faspi new myapp --sql --auth --cache
```

## 📁 Project Structure

``` bash
myapp/
├── app/
│   ├── __init__.py
│   ├── main.py
│   ├── routes/
│   │   ├── __init__.py
│   │   └── example_router.py
│   ├── models/
│   │   └── __init__.py
│   ├── services/
│   │   └── __init__.py
│   ├── core/
│   │   └── __init__.py
│   ├── ports/
│   │   ├── __init__.py
│   │   └── example_port.py
│   └── infrastructure/
│       ├── __init__.py
│       └── example_impl.py
├── tests/
│   ├── __init__.py
│   ├── conftest.py
│   └── test_example.py
├── .env.example
├── .gitignore
├── requirements.txt
└── README.md
```

## 🔧 Technical Requirements
- ⚙️ Rust and Cargo installed
- 💾 Compatible system (Linux, macOS, Windows)
- 📝 Write permissions in working directory

## 🧠 Python Environments
The CLI generates the base structure, and you choose your preferred environment:

```bash
# Example with venv
cd myapp
python -m venv .venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate
pip install -r requirements.txt
uvicorn main:app --reload
```

## 📚 Documentation
For more details about available commands:
```bash
faspi --help
faspi make --help
```


## 📄 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) (English) and [LICENSE.es](LICENSE.es) (Spanish) files for details.

[![Licence MIT](https://img.shields.io/badge/Licence-MIT-97CA00?style=flat-square&logo=opensourceinitiative&logoColor=white)](LICENSE)
