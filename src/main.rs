mod database;
use axum::{Json, Router, extract::State, routing::get};
use database::connect_db::connect_db;
use dotenvy::dotenv;
use futures::stream::TryStreamExt;
use mongodb::{Client, bson::doc};
use serde_json::json;
use std::env;
use tokio::net::TcpListener;

async fn hello() -> &'static str {
    "Hello from Axum!"
}

pub async fn get_users(State(db): State<Client>) -> Json<serde_json::Value> {
    let collection = db
        .database("veodb")
        .collection::<mongodb::bson::Document>("posts");

    let mut cursor = collection.find(doc! {}).await.unwrap();
    let mut items = Vec::new();

    while let Some(doc) = cursor.try_next().await.unwrap() {
        items.push(doc);
    }

    Json(json!({
        "data": items
    }))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = connect_db().await.expect("DB gagal konek");
    let port = env::var("PORT").unwrap_or("8080".into());
    let addr = format!("0.0.0.0:{port}");

    println!("Server on http://{addr}");

    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/api/users", get(get_users))
        .with_state(db); // inject DB

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
