use axum::{Json, extract::State, http::StatusCode};
use crate::database::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct VolonterVjestina {
    pub razina: String,
    pub id_volonter: i32,
    pub id_vjestina: i32,
}


pub async fn get_volonter_vjestina(State(db): State<Arc<Database>>) -> Result<Json<Vec<VolonterVjestina>>, StatusCode> {
    match db.get_volonter_vjestina_values().await {
        Ok(volonteri_vjestine) => {
            println!("Fetched volonter_vjestina: {:?}", volonteri_vjestine); //Log fetch
            Ok(Json(volonteri_vjestine))
        }
        Err(err) => {
            eprintln!("Error fetching dogadaji: {:?}", err); // Log error
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


pub async fn post_dogadaj(
    State(db): State<Arc<Database>>,
    Json(new_volonter_vjestina): Json<VolonterVjestina>,
) -> StatusCode {
    match db.create_volontetr_vjestina(new_volonter_vjestina).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
