use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Option<i64>,
    pub date: NaiveDate,
    pub description: String,
    pub amount: f64,
    pub category: Option<String>,
    pub account: Option<String>,
    pub raw_data: Option<String>,
}
