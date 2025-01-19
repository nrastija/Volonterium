use axum::{Json, extract::{State, Path}, http::StatusCode};
use crate::database::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub struct Grad {
    pub id: i32,
    pub naziv: String,
    pub id_drzava: i32,
}

#[derive(Deserialize)]
pub struct NewGrad {
    pub naziv: String,
    pub id_drzava: i32,
}

pub async fn get_grad(State(db): State<Arc<Database>>) -> Result<Json<Vec<Grad>>, StatusCode> {
    match db.get_grad_value().await {
        Ok(gradovi) => {
            println!("Fetched gradovi: {:?}", gradovi); //Log fetch
            Ok(Json(gradovi))
        }
        Err(err) => {
            eprintln!("Error fetching gradovi: {:?}", err); // Log error
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


pub async fn post_grad(
    State(db): State<Arc<Database>>,
    Json(new_grad): Json<NewGrad>,
) -> StatusCode {
    match db.create_grad(new_grad).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn put_grad(
    State(db): State<Arc<Database>>,
    Path(id): Path<i32>,
    Json(updated_grad): Json<NewGrad>,
) -> Result<(), StatusCode> {
    db.update_grad(id, updated_grad.naziv, updated_grad.id_drzava)
        .await
        .map_err(|err| {
            eprintln!("Error updating grad: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn delete_grad(
    State(db): State<Arc<Database>>,
    Path(id): Path<i32>,
) -> Result<(), StatusCode> {
    db.delete_grad(id)
        .await
        .map_err(|err| {
            eprintln!("Error deleting grad: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}