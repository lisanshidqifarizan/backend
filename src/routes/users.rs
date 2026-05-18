// routes/users.rs

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use sqlx::PgPool;

use crate::models::users::{
    CreateUser,
    Users,
};

pub async fn getusers(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Users>>, StatusCode> {

    let users = sqlx::query_as::<_, Users>(
        "
        SELECT
            id,
            name,
            username,
            email,
            role,
            created_at,
            updated_at,
            avatar
        FROM users
        "
    )
    .persistent(false)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

pub async fn senduser(
    State(pool): State<PgPool>,
    Json(body): Json<CreateUser>,
) -> Result<Json<Users>, StatusCode> {

    let user_exist: Option<String> = sqlx::query_scalar(
    "
    SELECT username
    FROM users
    WHERE username = $1 OR email = $2
    "
    )
    .bind(&body.username)
    .bind(&body.email)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if user_exist.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let send = sqlx::query_as::<_, Users>(
        "
        INSERT INTO users (
            name,
            username,
            email,
            password
        )

        VALUES ($1, $2, $3, $4)

        RETURNING
            id,
            name,
            username,
            email,
            role,
            created_at,
            updated_at,
            avatar
        "
    )
    .persistent(false)
    .bind(body.name)
    .bind(body.username)
    .bind(body.email)
    .bind(body.password)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(send))
}