use clap::Parser;
use super::commands::Commands;

#[derive(Parser)]
#[command(name = "faspi")]
#[command(version = r#"
use clap::Parser;
use super::commands::Commands;

#[derive(Parser)]
#[command(name = "faspi")]
#[command(version = r#"
╭─────────────────────────────────────────╮
│                                         │
│   ████████╗ █████╗  ███████╗ ██████╗ █╗ │
│   ██╔════╝ ██╔══██╗ ██╔════╝ ██╔══██╗█║ │
│   █████╗   ███████║ ███████╗ ██████╔╝█║ │
│   ██╔══╝   ██╔══██║ ╚════██║ ██╔═══╝ ╚╝ │
│   ██║      ██║  ██║ ███████║ ██║     ╔╗ │
│   ╚═╝      ╚═╝  ╚═╝ ╚══════╝ ╚═╝     ╚╝ │
│                                         │
╰─────────────────────────────────────────╯

💫 Faspi v1.0.0 - Hecho con ❤️ Rust
"#)]
#[command(about = r#"A modern CLI for creating FastAPI projects with hexagonal architecture

AVAILABLE COMMANDS:
  new     🆕 Create a new FastAPI application
  make    🏗️  Generate components (routes, models, etc)
  usecase 🧠 Create a use case
"#)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

💫 Faspi CLI v1.0.0 - Hecho con ❤️ Rust
"#)]
#[command(about = r#"A modern CLI for creating FastAPI projects with hexagonal architecture

AVAILABLE COMMANDS:
  new     🆕 Create a new FastAPI application
  make    🏗️  Generate components (routes, models, etc)
  usecase 🧠 Create a use case
"#)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
