use amicis_core::Name;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct HelloArgs {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Greet { name: Name },
}
