use axum::{Json, extract::State, http::StatusCode};
use crate::database::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct DogadajOrganizator {
    pub uloga: String,
    pub komentar: String,
    pub id_dogadaj: i32,
    pub id_organizator: i32,
    pub id_lokacija: i32,
}


pub async fn get_dogadaj_organizator(State(db): State<Arc<Database>>) -> Result<Json<Vec<DogadajOrganizator>>, StatusCode> {
    match db.get_dogadaj_organizator_values().await {
        Ok(dogadaji_organizatori) => {
            println!("Fetched dogadaj_organizator: {:?}", dogadaji_organizatori); //Log fetch
            Ok(Json(dogadaji_organizatori))
        }
        Err(err) => {
            eprintln!("Error fetching dogadaji: {:?}", err); // Log error
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


pub async fn post_dogadaj(
    State(db): State<Arc<Database>>,
    Json(new_dogadaj_organizator): Json<DogadajOrganizator>,
) -> StatusCode {
    match db.create_dogadaj_organizator(new_dogadaj_organizator).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
