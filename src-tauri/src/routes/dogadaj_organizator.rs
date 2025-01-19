use axum::{Json, extract::{State, Path}, http::StatusCode};
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


pub async fn put_dogadaj_organizator(
    State(db): State<Arc<Database>>,
    Path((id_dogadaj, id_organizator, id_lokacija)): Path<(i32, i32, i32)>,
    Json(updated_data): Json<DogadajOrganizator>,
) -> Result<(), StatusCode> {
    db.update_dogadaj_organizator(id_dogadaj, id_organizator, id_lokacija, updated_data.uloga, updated_data.komentar)
        .await
        .map_err(|err| {
            eprintln!("Error updating dogadaj_organizator: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn delete_dogadaj_organizator(
    State(db): State<Arc<Database>>,
    Path((id_dogadaj, id_organizator, id_lokacija)): Path<(i32, i32, i32)>,
) -> Result<(), StatusCode> {
    db.delete_dogadaj_organizator(id_dogadaj, id_organizator, id_lokacija)
        .await
        .map_err(|err| {
            eprintln!("Error deleting dogadaj_organizator: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
