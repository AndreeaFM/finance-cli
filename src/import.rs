use anyhow::Result;
use chrono::NaiveDate;
use rusqlite::Connection;
use serde::Deserialize;
use crate::models::Transaction;
use crate::db::insert_transaction;

#[derive(Debug, Deserialize)]
pub struct CsvRecord {
    pub date: String,
    pub description: String,
    pub amount: f64,
    pub category: Option<String>,
    pub account: Option<String>,
}

pub fn import_csv(conn: &Connection, path: &std::path::Path) -> Result<usize> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut count = 0;

    for result in rdr.deserialize() {
        let rec: CsvRecord = result?;
        let date = NaiveDate::parse_from_str(&rec.date, "%Y-%m-%d")?;

        let tx = Transaction {
            id: None,
            date,
            description: rec.description,
            amount: rec.amount,
            category: rec.category,
            account: rec.account,
            raw_data: None,
        };

        insert_transaction(conn, &tx)?;
        count += 1;
    }

    Ok(count)
}
