// models/users.rs

use chrono::{DateTime, Utc};

use serde::{
    Deserialize,
    Serialize,
};

use sqlx::{
    FromRow,
    Type,
};

use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, FromRow)]
pub struct Users {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub avatar: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub identifier: String, //* username atau email */
    pub password: String,
}

#[derive(FromRow)]
pub struct LoginQuery {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}