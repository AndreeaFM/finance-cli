use anyhow::Result;
use chrono::NaiveDate;
use rusqlite::{params, Connection};
use crate::models::Transaction;

pub fn open_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            description TEXT NOT NULL,
            amount REAL NOT NULL,
            category TEXT,
            account TEXT,
            raw_data TEXT
        );
        "#,
    )?;
    Ok(conn)
}

pub fn insert_transaction(conn: &Connection, tx: &Transaction) -> Result<()> {
    conn.execute(
        r#"
        INSERT INTO transactions (date, description, amount, category, account, raw_data)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
        params![
            tx.date.format("%Y-%m-%d").to_string(),
            tx.description,
            tx.amount,
            tx.category,
            tx.account,
            tx.raw_data,
        ],
    )?;
    Ok(())
}

pub fn list_transactions(conn: &Connection) -> Result<Vec<Transaction>> {
    let mut stmt = conn.prepare(
        "SELECT id, date, description, amount, category, account, raw_data FROM transactions ORDER BY date ASC"
    )?;

    let rows = stmt.query_map([], |row| {
        let date_str: String = row.get(1)?;
        Ok(Transaction {
            id: row.get(0)?,
            date: NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap(),
            description: row.get(2)?,
            amount: row.get(3)?,
            category: row.get(4)?,
            account: row.get(5)?,
            raw_data: row.get(6)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;

    Ok(rows)
}
