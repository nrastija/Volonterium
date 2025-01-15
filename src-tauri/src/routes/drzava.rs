use axum::{Json, extract::State, http::StatusCode};
use crate::database::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub struct Drzava {
    pub id: i32,
    pub naziv: String,
}

#[derive(Deserialize)]
pub struct NewDrzava {
    pub naziv: String,
}

pub async fn get_drzava(State(db): State<Arc<Database>>) -> Result<Json<Vec<Drzava>>, StatusCode> {
    match db.get_drzava_values().await {
        Ok(drzave) => {
            println!("Fetched drzave: {:?}", drzave); //Log fetch
            Ok(Json(drzave))
        }
        Err(err) => {
            eprintln!("Error fetching drzave: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn post_drzava(
    State(db): State<Arc<Database>>,
    Json(new_drzava): Json<NewDrzava>,
) -> StatusCode {
    match db.create_drzava(new_drzava).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
