# fac (FastAPI CLI)

[![Created by 1Terabit](https://img.shields.io/badge/created%20by-1Terabit-orange.svg)](https://github.com/1Terabit)

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
cargo install fac
```

### 💻 Basic Commands
```bash
# Create new project
fac new myapp

# Generate components
fac make route user
fac make model user
fac make service user
fac make core user
fac make ports user
fac make infrastructure user
```
### 🎯 Optional Features
When creating a new project, you can include additional features:

```bash
# Create project with optional features
fac new myapp --sql        # Include SQLAlchemy ORM support
fac new myapp --supabase   # Include Supabase integration
fac new myapp --auth       # Include JWT authentication
fac new myapp --cors       # Include CORS middleware
fac new myapp --cache      # Include Redis and FastAPI-Cache support
fac new myapp --tasks      # Include Celery task queue and Flower monitoring

# Combine multiple features
fac new myapp --sql --auth --cache
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
fac --help
fac make --help
```


## 📄 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) (English) and [LICENSE.es](LICENSE.es) (Spanish) files for details.

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

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
cargo install fac
```

### 💻 Comandos Básicos
```bash
# Crear nuevo proyecto
fac new myapp

# Generar componentes
fac make route user
fac make model user
fac make service user
fac make core user
fac make ports user
fac make infrastructure user
```

### 🎯 Características Opcionales
Al crear un nuevo proyecto, puedes incluir características adicionales:

```bash
# Crear proyecto con características opcionales
fac new myapp --sql        # Incluir soporte para SQLAlchemy ORM
fac new myapp --supabase   # Incluir integración con Supabase
fac new myapp --auth       # Incluir autenticación JWT
fac new myapp --cors       # Incluir middleware CORS
fac new myapp --cache      # Incluir soporte para Redis y FastAPI-Cache
fac new myapp --tasks      # Incluir cola de tareas Celery y monitoreo Flower

# Combinar múltiples características
fac new myapp --sql --auth --cache
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
fac --help
fac make --help
```

---

## 📄 Licencia


Este proyecto está bajo la Licencia MIT - vea los archivos [LICENSE](LICENSE) (inglés) y [LICENSE.es](LICENSE.es) (español) para más detalles.


[![Licencia MIT](https://img.shields.io/badge/Licencia-MIT-green.svg)](LICENSE)



