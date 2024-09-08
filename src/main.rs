use crate::jarayon::{kitob_qoshish, kitoblarni_korish};

mod models;
mod db;
mod jarayon;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = db::baza_ochish("kitoblar.db")?;

    kitob_qoshish(&db, "Rust dasturlash tili", "Steve klabnik")?;
    kitob_qoshish(&db, "Python dastrulash tili", "Andrew Hunt")?;

    kitoblarni_korish(&db)?;

    Ok(())
}
