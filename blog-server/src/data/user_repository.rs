use sqlx::PgPool;

use crate::domain::{error::BlogError, user::User};

pub async fn create_user(pool: &PgPool, user: &User) -> Result<i64, BlogError> {
    let id = sqlx::query_scalar!(
        r#"insert into users
            (username, email, password_hash, created_at)
            values
            ($1, $2, $3, $4)
            returning id
        "#,
        user.username,
        user.email,
        user.password_hash,
        user.created_at
    )
    .fetch_one(pool)
    .await
    .map_err(|e| BlogError::SqlError(e.to_string()))?;

    Ok(id)
}

pub async fn update_user(pool: &PgPool, user: &User) -> Result<(), BlogError> {
    _ = sqlx::query(
        r#"
        update users
            set username = $1,
                email = $2,
                password_hash = $3
            where id = &4
    "#,
    )
    .bind(&user.username)
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(&user.created_at)
    .execute(pool)
    .await
    .map_err(|e| BlogError::SqlError(e.to_string()))?;

    Ok(())
}

pub async fn delete_user(pool: &PgPool, id: i64) -> Result<(), BlogError> {
    _ = sqlx::query(
        r#"
            delete form users
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

pub async fn find_user_by_id(pool: &PgPool, id: i64) -> Result<User, BlogError> {
    let user = sqlx::query_as!(
        User,
        "select id, username, email, password_hash, created_at from users where id = $1",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| BlogError::SqlError(e.to_string()))?;

    Ok(user)
}

pub async fn find_user_by_email(pool: &PgPool, email: String) -> Result<User, BlogError> {
    let user = sqlx::query_as!(
        User,
        "select id, username, email, password_hash, created_at from users where email = $1",
        email
    )
    .fetch_one(pool)
    .await
    .map_err(|e| BlogError::SqlError(e.to_string()))?;
    Ok(user)
}
