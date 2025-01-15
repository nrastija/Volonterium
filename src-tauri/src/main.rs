#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod routes;

use routes::drzava; use routes::organizator;

use axum::{http::{header, Method}, routing::get, Router};
use database::Database;
use std::sync::Arc;
use std::net::SocketAddr;

use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};

#[tokio::main]
async fn main() {
    // Initialize the database
    let db = Arc::new(
        Database::new("src/database/volonteri.db").expect("Failed to connect to the database"),
    );

    // Print table names
    match db.get_table_names().await {
        Ok(tables) => {
            println!("Tables in the database:");
            for table in tables {
                println!("{}", table);
            }
        }
        Err(err) => {
            eprintln!("Error fetching table names: {}", err);
        }
    }

    // Definiranje ruta i putanja
    let app = Router::new()
        .route("/api/drzava", get(drzava::get_drzava).post(drzava::post_drzava))
        .route("/api/organizator", get(organizator::get_organizator).post(organizator::post_organizator))
        .with_state(db.clone())
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::any())   
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(AllowHeaders::list([header::CONTENT_TYPE]))
        );

    // Pokretanje backend servera (Axum) 
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Starting server at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
