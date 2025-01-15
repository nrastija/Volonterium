use axum::{Json, extract::State, http::StatusCode};
use crate::database::Database;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub struct Dogadaj {
    pub id: i32,
    pub naziv: String,
    pub datum_vrijeme: String,
    pub opis: String,
    pub potrebni_volonteri: i32,
}

#[derive(Deserialize)]
pub struct NewDogadaj {
    pub naziv: String,
    pub datum_vrijeme: NaiveDateTime,
    pub opis: String,
    pub potrebni_volonteri: i32,
}

pub async fn get_dogadaj(State(db): State<Arc<Database>>) -> Result<Json<Vec<Dogadaj>>, StatusCode> {
    match db.get_dogadaj_values().await {
        Ok(dogadaji) => {
            println!("Fetched dogadaji: {:?}", dogadaji); // Log the fetched data
            Ok(Json(dogadaji))
        }
        Err(err) => {
            eprintln!("Error fetching dogadaji: {:?}", err); // Log the error
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


pub async fn post_dogadaj(
    State(db): State<Arc<Database>>,
    Json(new_dogadaj): Json<NewDogadaj>,
) -> StatusCode {
    match db.create_dogadaj(new_dogadaj).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
