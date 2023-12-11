use sqlite::*;
use std::path::Path;

static DB_BASE_PATH: &str = "/home/garrett/Coding/rencchi/src/data/";

pub async fn init_db(path: &str) -> Result<Connection> {
    let path = format!("{}{}", DB_BASE_PATH, path);
    let file = Path::new(&path);
    let conn = Connection::open(file)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY,
            author_id INTEGER,
            status INTEGER,
            created_date TEXT,
            scheduled_date TEXT
        )",
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS responses (
            session_id INTEGER,
            respondee_id INTEGER,
            status INTEGER,
            responded_date TEXT
        )",
    )?;

    Ok(conn)
}
