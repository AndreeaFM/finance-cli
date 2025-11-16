use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "finance-cli")]
#[command(about = "Personal Finance CLI Manager")]
pub struct Cli {
    #[arg(long, global = true, default_value = "finance.db")]
    pub db: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Import transactions from a CSV file
    Import {
        #[arg(long)]
        file: PathBuf,
    },

    /// Add a single transaction manually
    Add {
        #[arg(long)]
        date: String,
        #[arg(long)]
        amount: f64,
        #[arg(long)]
        description: String,
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        account: Option<String>,
    },

    /// Generate a monthly report
    Report {
        #[arg(long)]
        month: String,
    },
}
