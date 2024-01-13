use std::fmt;
use std::fmt::Formatter;
use chrono::{DateTime, Utc};
use rusqlite::{Row, Result};
use uuid::Uuid;

pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub done: bool
}

impl fmt::Display for Task {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
           "{} : {} => [{}] {}",
            self.created_at.format("%d/%m/%Y %H:%M"),
            self.id,
            if self.done { "x" } else { " " },
            self.description
        )
    }
}

impl Task {

    pub fn new(description: String) -> Self {
        Self { id: Uuid::new_v4(), description, created_at: Utc::now(), done: false }
    }

    pub fn parse_row(row: &Row) -> Result<Task> {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            created_at: row.get(2)?,
            done: row.get(3)?
        })
    }
}