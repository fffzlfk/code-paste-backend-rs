use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Paste {
    pub id: String,
    pub expired_at: String,
    pub language: String,
    pub data: String,
}

#[derive(Deserialize)]
pub struct CreatePasteReq {
    pub expired_days: i64,
    pub language: String,
    pub data: String,
}

impl Paste {
    pub fn from_create_paste_req(cpr: CreatePasteReq) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            expired_at: Local::now()
                .add(Duration::days(cpr.expired_days))
                .to_string(),
            language: cpr.language,
            data: cpr.data,
        }
    }

    pub fn from_row(row: &Row) -> Self {
        Self {
            id: row.get(0),
            expired_at: row.get(1),
            language: row.get(2),
            data: row.get(3),
        }
    }
}
