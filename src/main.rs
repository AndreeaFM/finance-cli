mod cli;
mod db;
mod models;
mod import;
mod report;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use chrono::NaiveDate;
use models::Transaction;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = db::open_db(&cli.db)?;

    match cli.command {
        Commands::Import { file } => {
            let count = import::import_csv(&conn, &file)?;
            println!("Imported {} transactions.", count);
        }

        Commands::Add {
            date,
            amount,
            description,
            category,
            account,
        } => {
            let tx = Transaction {
                id: None,
                date: NaiveDate::parse_from_str(&date, "%Y-%m-%d")?,
                description,
                amount,
                category,
                account,
                raw_data: None,
            };
            db::insert_transaction(&conn, &tx)?;
            println!("Transaction added.");
        }

        Commands::Report { month } => {
            report::monthly_report(&conn, &month)?;
        }
    }

    Ok(())
}
