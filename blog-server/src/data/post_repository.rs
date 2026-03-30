use sqlx::PgPool;

use crate::domain::post::Post;

pub async fn create_post(pool: &PgPool, post: &Post) -> Result<(), sqlx::Error> {
    _ = sqlx::query(
        r#"insert into posts 
            (title, content, author_id, created_at, updated_at)
            values
            ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(&post.title)
    .bind(&post.content)
    .bind(&post.author_id)
    .bind(post.created_at)
    .bind(post.updated_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async  fn update_post(pool: &PgPool, post: &Post) -> Result<(), sqlx::Error> {
       _ = sqlx::query(
        r#"
            update posts 
                set title = $1,
                    content = $2, 
                    author_id = &3, 
                    updated_at = &4
                where id = $5
        "#,
    )
    .bind(&post.title)
    .bind(&post.content)
    .bind(&post.author_id)
    .bind(post.updated_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_post(pool: &PgPool, id : i64) ->Result<(), sqlx::Error> {
        _ = sqlx::query(
        r#"
            delete form posts
            where
                id = $1
        "#
    )
    .bind(id)
    .execute(pool)
    .await?
    ;
    Ok(())
}

pub async fn find_post_by_id(pool: &PgPool, id: i64) -> Result<Post, sqlx::Error> {
    let post = sqlx::query_as!(
        Post,
        "select id, title, content, author_id, created_at, updated_at from posts where id = $1",
        id
    )
    .fetch_one(pool)
    .await?
    ;

    Ok(post)
}
