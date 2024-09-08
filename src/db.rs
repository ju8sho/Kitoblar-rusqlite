use rusqlite::{Connection, Result};
pub fn baza_ochish(fayil_nomi: &str) -> Result<Connection> {
    let db = Connection::open(fayil_nomi)?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS kitoblar (\
        id INTEGER NOT NULL PRIMARY KEY,\
        name TEXT NOT NULL,\
        muallif TEXT NOT NULL\
        )",
        []
    )?;
    Ok(db)
}