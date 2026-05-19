// * routes/posts.rs

use axum::{
    Json, extract::{Path, State}, http::StatusCode
};
use uuid::Uuid;
use sqlx::PgPool;

use crate::models::posts::{
    CreatePost, Posts, PutPost
};

// * Mengambil banyak Postingan
pub async fn getposts(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Posts>>, StatusCode> {

    let posts = sqlx::query_as::<_, Posts>(
        "
        SELECT
            id,
            title,
            slug,
            content,
            tags,
            likes,
            views,
            user_id,
            created_at,
            updated_at
        FROM posts
        "
    )
    .persistent(false)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}

// * Mengambil satu Postingan berdasarkan slug -> SEHARUSNYA! nanti deh diupdate.
// pub async fn getpost(
//     // Path(slug): Path<String>,
//     State(pool): State<PgPool>,
// ) -> Result<Json<Posts>, StatusCode> {

//     let result = sqlx::query_as::<_, Posts>(
//         "
//         SELECT
//             id,
//             title,
//             slug,
//             content,
//             tags,
//             likes,
//             views,
//             user_id,
//             created_at,
//             updated_at
//         FROM posts
//         "
//     )
//     .persistent(false)
//     // .bind(slug)
//     .fetch_one(&pool)
//     .await
//     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     Ok(Json(result))
// }

pub async fn sendpost(
    State(pool): State<PgPool>,
    Json(body): Json<CreatePost>,
) -> Result<Json<Posts>, StatusCode> {

    let send = sqlx::query_as::<_, Posts>(
        "
        INSERT INTO posts (
            user_id,
            title,
            slug,
            content
        )

        VALUES ($1, $2, $3, $4)

        RETURNING
            id,
            title,
            slug,
            content,
            tags,
            likes,
            views,
            user_id,
            created_at,
            updated_at
        "
    )
    .persistent(false)
    .bind(body.user_id)
    .bind(body.title)
    .bind(body.slug)
    .bind(body.content)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(send))
}

pub async fn putpost(
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
    Json(body): Json<PutPost>,
) -> Result<Json<Posts>, StatusCode> {
    let put = sqlx::query_as::<_, Posts>(
        "
        UPDATE posts
        SET
            title = $1,
            slug = %2,
            content = $3,
            tags = $4,
            updated_at = NOW()

        WHERE id = $5

        RETURNING
            id,
            title,
            slug,
            content,
            tags,
            likes,
            views,
            user_id,
            created_at,
            updated_at
        ")
        .persistent(false)
        .bind(body.title)
        .bind(body.slug)
        .bind(body.content)
        .bind(body.tags)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(put))
}

pub async fn deletepost(
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        "
        DELETE FROM posts
        WHERE id = $1
        "
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND)
    };

    Ok(StatusCode::NO_CONTENT)
}