use sqlx::PgPool;

use crate::domain::{error::BlogError, post::Post};

pub async fn create_post(pool: &PgPool, post: &Post) -> Result<i64, BlogError> {
    let id = sqlx::query_scalar!(
        r#"insert into posts 
            (title, content, author_id, created_at, updated_at)
            values
            ($1, $2, $3, $4, $5)
			returning id
        "#,
        &post.title,
        &post.content,
        &post.author_id,
        post.created_at,
        post.updated_at,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| BlogError::SqlError(e.to_string()))?;
    Ok(id)
}

pub async fn update_post(pool: &PgPool, post: &Post) -> Result<Post, BlogError> {
    let post_id = post.id.ok_or(BlogError::PostNotFound("Post ID is required".to_string()))?;

    let updated_post = sqlx::query_as!(
        Post,
        r#"
            update posts
                set title = $1,
                    content = $2,
                    author_id = $3,
                    updated_at = $4
                where id = $5
            returning id, title, content, author_id, created_at, updated_at
        "#,
        &post.title,
        &post.content,
        &post.author_id,
        post.updated_at,
        post_id as i64
    )
    .fetch_one(pool)
    .await
    .map_err(|e| BlogError::SqlError(e.to_string()))?;

    Ok(updated_post)
}

pub async fn delete_post(pool: &PgPool, id: i64) -> Result<(), BlogError> {
    _ = sqlx::query(
        r#"
            delete from posts
            where
                id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| BlogError::SqlError(e.to_string()))?;
    Ok(())
}

pub async fn find_post_by_id(pool: &PgPool, id: i64) -> Result<Post, BlogError> {
    let post = sqlx::query_as!(
        Post,
        "select id, title, content, author_id, created_at, updated_at from posts where id = $1",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| BlogError::SqlError(e.to_string()))?;

    Ok(post)
}
