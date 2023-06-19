use rusqlite::{Connection, params, Result, OpenFlags};
use std::sync::{Arc, Mutex};
pub struct StringDB { conn: Arc<Mutex<Connection>> }
impl StringDB {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE)?;
        conn.pragma_update(None, "journal_mode", &"wal")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS db (value TEXT UNIQUE)",
            params![],
        )?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn add(&self, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        conn.execute(
            "INSERT INTO db (value) VALUES (?)",
            params![value],
        )?;
        Ok(())
    }

    pub fn remove(&self, value: &str) -> Result<usize> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let changes = conn.execute(
            "DELETE FROM db WHERE value = ?",
            params![value],
        )?;
        Ok(changes)
    }
    pub fn flush(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let changes = conn.execute(
            "DELETE * FROM",
            params![],
        )?;
        Ok(changes)
    }
}