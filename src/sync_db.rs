use rusqlite::{Connection, params, Result, OpenFlags, Statement};
use std::sync::{Arc, Mutex};
pub struct StringDBSync { conn: Arc<Mutex<Connection>>}
impl StringDBSync {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE)?;
        const add_st: Statement = conn.prepare("INSERT INTO db (value) VALUES (?)").unwrap();
        const remove_st: Statement = conn.prepare("DELETE FROM db WHERE value = ?").unwrap();
        conn.pragma_update(None, "journal_mode", &"wal")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS db (value TEXT UNIQUE)",
            params![],
        )?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            // add_st: add_st_prep,
            // remove_st: remove_st_prep
        })
    }

    pub fn add(&self, value: &str) -> Result<()> {
        // let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        StringDBSync::add_st.execute(
            // "INSERT INTO db (value) VALUES (?)",
            params![value],
        )?;
        Ok(())
    }

    pub fn remove(&self, value: &str) -> Result<usize> {
        // let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let changes = StringDBSync::remove_st.execute(
            // "DELETE FROM db WHERE value = ?",
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