use std::error::Error;
use clap::{Parser, Subcommand};
use crate::add::add;
use crate::init::create;

mod github;
mod init;
mod constants;
mod add;
mod utils;

#[derive(Subcommand)]
enum Commands {
  #[command(alias = "create")]
  New {
    project_name: String
  },
  Add {
    component_names: Vec<String>
  }
}
#[derive(Parser)]
#[command(name = "yewi")]
#[command(about = "Yewi CLI - A tool to manage Yewi components", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Commands
}

fn main() -> Result<(), Box<dyn Error>> {
  let cli = Cli::parse();

  match cli.command {
    Commands::New { project_name } => {
      create(&project_name)?;
    }
    Commands::Add { component_names } => {
      for component_name in component_names {
        add(&component_name)?;
      };
    }
  }

  Ok(())
}
