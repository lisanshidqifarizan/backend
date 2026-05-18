use axum::{
    routing::{get, post,put},
    Router,
};

use tokio::net::TcpListener;

// Folder Lokal
mod routes;
mod database;
mod models;

use database::connect_supa::connect_db;

use routes::{
    posts::{getposts, sendpost, /*getpost,*/ putpost, deletepost},
    users::{getusers, senduser},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let pool = connect_db().await?;

    let app = Router::new()
        .route("/register", post(senduser)) //* Daftar user */
        .route("/admin/users", get(getusers)) //* Ambil semua user */
        .route("/posts", get(getposts).post(sendpost)) //* Ambil semua dan Kirim postingan */
        // .route("/posts/{slug}",get(getpost)) //* Buka postingan 1 page dengan slug */
        .route("/posts/e/{id}",put(putpost).delete(deletepost)) //* Edit dan Delete by post */
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
