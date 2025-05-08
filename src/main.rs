mod cli;
mod handlers;
mod utils;

use cli::{Cli, Commands, MakeItem};
use clap::Parser;
use handlers::new::NewCommand;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, dependencies, options, no_interactive } => {
            let command = NewCommand::new(&options);
            command.execute(&name, dependencies, no_interactive).unwrap();
        }
        Commands::Make { item } => match item {
            MakeItem::Route { name, method } => handlers::make::create_route(&name, method),
            MakeItem::Model { name } => handlers::make::create_model(&name),
            MakeItem::Service { name } => handlers::make::create_component("services", &name, "💼 Service", "service"),
            MakeItem::Core { name } => handlers::make::create_component("core", &name, "🧠 Core logic", "core"),
        },
        Commands::Usecase { name } => handlers::usecase::create_usecase(&name),
    }
}