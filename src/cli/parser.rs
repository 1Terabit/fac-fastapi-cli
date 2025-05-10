use clap::Parser;
use super::commands::Commands;

#[derive(Parser)]
#[command(name = "")]
#[command(version = r#"

   ████████╗  █████╗  ███████╗ ██████╗  ╔╗     
   ██╔═════╝ ██╔══██╗ ██╔════╝ ██╔══██╗ ╚╝     
   █████╗    ███████║ ███████╗ ██████╔╝ ╔╗     
   ██╔══╝    ██╔══██║ ╚════██║ ██╔═══╝  ║█     
   ██║       ██║  ██║ ███████║ ██║      ║█     
   ╚═╝       ╚═╝  ╚═╝ ╚══════╝ ╚═╝      ╚╝     
                    v1.1.0            

"#)]
#[command(about = r#"

   ████████╗  █████╗  ███████╗ ██████╗  ╔╗     
   ██╔═════╝ ██╔══██╗ ██╔════╝ ██╔══██╗ ╚╝     
   █████╗    ███████║ ███████╗ ██████╔╝ ╔╗     
   ██╔══╝    ██╔══██║ ╚════██║ ██╔═══╝  ║█     
   ██║       ██║  ██║ ███████║ ██║      ║█     
   ╚═╝       ╚═╝  ╚═╝ ╚══════╝ ╚═╝      ╚╝     
          💫 Hecho con ❤️ en Rust            

  new     🆕 Create a new FastAPI application
  make    🏗️  Generate components (routes, models, etc)
  usecase 🧠 Create a use case
"#)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
