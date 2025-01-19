use axum::{Json, extract::{State, Path}, http::StatusCode};
use crate::database::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct VolonterDogadaj {
    pub broj_sati: i32,
    pub status: String,
    pub id_volonter: i32,
    pub id_dogadaj: i32,
}


pub async fn get_volonter_dogadaj(State(db): State<Arc<Database>>) -> Result<Json<Vec<VolonterDogadaj>>, StatusCode> {
    match db.get_volonter_dogadaj_values().await {
        Ok(volonteri_dogadaji) => {
            println!("Fetched volonter_dogadaj: {:?}", volonteri_dogadaji); //Log fetch
            Ok(Json(volonteri_dogadaji))
        }
        Err(err) => {
            eprintln!("Error fetching dogadaji: {:?}", err); // Log error
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


pub async fn post_volonter_dogadaj(
    State(db): State<Arc<Database>>,
    Json(new_volonter_dogadaj): Json<VolonterDogadaj>,
) -> StatusCode {
    match db.create_volonter_dogadaj(new_volonter_dogadaj).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn put_volonter_dogadaj(
    State(db): State<Arc<Database>>,
    Path((id_volonter, id_dogadaj)): Path<(i32, i32)>,
    Json(new_dogadaj): Json<VolonterDogadaj>,
) -> Result<(), StatusCode> {
    db.update_volonter_dogadaj(id_volonter, id_dogadaj, new_dogadaj.status, new_dogadaj.broj_sati)
        .await
        .map_err(|err| {
            eprintln!("Error updating volonter_vjestina: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn delete_volonter_dogadaj(
    State(db): State<Arc<Database>>,
    Path((id_volonter, id_dogadaj)): Path<(i32, i32)>,
) -> Result<(), StatusCode> {
    db.delete_volonter_dogadaj(id_volonter, id_dogadaj)
        .await
        .map_err(|err| {
            eprintln!("Error deleting volonter_vjestina: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}