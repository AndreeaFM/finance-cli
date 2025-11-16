use anyhow::Result;
use rusqlite::Connection;
use crate::db::list_transactions;

pub fn monthly_report(conn: &Connection, month: &str) -> Result<()> {
    let txs = list_transactions(conn)?;
    println!("Monthly Report for {}\n", month);

    for tx in txs {
        if tx.date.format("%Y-%m").to_string() == month {
            println!(
                "{} | {:>8.2} | {}",
                tx.date,
                tx.amount,
                tx.description
            );
        }
    }

    Ok(())
}
