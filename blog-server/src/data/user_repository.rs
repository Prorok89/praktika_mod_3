use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::user::User;

pub async fn create_user(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
    let p = false;

    let insert = sqlx::query(
        r#"insert into users 
            (id, username, email, password_hash, created_at)
            values
            ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(user.id)
    .bind(&user.username)
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(user.created_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_user(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {

    _ = sqlx::query(r#"
        update users
            set username = $1,
                email = $2,
                password_hash = $3
    "#)
    .bind(&user.username)
    .bind(&user.email)
    .bind(&user.password_hash)
    .execute(pool)
    .await?
    ;
    
    Ok(())
}

pub async fn delete_user(pool: &PgPool, id : Uuid) -> Result<(), sqlx::Error> {
    _ = sqlx::query(
        r#"
            delete form users
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

pub async fn find_user_by_id(pool: &PgPool, id : Uuid) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "select id, username, email, password_hash, created_at from users where id = $1",
        id
    )
    .fetch_one(pool)
    .await?
    ;
    println!("{:?}", user);
    Ok(user)
}

pub async fn find_user_by_email(pool: &PgPool, email : String) -> Result<User, sqlx::Error> {
   let user = sqlx::query_as!(
        User,
        "select id, username, email, password_hash, created_at from users where email = $1",
        email
    )
    .fetch_one(pool)
    .await?
    ;
    Ok(user)
}