

[![Created by 1Terabit](https://img.shields.io/badge/Created%20by-1Terabit-FF8C00?style=flat-square&logoColor=white)](https://github.com/1Terabit) [![Rust](https://img.shields.io/badge/Rust-E33616?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/) [![FastAPI](https://img.shields.io/badge/FastAPI-005571?style=flat-square&logo=fastapi)](https://fastapi.tiangolo.com/) [![Python](https://img.shields.io/badge/Python-3776AB?style=flat-square&logo=python&logoColor=white)](https://www.python.org/) 

<p align="center">
  <img src="./assets/faspi.png" alt="Faspi Logo" width="250">
</p>

[Español](#español) | [English](#english)

---
<a name="english"></a>

# 🚀 FastAPI CLI in Rust


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

---

&nbsp;
&nbsp;

---

<a name="español"></a>
# 🚀 FastAPI CLI en Rust

## 📝 Descripción
Un CLI potente y eficiente para generar proyectos FastAPI con arquitectura hexagonal, escrito en Rust.

## ✨ Características
- 🏗️ Generación de proyectos con arquitectura hexagonal
- 🛠️ Creación de rutas, modelos y servicios
- 🧪 Generación automática de tests
- 🔄 Actualización automática de módulos
- 🎯 Agnóstico al entorno de Python

## 📦 Instalación y Uso

### 🛠️ Instalación Global
```bash
cargo install faspi
```

### 💻 Comandos Básicos
```bash
# Crear nuevo proyecto
faspi new myapp

# Generar componentes
faspi make route user
faspi make model user
faspi make service user
faspi make core user
faspi make ports user
faspi make infrastructure user
```

### 🎯 Características Opcionales
Al crear un nuevo proyecto, puedes incluir características adicionales:

```bash
# Crear proyecto con características opcionales
faspi new myapp --sql        # Incluir soporte para SQLAlchemy ORM
faspi new myapp --supabase   # Incluir integración con Supabase
faspi new myapp --auth       # Incluir autenticación JWT
faspi new myapp --cors       # Incluir middleware CORS
faspi new myapp --cache      # Incluir soporte para Redis y FastAPI-Cache
faspi new myapp --tasks      # Incluir cola de tareas Celery y monitoreo Flower

# Combinar múltiples características
faspi new myapp --sql --auth --cache
```

## 📁 Estructura del Proyecto

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

## 🔧 Requisitos Técnicos
- ⚙️ Rust y Cargo instalados
- 💾 Sistema compatible (Linux, macOS, Windows)
- 📝 Permisos de escritura en el directorio de trabajo

## 🧠 Entornos Python
El CLI genera la estructura base, y tú eliges tu entorno preferido:

```bash
# Ejemplo con venv
cd myapp
python -m venv .venv
source .venv/bin/activate  # En Windows: .venv\Scripts\activate
pip install -r requirements.txt
uvicorn main:app --reload
```

## 📚 Documentación
Para más detalles sobre los comandos disponibles:
```bash
faspi --help
faspi make --help
```

---

## 📄 Licencia


Este proyecto está bajo la Licencia MIT - vea los archivos [LICENSE](LICENSE) (inglés) y [LICENSE.es](LICENSE.es) (español) para más detalles.


[![Licencia MIT](https://img.shields.io/badge/Licencia-MIT-97CA00?style=flat-square&logo=opensourceinitiative&logoColor=white)](LICENSE)



