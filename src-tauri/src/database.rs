use rusqlite::{Connection, Result};
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::routes::drzava::Drzava;
use crate::routes::drzava::NewDrzava;

use crate::routes::organizator::Organizator;
use crate::routes::organizator::NewOrganizator;

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

    /* ------------------------------- JEDNOSTAVNE TABLICE U BAZI ------------------------------- */

    /* Tablica drzava */
    pub async fn get_drzava_values(&self) -> Result<Vec<Drzava>> {
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

    /* Tablica Organizator */
    
    pub async fn get_organizator_values(&self) -> Result<Vec<Organizator>> {
        let conn = self.conn.lock().await; 
        let mut stmt = conn.prepare("SELECT id, naziv, kontakt_osoba, telefon, mail FROM organizator")?;
        let organizatori = stmt
            .query_map([], |row| {
                Ok(Organizator {
                    id: row.get(0)?,
                    naziv: row.get(1)?,
                    kontakt_osoba: row.get(2)?,
                    kontakt_telefon: row.get(3)?,
                    mail: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(organizatori)
    }

    pub async fn create_organizator(&self, organizator: NewOrganizator) -> Result<()> {
        let conn = self.conn.lock().await;
    
        let mut stmt = conn.prepare(
            "INSERT INTO organizator (naziv, kontakt_osoba, telefon, mail) VALUES (?, ?, ?, ?)"
        )?;
        
        println!(
            "Preparing to insert: {}, {}, {}, {}",
            organizator.naziv, organizator.kontakt_osoba, organizator.telefon, organizator.mail
        );
        
        stmt.execute((
            &organizator.naziv,
            &organizator.kontakt_osoba,
            &organizator.telefon,
            &organizator.mail,
        ))?;
    
        println!(
            "Unešen novi zapis u tablicu organizator: {}, {}, {}, {}",
            organizator.naziv, organizator.kontakt_osoba, organizator.telefon, organizator.mail
        );
    
        Ok(())
    }

}