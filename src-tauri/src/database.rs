use chrono::NaiveDateTime;
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

use crate::routes::lokacija::Lokacija;
use crate::routes::lokacija::NewLokacija;

use crate::routes::dogadaj_organizator::DogadajOrganizator;

use crate::routes::volonter_dogadaj::VolonterDogadaj;

use crate::routes::volonter_vjestina::VolonterVjestina;

use crate::routes::povratna_informacija::PovratnaInformacija;
use crate::routes::povratna_informacija::NewPovratnaInformacija;
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

    pub async fn update_drzava(&self, id: i32, naziv: String) -> Result<()> {
        let conn = self.conn.lock().await;
    
        let naziv_clone = naziv.clone();
        let mut stmt = conn.prepare("UPDATE drzava SET naziv = ? WHERE id = ?")?;
    
        stmt.execute((naziv, id))?;
    
        println!("Ažuriran zapis u tablici drzava: id={}, naziv={}", id, naziv_clone);
    
        Ok(())
    }

    pub async fn delete_drzava(&self, id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
    
        let mut stmt = conn.prepare("DELETE FROM drzava WHERE id = ?")?;
    
        stmt.execute((id,))?;
    
        println!("Obrisan zapis iz tablice drzava: id={}", id);
    
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
                    telefon: row.get(3)?,
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

    pub async fn update_organizator(&self, id: i32, naziv: String, kontakt_osoba: String, telefon: String, mail: String) -> Result<()> {
        let conn = self.conn.lock().await;
    
        let naziv_clone = naziv.clone();
        let kontakt_osoba_clone = kontakt_osoba.clone();
        let telefon_clone = telefon.clone();
        let mail_clone = mail.clone();
        let mut stmt = conn.prepare("UPDATE organizator SET naziv = ?, kontakt_osoba = ?, telefon = ?, mail = ?  WHERE id = ?")?;
    
        stmt.execute((naziv, kontakt_osoba, telefon, mail, id))?;
    
        println!("Ažuriran zapis u tablici organizator: id={}, naziv={}, kontakt_osoba={}, telefon={}, mail={}", id, naziv_clone, kontakt_osoba_clone, telefon_clone, mail_clone);
    
        Ok(())
    }

    pub async fn delete_organizator(&self, id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
    
        let mut stmt = conn.prepare("DELETE FROM organizator WHERE id = ?")?;
    
        stmt.execute((id,))?;
    
        println!("Obrisan zapis iz tablice organizator: id={}", id);
    
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
                    opis: row.get::<_, Option<String>>(3)?,
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
            &dogadaj.opis.as_deref(),
            &dogadaj.potrebni_volonteri,
        ))?;

        println!(
            "Unešen novi zapis u tablicu dogadaj: {}, {}, {}, {}",
            dogadaj.naziv,
            dogadaj.datum_vrijeme,
            dogadaj.opis.clone().unwrap_or("N/A".to_string()),
            dogadaj.potrebni_volonteri
        );

        Ok(())
    }  

    pub async fn update_dogadaj(&self, id: i32, naziv: String, datum_vrijeme: NaiveDateTime, opis: Option<String>, potrebni_volonteri: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        
        let opis = opis.unwrap_or_else(|| "N/A".to_string());
        let datum_vrijeme_str = datum_vrijeme.format("%Y-%m-%d %H:%M:%S").to_string();

        let naziv_clone = naziv.clone();
        let datum_vrijeme_clone = datum_vrijeme.clone();
        let opis_clone = opis.clone();
        let potrebni_volonteri_clone = potrebni_volonteri.clone();
        
        

        let mut stmt = conn.prepare("UPDATE dogadaj SET naziv = ?, datum_vrijeme = ?, opis = ?, potrebni_volonteri = ? WHERE id = ?")?;
        
        stmt.execute((naziv, datum_vrijeme_str, opis, potrebni_volonteri, id))?;
    
        println!("Ažuriran zapis u tablici dogadaj: id={}, naziv={}, datum_vrijeme={}, opis={}, potrebni_volonteri={},", id, naziv_clone, datum_vrijeme_clone, opis_clone, potrebni_volonteri_clone);
    
        Ok(())
    }

    pub async fn delete_dogadaj(&self, id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
    
        let mut stmt = conn.prepare("DELETE FROM dogadaj WHERE id = ?")?;
    
        stmt.execute((id,))?;
    
        println!("Obrisan zapis iz tablice dogadaj: id={}", id);
    
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

    pub async fn update_volonter(&self, id: i32, ime: String, prezime: String, mail: String, telefon: String) -> Result<()> {
        let conn = self.conn.lock().await;
                
        let ime_clone = ime.clone();
        let prezime_clone = prezime.clone();
        let mail_clone = mail.clone();
        let telefon_clone = telefon.clone();

        let mut stmt = conn.prepare("UPDATE volonter SET ime = ?, prezime = ?, mail = ?, telefon = ? WHERE id = ?")?;
        
        stmt.execute((ime, prezime, mail, telefon, id))?;
    
        println!("Ažuriran zapis u tablici volonter: id={}, ime={}, prezime={}, telefon={}, mail={},", id, ime_clone, prezime_clone, telefon_clone, mail_clone);
    
        Ok(())
    }

    pub async fn delete_volonter(&self, id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
    
        let mut stmt = conn.prepare("DELETE FROM volonter WHERE id = ?")?;
    
        stmt.execute((id,))?;
    
        println!("Obrisan zapis iz tablice volonter: id={}", id);
    
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
            "Unešen novi zapis u tablicu vjestina: {}, {}",
            vjestina.naziv,
            vjestina.opis,
        );

        Ok(())
    }
    
    pub async fn update_vjestina(&self, id: i32, naziv: String, opis: String) -> Result<()> {
        let conn = self.conn.lock().await;
                
        let naziv_clone = naziv.clone();
        let opis_clone = opis.clone();

        let mut stmt = conn.prepare("UPDATE vjestina SET naziv = ?, opis = ? WHERE id = ?")?;
        
        stmt.execute((naziv, opis, id))?;
    
        println!("Ažuriran zapis u tablici vjestina: id={}, naziv={}, opis={}", id, naziv_clone, opis_clone);
    
        Ok(())
    }

    pub async fn delete_vjestina(&self, id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
    
        let mut stmt = conn.prepare("DELETE FROM vjestina WHERE id = ?")?;
    
        stmt.execute((id,))?;
    
        println!("Obrisan zapis iz tablice vjestina: id={}", id);
    
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
            "Unešen novi zapis u tablicu grad: {}, {}",
            grad.naziv,
            grad.id_drzava,
        );

        Ok(())
    }

    pub async fn update_grad(&self, id: i32, naziv: String, id_drzava: i32) -> Result<()> {
        let conn = self.conn.lock().await;
                
        let naziv_clone = naziv.clone();
        let id_drzava_clone = id_drzava.clone();

        let mut stmt = conn.prepare("UPDATE grad SET naziv = ?, id_drzava = ? WHERE id = ?")?;
        
        stmt.execute((naziv, id_drzava, id))?;
    
        println!("Ažuriran zapis u tablici grad: id={}, naziv={}, id_drzava={}", id, naziv_clone, id_drzava_clone);
    
        Ok(())
    }

    pub async fn delete_grad(&self, id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
    
        let mut stmt = conn.prepare("DELETE FROM grad WHERE id = ?")?;
    
        stmt.execute((id,))?;
    
        println!("Obrisan zapis iz tablice grad: id={}", id);
    
        Ok(())
    }  

    /* Tablica Lokacija */
    pub async fn get_lokacija_values(&self) -> Result<Vec<Lokacija>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT id, adresa, id_grad FROM lokacija")?;

        let lokacija = stmt
            .query_map([], |row| {
                Ok(Lokacija {
                    id: row.get(0)?,
                    adresa: row.get(1)?,
                    id_grad: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(lokacija)
    }
    
    pub async fn create_lokacija(&self, lokacija: NewLokacija) -> Result<()> {
        let conn = self.conn.lock().await;

        let mut stmt = conn.prepare(
            "INSERT INTO lokacija (adresa, id_grad) VALUES (?, ?)",
        )?;

        stmt.execute((
            &lokacija.adresa,
            &lokacija.id_grad,
        ))?;

        println!(
            "Unešen novi zapis u tablicu lokacija: {}, {}",
            lokacija.adresa,
            lokacija.id_grad,
        );

        Ok(())
    }

    pub async fn update_lokacija(&self, id: i32, adresa: String, id_grad: i32) -> Result<()> {
        let conn = self.conn.lock().await;
                
        let adresa_clone = adresa.clone();
        let id_grad_clone = id_grad.clone();

        let mut stmt = conn.prepare("UPDATE lokacija SET adresa = ?, id_grad = ? WHERE id = ?")?;
        
        stmt.execute((adresa, id_grad, id))?;
    
        println!("Ažuriran zapis u tablici grad: id={}, adresa={}, id_grad={}", id, adresa_clone, id_grad_clone);
    
        Ok(())
    }

    pub async fn delete_lokacija(&self, id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
    
        let mut stmt = conn.prepare("DELETE FROM lokacija WHERE id = ?")?;
    
        stmt.execute((id,))?;
    
        println!("Obrisan zapis iz tablice grad: id={}", id);
    
        Ok(())
    }  


    /* Tablica dogadaj_organizator */
    pub async fn get_dogadaj_organizator_values(&self) -> Result<Vec<DogadajOrganizator>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT uloga_organizatora, komentar, id_dogadaj, id_organizator, id_lokacija FROM dogadaj_organizator")?;

        let dogadaji_organizatori = stmt
            .query_map([], |row| {
                Ok(DogadajOrganizator {
                    uloga: row.get(0)?,
                    komentar: row.get(1)?,
                    id_dogadaj: row.get(2)?,
                    id_organizator: row.get(3)?,
                    id_lokacija: row.get(4)?
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(dogadaji_organizatori)
    }
    
    pub async fn create_dogadaj_organizator(&self, dogadaj_organizator: DogadajOrganizator) -> Result<()> {
        let conn = self.conn.lock().await;

        let mut stmt = conn.prepare(
            "INSERT INTO dogadaj_organizator (uloga_organizatora, komentar, id_dogadaj, id_organizator, id_lokacija) VALUES (?, ?, ?, ?, ?)",
        )?;

        stmt.execute((
            &dogadaj_organizator.uloga,
            &dogadaj_organizator.komentar,
            &dogadaj_organizator.id_dogadaj,
            &dogadaj_organizator.id_organizator,
            &dogadaj_organizator.id_lokacija,
        ))?;

        println!(
            "Unešen novi zapis u tablicu lokacija: {}, {}, {}, {}, {}",
            dogadaj_organizator.uloga,
            dogadaj_organizator.komentar,
            dogadaj_organizator.id_dogadaj,
            dogadaj_organizator.id_organizator,
            dogadaj_organizator.id_lokacija,
        );

        Ok(())
    }

    pub async fn update_dogadaj_organizator(
        &self, 
        id_dogadaj: i32, 
        id_organizator: i32,
        id_lokacija: i32,
        uloga: String,
        komentar: String,
    ) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE dogadaj_organizator SET uloga = ?, komentar = ? WHERE id_dogadaj = ? AND id_organizator = ? AND id_lokacija = ?",
            (uloga, komentar, id_dogadaj, id_organizator, id_lokacija),
        )?;
        Ok(())
    }
    
    pub async fn delete_dogadaj_organizator(&self, id_dogadaj: i32, id_organizator: i32, id_lokacija: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "DELETE FROM dogadaj_organizator WHERE id_dogadaj = ? AND id_organizator = ? AND id_lokacija = ?",
            (id_dogadaj, id_organizator, id_lokacija),
        )?;
        Ok(())
    }
    
    /* Tablica volonter_vjestina */
    pub async fn get_volonter_vjestina_values(&self) -> Result<Vec<VolonterVjestina>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT razina_vjestine, id_volonter, id_vjestina FROM volonter_vjestina")?;

        let volonteri_vjestine = stmt
            .query_map([], |row| {
                Ok(VolonterVjestina {
                    razina: row.get(0)?,
                    id_volonter: row.get(1)?,
                    id_vjestina: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(volonteri_vjestine)
    }
    
    pub async fn create_volonter_vjestina(&self, volonter_vjestina: VolonterVjestina) -> Result<()> {
        let conn = self.conn.lock().await;

        let mut stmt = conn.prepare(
            "INSERT INTO volonter_vjestina (razina_vjestine, id_volonter, id_vjestina) VALUES (?, ?, ?)",
        )?;

        stmt.execute((
            &volonter_vjestina.razina,
            &volonter_vjestina.id_volonter,
            &volonter_vjestina.id_vjestina,
        ))?;

        println!(
            "Unešen novi zapis u tablicu lokacija: {}, {}, {}",
            volonter_vjestina.razina,
            volonter_vjestina.id_volonter,
            volonter_vjestina.id_vjestina,
        );

        Ok(())
    }

    /* Tablica volonter_dogadaj */
    pub async fn get_volonter_dogadaj_values(&self) -> Result<Vec<VolonterDogadaj>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT broj_sati, status, id_volonter, id_dogadaj FROM volonter_dogadaj")?;

        let volotneri_dogadaji = stmt
            .query_map([], |row| {
                Ok(VolonterDogadaj {
                    broj_sati: row.get(0)?,
                    status: row.get(1)?,
                    id_volonter: row.get(2)?,
                    id_dogadaj: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(volotneri_dogadaji)
    }
    
    pub async fn create_volonter_dogadaj(&self, volonter_dogadaj: VolonterDogadaj) -> Result<()> {
        let conn = self.conn.lock().await;

        let mut stmt = conn.prepare(
            "INSERT INTO volonter_dogadaj (id_volonter, id_dogadaj, broj_sati, status) VALUES (?, ?, ?, ?)",
        )?;
        
        stmt.execute((
            &volonter_dogadaj.id_volonter,
            &volonter_dogadaj.id_dogadaj,
            &volonter_dogadaj.broj_sati,
            &volonter_dogadaj.status,
        ))?;

        println!(
            "Unešen novi zapis u tablicu lokacija: {}, {}, {}, {}",
            volonter_dogadaj.id_volonter,
            volonter_dogadaj.id_dogadaj,
            volonter_dogadaj.broj_sati,
            volonter_dogadaj.status
        );

        Ok(())
    }

    /* Tablica povratna_informacija */
    pub async fn get_povratna_informacija_values(&self) -> Result<Vec<PovratnaInformacija>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT ocjena, komentar, datum, id_volonter, id_dogadaj FROM povratna_informacija")?;

        let volotneri_dogadaji = stmt
            .query_map([], |row| {
                Ok(PovratnaInformacija {
                    ocjena: row.get(0)?,
                    komentar: row.get(1)?,
                    datum: row.get(2)?,
                    id_volonter: row.get(3)?,
                    id_dogadaj: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(volotneri_dogadaji)
    }
    
    pub async fn create_povratna_informacija(
        &self,
        povratna_informacija: NewPovratnaInformacija,
    ) -> Result<()> {
        let sql = "INSERT INTO povratna_informacija (ocjena, komentar, id_volonter, id_dogadaj) VALUES (?, ?, ?, ?)";
    
        // Debug SQL query and values
        println!("Executing SQL: {}", sql);
        println!(
            "With values: ocjena = {}, komentar = {}, id_volonter = {}, id_dogadaj = {}",
            povratna_informacija.ocjena,
            povratna_informacija.komentar,
            povratna_informacija.id_volonter,
            povratna_informacija.id_dogadaj,
        );
    
        // Use the connection within a limited scope
        {
            let conn = self.conn.lock().await;
    
            // Prepare the query
            let mut stmt = match conn.prepare(sql) {
                Ok(stmt) => stmt,
                Err(e) => {
                    println!("Error preparing query: {}", e);
                    return Err(e);
                }
            };
    
            // Execute the query
            match stmt.execute((
                &povratna_informacija.ocjena,
                &povratna_informacija.komentar,
                &povratna_informacija.id_volonter,
                &povratna_informacija.id_dogadaj,
            )) {
                Ok(rows_affected) => {
                    println!("Query executed successfully. Rows affected: {}", rows_affected);
                    Ok(())
                }
                Err(e) => {
                    println!("Error executing query: {}", e);
                    Err(e)
                }
            }
        }
    }
}