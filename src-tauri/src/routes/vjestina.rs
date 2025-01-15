use axum::{Json, extract::State, http::StatusCode};
use crate::database::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub struct Vjestina {
    pub id: i32,
    pub naziv: String,
    pub opis: String,
}

#[derive(Deserialize)]
pub struct NewVjestina {
    pub naziv: String,
    pub opis: String,
}

pub async fn get_vjestina(State(db): State<Arc<Database>>) -> Result<Json<Vec<Vjestina>>, StatusCode> {
    match db.get_vjestina_values().await {
        Ok(vjestine) => {
            println!("Fetched vjestine: {:?}", vjestine); //Log fetch
            Ok(Json(vjestine))
        }
        Err(err) => {
            eprintln!("Error fetching vjestine: {:?}", err); // Log error
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


pub async fn post_vjestina(
    State(db): State<Arc<Database>>,
    Json(new_vjestina): Json<NewVjestina>,
) -> StatusCode {
    match db.create_vjestina(new_vjestina).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
