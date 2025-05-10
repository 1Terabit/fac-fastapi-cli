mod cli;
mod handlers;
mod utils;

use crate::cli::update_checker::UpdateChecker;
use clap::Parser;
use cli::{Cli, Commands, MakeItem};
use handlers::new::NewCommand;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::New {
            name,
            dependencies,
            options,
            no_interactive,
        } => {
            let command = NewCommand::new(&options);
            if let Err(e) = command.execute(&name, dependencies, no_interactive) {
                eprintln!("Error creating new project: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Make { item } => match item {
            MakeItem::Route { name, method } => handlers::make::create_route(&name, method),
            MakeItem::Model { name } => handlers::make::create_model(&name),
            MakeItem::Service { name } => {
                handlers::make::create_component("services", &name, "💼 Service", "service")
            }
            MakeItem::Core { name } => {
                handlers::make::create_component("core", &name, "🧠 Core logic", "core")
            }
        },
        Commands::Usecase { name } => handlers::usecase::create_usecase(&name),
    }
    let checker = UpdateChecker::new();
    if let Err(e) = checker.check_for_updates() {
        eprintln!("Error checking for updates: {}", e);
    }
}
