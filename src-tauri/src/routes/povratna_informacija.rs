use axum::{Json, extract::{State, Path}, http::StatusCode};
use crate::database::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub struct PovratnaInformacija {
    pub ocjena: i32,
    pub komentar: String,
    pub datum: String,
    pub id_volonter: i32,
    pub id_dogadaj: i32,
}

#[derive(Deserialize)]
pub struct NewPovratnaInformacija {
    pub ocjena: i32,
    pub komentar: String,
    pub id_volonter: i32,
    pub id_dogadaj: i32,
}


pub async fn get_povratna_informacija(State(db): State<Arc<Database>>) -> Result<Json<Vec<PovratnaInformacija>>, StatusCode> {
    match db.get_povratna_informacija_values().await {
        Ok(povratne_informacije) => {
            println!("Fetched povratna_informacija: {:?}", povratne_informacije); //Log fetch
            Ok(Json(povratne_informacije))
        }
        Err(err) => {
            eprintln!("Error fetching povratna_informacija: {:?}", err); // Log error
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


pub async fn post_povratna_informacija(
    State(db): State<Arc<Database>>,
    Json(new_povratna_informacija): Json<NewPovratnaInformacija>,
) -> StatusCode {
    match db.create_povratna_informacija(new_povratna_informacija).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn put_povratna_informacija(
    State(db): State<Arc<Database>>,
    Path((id_volonter, id_dogadaj)): Path<(i32, i32)>,
    Json(new_inf): Json<PovratnaInformacija>,
) -> Result<(), StatusCode> {
    db.update_povratna_informacija(id_volonter, id_dogadaj, new_inf.ocjena, new_inf.komentar)
        .await
        .map_err(|err| {
            eprintln!("Error updating volonter_dogadaj: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn delete_povratna_informacija(
    State(db): State<Arc<Database>>,
    Path((id_volonter, id_dogadaj)): Path<(i32, i32)>,
) -> Result<(), StatusCode> {
    db.delete_povratna_informacija(id_volonter, id_dogadaj)
        .await
        .map_err(|err| {
            eprintln!("Error deleting povratna_informacija: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
