# Kitoblar-sqllite

**Kitoblar-sqllite** — bu Rust tilida yozilgan va SQLite bilan ishlaydigan oddiy loyiha. Ushbu dastur orqali kitoblarning ma'lumotlarini saqlash va ularga kirish mumkin.

## O'rnatish

Loyihani quyidagi amallar orqali o'rnatasiz:

```bash
git clone https://github.com/username/kitoblar-sqllite.git
cd kitoblar-sqllite
cargo build
cargo run

## Foydalanish
Ma'lumotlar bazasi bilan ishlash misoli:
'''bash
use rusqlite::{params, Connection};

let conn = Connection::open("kitoblar.db").unwrap();
conn.execute(
    "CREATE TABLE kitoblar (
        id INTEGER PRIMARY KEY,
        nom TEXT NOT NULL,
        muallif TEXT NOT NULL
    )",
    [],
).unwrap();
