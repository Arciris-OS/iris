use clap::{Parser, Subcommand};

mod install;
mod sync;
mod check;
mod info;
mod search;
mod utils;


#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    /// Install package from iris repository
    Install {
        #[arg(long, short)]
        sync: bool,
        package: Vec<String>,
    },

    /// Sync a iris repository
    Sync,

    /// Check all package sums
    Check {
        #[arg(long)]
        asynchronous: bool,
        #[arg(long)]
        max_jobs: Option<u64>,
        #[arg(long)]
        verify: bool,
    },

    /// Display package's info
    Info {
        package: String,
    },

    /// Search all packages
    Search {
        regex: Option<String>,
    }
}



fn main() {
    let parse = Cli::parse();


    match parse.commands {
        SubCommands::Install { sync, package } => {
            todo!();
        },

        SubCommands::Sync => todo!(),

        SubCommands::Info { package } => todo!("{}", package),
        SubCommands::Check { asynchronous, max_jobs, verify } => todo!(),

        SubCommands::Search { regex } => todo!(),

    }

}
