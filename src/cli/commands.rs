use clap::Subcommand;
use super::options::NewOptions;

#[derive(Subcommand)]
pub enum Commands {
    /// 🆕 Create a new FastAPI app
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
    /// 🧠 Create a use case to connect ports, services, and implementations
    Usecase {
        name: String,
    },
}

#[derive(Subcommand)]
pub enum MakeItem {
    Route {
        name: String,
        #[arg(short, long, help = "HTTP method for the route (GET, POST, etc.)")]
        method: Option<String>,
    },
    Model {
        name: String,
    },
    Service {
        name: String,
    },
    Core {
        name: String,
    },
}
