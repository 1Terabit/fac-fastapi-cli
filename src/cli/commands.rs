use clap::Subcommand;
use super::options::NewOptions;

#[derive(Subcommand)]
pub enum Commands {
    /// 🆕 Create a new FastAPI project
    New {
        name: String,
        #[arg(short, long, help = "Additional dependencies to include")]
        dependencies: Option<String>,
        #[arg(long, help = "Skip interactive mode")]
        no_interactive: bool,
        #[command(flatten)]
        options: NewOptions,
    },
    /// 🏗️ Generate components like route or model
    Make {
        #[command(subcommand)]
        item: MakeItem,
    },
    /// 🧩 Create a feature to organize your code
        Entity {
        name: String,
    },
    /// 🧠 Create a use case to connect ports, services, and implementations
    Usecase {
        name: String,
    },
}

#[derive(Subcommand)]
pub enum MakeItem {
    /// 🏗️ Create a complete entity with all its components
    Entity {
        /// Entity name
        name: String,
    },
}
