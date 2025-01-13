#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

use axum::{Router, routing::get};
use database::Database;
use std::net::SocketAddr;
use tokio::task;

async fn health_check() -> &'static str {
    "Backend is running!"
}

#[tokio::main]
async fn main() {
    // Inicijalizacija backend servera na portu 127.0.0.1:3000
    let backend_task = task::spawn(async {
        let app = Router::new().route("/api/health", get(health_check));

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        println!("Starting backend server at http://{}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    //Inicijalizacija baze podataka
    let db = Database::new("src/database/volonteri.db")
        .expect("Failed to connect to the database");

    match db.get_table_names() {
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

    volonterium_lib::run();

    backend_task.await.unwrap();
}
