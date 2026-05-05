use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open() -> Result<Self> {
        let path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rusticize")
            .join("progress.db");
        std::fs::create_dir_all(path.parent().unwrap())?;
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS progress (
                lesson_id TEXT PRIMARY KEY,
                completed INTEGER,
                attempts INTEGER,
                last_code TEXT,
                completed_at INTEGER
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn get_progress(&self, lesson_id: &str) -> Result<Option<Progress>> {
        let mut stmt = self.conn.prepare(
            "SELECT lesson_id, completed, attempts, last_code, completed_at FROM progress WHERE lesson_id = ?1"
        )?;
        let mut rows = stmt.query_map([lesson_id], |row| {
            Ok(Progress {
                lesson_id: row.get(0)?,
                completed: row.get(1)?,
                attempts: row.get(2)?,
                last_code: row.get(3)?,
                completed_at: row.get(4)?,
            })
        })?;
        Ok(rows.next().transpose()?)
    }

    pub fn save_progress(&self, lesson_id: &str, completed: bool, code: &str) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        let existing = self.get_progress(lesson_id)?;
        let attempts = existing.map(|p| p.attempts + 1).unwrap_or(1);
        let completed_at = if completed { Some(now) } else { None };

        self.conn.execute(
            "INSERT INTO progress (lesson_id, completed, attempts, last_code, completed_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(lesson_id) DO UPDATE SET
                completed = excluded.completed,
                attempts = excluded.attempts,
                last_code = excluded.last_code,
                completed_at = COALESCE(excluded.completed_at, completed_at)",
            params![lesson_id, completed, attempts, code, completed_at],
        )?;
        Ok(())
    }

    pub fn get_all_progress(&self) -> Result<Vec<Progress>> {
        let mut stmt = self.conn.prepare(
            "SELECT lesson_id, completed, attempts, last_code, completed_at FROM progress"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Progress {
                lesson_id: row.get(0)?,
                completed: row.get(1)?,
                attempts: row.get(2)?,
                last_code: row.get(3)?,
                completed_at: row.get(4)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.into())
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Progress {
    pub lesson_id: String,
    pub completed: bool,
    pub attempts: i32,
    pub last_code: Option<String>,
    pub completed_at: Option<i64>,
}
