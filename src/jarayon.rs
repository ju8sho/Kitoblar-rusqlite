use rusqlite::{Connection, Result, params};
use crate::models::Kitob;

pub fn kitob_qoshish(db: &Connection, name: &str, muallif: &str) -> Result<()> {
    db.execute(
        "INSERT INTO kitoblar (name, muallif) VALUES (?1, ?2)",
        params![name, muallif],
    )?;
    println!("Kitob qo'shildi: {} - {}", name, muallif);
    Ok(())
}


pub fn kitoblarni_korish(db: &Connection) -> Result<()> {
    let mut natija_olish = db.prepare("SELECT id, name, muallif FROM kitoblar")?;

    let kitob_iter = natija_olish.query_map([], |row| {
        Ok(Kitob {
            id: row.get(0)?,
            name: row.get(1)?,
            mualllif: row.get(2)?,
        })
    })?;

    println!("Kitoblar ro'yxati");
    for kitob in kitob_iter {
        let kitob = kitob?;
        println!("{}: {} - {}", kitob.id, kitob.name, kitob.mualllif);
    }
    Ok(())


}