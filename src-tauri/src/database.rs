use rusqlite::{Connection, Result};
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::routes::drzava::Drzava;
use crate::routes::drzava::NewDrzava;

use crate::routes::organizator::Organizator;
use crate::routes::organizator::NewOrganizator;

use crate::routes::dogadaj::Dogadaj;
use crate::routes::dogadaj::NewDogadaj;


use crate::routes::volonter::NewVolonter;
use crate::routes::volonter::Volonter;

use crate::routes::vjestina::Vjestina;
use crate::routes::vjestina::NewVjestina;

use crate::routes::grad::Grad;
use crate::routes::grad::NewGrad;

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

    /* Tablica Dogadaj */
    pub async fn get_dogadaj_values(&self) -> Result<Vec<Dogadaj>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT id, naziv, datum_vrijeme, opis, potrebni_volonteri FROM dogadaj")?;

        let dogadaji = stmt
            .query_map([], |row| {
                Ok(Dogadaj {
                    id: row.get(0)?,
                    naziv: row.get(1)?,
                    datum_vrijeme: row.get::<_, String>(2)?,
                    opis: row.get(3)?,
                    potrebni_volonteri: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(dogadaji)
    }
    
    pub async fn create_dogadaj(&self, dogadaj: NewDogadaj) -> Result<()> {
        let conn = self.conn.lock().await;

        let mut stmt = conn.prepare(
            "INSERT INTO dogadaj (naziv, datum_vrijeme, opis, potrebni_volonteri) VALUES (?, ?, ?, ?)",
        )?;

        stmt.execute((
            &dogadaj.naziv,
            &dogadaj.datum_vrijeme.format("%Y-%m-%d %H:%M:%S").to_string(),
            &dogadaj.opis,
            &dogadaj.potrebni_volonteri,
        ))?;

        println!(
            "Unešen novi zapis u tablicu dogadaj: {}, {}, {}, {}",
            dogadaj.naziv,
            dogadaj.datum_vrijeme,
            dogadaj.opis,
            dogadaj.potrebni_volonteri
        );

        Ok(())
    }

    /* Tablica Volonter */
    pub async fn get_volonter_values(&self) -> Result<Vec<Volonter>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT id, ime, prezime, mail, telefon, datum_pridruzivanja FROM volonter")?;

        let volonteri = stmt
            .query_map([], |row| {
                Ok(Volonter {
                    id: row.get(0)?,
                    ime: row.get(1)?,
                    prezime: row.get(2)?,
                    mail: row.get(3)?,
                    telefon: row.get(4)?,
                    datum_pridruzivanja: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(volonteri)
    }
    
    pub async fn create_volonter(&self, volonter: NewVolonter) -> Result<()> {
        let conn = self.conn.lock().await;

        let mut stmt = conn.prepare(
            "INSERT INTO volonter (ime, prezime, mail, telefon, datum_pridruzivanja) VALUES (?, ?, ?, ?, ?)",
        )?;

        stmt.execute((
            &volonter.ime,
            &volonter.prezime,
            &volonter.mail,
            &volonter.telefon,
            &volonter.datum_pridruzivanja.format("%Y-%m-%d").to_string()
        ))?;

        println!(
            "Unešen novi zapis u tablicu volonter: {}, {}, {}, {}, {}",
            volonter.ime,
            volonter.prezime,
            volonter.mail,
            volonter.telefon,
            volonter.datum_pridruzivanja
        );

        Ok(())
    }

    /* Tablica Vjestina */
    pub async fn get_vjestina_values(&self) -> Result<Vec<Vjestina>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT id, naziv, opis FROM vjestina")?;

        let vjestine = stmt
            .query_map([], |row| {
                Ok(Vjestina {
                    id: row.get(0)?,
                    naziv: row.get(1)?,
                    opis: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(vjestine)
    }
    
    pub async fn create_vjestina(&self, vjestina: NewVjestina) -> Result<()> {
        let conn = self.conn.lock().await;

        let mut stmt = conn.prepare(
            "INSERT INTO vjestina (naziv, opis) VALUES (?, ?)",
        )?;

        stmt.execute((
            &vjestina.naziv,
            &vjestina.opis,
        ))?;

        println!(
            "Unešen novi zapis u tablicu volonter: {}, {}",
            vjestina.naziv,
            vjestina.opis,
        );

        Ok(())
    }
    
    /* ------------------------------- SLOZENE TABLICE U BAZI ------------------------------- */
    /* Tablica Grad */
    pub async fn get_grad_value(&self) -> Result<Vec<Grad>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT id, naziv, id_drzava FROM grad")?;

        let grad = stmt
            .query_map([], |row| {
                Ok(Grad {
                    id: row.get(0)?,
                    naziv: row.get(1)?,
                    id_drzava: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(grad)
    }
    
    pub async fn create_grad(&self, grad: NewGrad) -> Result<()> {
        let conn = self.conn.lock().await;

        let mut stmt = conn.prepare(
            "INSERT INTO grad (naziv, id_drzava) VALUES (?, ?)",
        )?;

        stmt.execute((
            &grad.naziv,
            &grad.id_drzava,
        ))?;

        println!(
            "Unešen novi zapis u tablicu volonter: {}, {}",
            grad.naziv,
            grad.id_drzava,
        );

        Ok(())
    }
}