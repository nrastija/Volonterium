use axum::{Json, extract::State, http::StatusCode};
use crate::database::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub struct Organizator {
    pub id: i32,
    pub naziv: String,
    pub kontakt_osoba: String,
    pub kontakt_telefon: String,
    pub mail: String,
}

#[derive(Deserialize)]
pub struct NewOrganizator {
    pub naziv: String,
    pub kontakt_osoba: String,
    pub telefon: String,
    pub mail: String,
}

pub async fn get_organizator(State(db): State<Arc<Database>>) -> Result<Json<Vec<Organizator>>, StatusCode> {
    match db.get_organizator_values().await {
        Ok(organizatori) => {
            println!("Fetched organizatori: {:?}", organizatori);
            Ok(Json(organizatori))
        }
        Err(err) => {
            eprintln!("Error fetching organizatori: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn post_organizator(
    State(db): State<Arc<Database>>,
    Json(new_organizator): Json<NewOrganizator>,
) -> StatusCode {
    match db.create_organizator(new_organizator).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
