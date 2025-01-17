use axum::{Json, extract::State, http::StatusCode};
use crate::database::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub struct Lokacija {
    pub id: i32,
    pub adresa: String,
    pub id_grad: i32,
}

#[derive(Deserialize)]
pub struct NewLokacija {
    pub adresa: String,
    pub id_grad: i32,
}

pub async fn get_lokacija(State(db): State<Arc<Database>>) -> Result<Json<Vec<Lokacija>>, StatusCode> {
    match db.get_lokacija_values().await {
        Ok(lokacije) => {
            println!("Fetched lokacije: {:?}", lokacije); //Log fetch
            Ok(Json(lokacije))
        }
        Err(err) => {
            eprintln!("Error fetching lokacije: {:?}", err); // Log error
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


pub async fn post_lokacija(
    State(db): State<Arc<Database>>,
    Json(new_lokacija): Json<NewLokacija>,
) -> StatusCode {
    match db.create_lokacija(new_lokacija).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
