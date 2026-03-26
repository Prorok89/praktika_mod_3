use sqlx::PgPool;

use crate::domain::user::User;

pub async fn create_user(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
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

/*

pub struct User {
    pub id : u64,
    pub username : String,
    pub email : String,
    pub password_hash : String,
    pub created_at : DateTime<Utc>
}
*/

pub fn update_user() {}

pub fn delete_user() {}

pub fn find_user() {}
