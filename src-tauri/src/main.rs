#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

use axum::{Router, routing::get};
use database::Database;
use std::net::SocketAddr;
use tokio::task;

async fn drzava_api() -> &'static str {

}

async fn organizator_api() -> &'static str {
}

async fn dogadaj_api() -> &'static str {
}

async fn volonter_api() -> &'static str {
}

async fn vjestina_api() -> &'static str {
}

async fn grad_api() -> &'static str {
}

async fn lokacija_api() -> &'static str {
}

async fn dogadaj_organizator_api() -> &'static str {
}

async fn volonter_dogadaj_api() -> &'static str {
}

async fn volonter_vjestina_api() -> &'static str {
}

async fn povratna_informacija_api() -> &'static str {
}

#[tokio::main]
async fn main() {
    // Inicijalizacija backend servera na portu 127.0.0.1:3000
    let backend_task = task::spawn(async {
        let app = Router::new()
        .route("/api/drzava", get(drzava_api))
        .route("/api/organizator", get(organizator_api))
        .route ("/api/dogadaj", get(dogadaj_api))
        .route ("/api/volonter", get(volonter_api))
        .route ("/api/vjestina", get(vjestina_api))
        .route ("/api/grad", get(grad_api))
        .route ("api/lokacija", get(lokacija_api))
        .route ("api/dogadaj-organizator", get(dogadaj_organizator_api))
        .route ("api/volonter-dogadaj", get(volonter_dogadaj_api))
        .route ("api/volonter-vjestina", get(volonter_vjestina_api))
        .route ("api/povratna-informacija", get(povratna_informacija_api));


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
