mod cli;
mod handlers;
mod utils;

use crate::cli::commands::MakeItem;
use crate::cli::update_checker::UpdateChecker;
use clap::Parser;
use cli::{Cli, Commands};
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
            MakeItem::Entity { name } => handlers::make::create_entity(&name),
        },
        Commands::Usecase { name } => handlers::usecase::create_usecase(&name),
        Commands::Entity { name } => handlers::make::create_entity(&name),
    }
    let checker = UpdateChecker::new();
    if let Err(e) = checker.check_for_updates() {
        eprintln!("Error checking for updates: {}", e);
    }
}
