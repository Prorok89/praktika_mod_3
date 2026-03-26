use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_pool(url : &String) -> Result<PgPool, sqlx::Error>
{
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(url)
        .await?
    ;

    Ok(pool)
}


pub async fn migration(poll : &PgPool) ->Result<(), sqlx::Error>{
    sqlx::migrate!("./migrations").run(poll).await?;

    Ok(())
}