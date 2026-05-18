// models/posts.rs

use chrono::{DateTime, Utc};

use serde::{
    Deserialize,
    Serialize,
};

use sqlx::FromRow;

use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreatePost {
    pub user_id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
}

#[derive(Serialize, FromRow)]
pub struct Posts {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub tags: Vec<String>,
    pub likes: i32,
    pub views: i32,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct PutPost {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}