use rusqlite::{Connection, Result};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::routes::drzava::Drzava;
use crate::routes::drzava::NewDrzava;

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>, 
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub async fn get_table_names(&self) -> Result<Vec<String>> {
        let conn = self.conn.lock().await; 
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
        let table_names = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(table_names)
    }

    pub async fn get_drzave(&self) -> Result<Vec<Drzava>> {
        let conn = self.conn.lock().await; 
        let mut stmt = conn.prepare("SELECT id, naziv FROM drzava")?;
        let drzave = stmt
            .query_map([], |row| {
                Ok(Drzava {
                    id: row.get(0)?,
                    naziv: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(drzave)
    }

    pub async fn create_drzava(&self, drzava: NewDrzava) -> Result<()> {
        let conn = self.conn.lock().await; 

        let naziv = drzava.naziv.clone();

        let mut stmt = conn.prepare("INSERT INTO drzava (naziv) VALUES (?)")?;
        stmt.execute([naziv])?;

        println!("Unešen novi zapis u tablicu drzava: {}", drzava.naziv);
        Ok(())
    }
}