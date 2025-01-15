use axum::{Json, extract::State, http::StatusCode};
use crate::database::Database;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate};
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub struct Volonter {
    pub id: i32,
    pub ime: String,
    pub prezime: String,
    pub mail: String,
    pub telefon: String,
    pub datum_pridruzivanja: String,
}

#[derive(Deserialize)]
pub struct NewVolonter {
    pub ime: String,
    pub prezime: String,
    pub mail: String,
    pub telefon: String,
    pub datum_pridruzivanja: NaiveDate,
}

pub async fn get_volonter(State(db): State<Arc<Database>>) -> Result<Json<Vec<Volonter>>, StatusCode> {
    match db.get_volonter_values().await {
        Ok(volonteri) => {
            println!("Fetched volonteri: {:?}", volonteri); //Log fetch
            Ok(Json(volonteri))
        }
        Err(err) => {
            eprintln!("Error fetching volonteri: {:?}", err); // Log error
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


pub async fn post_volonter(
    State(db): State<Arc<Database>>,
    Json(new_volonter): Json<NewVolonter>,
) -> StatusCode {
    match db.create_volonter(new_volonter).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
