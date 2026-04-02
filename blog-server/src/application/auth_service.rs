use sqlx::PgPool;

use crate::{data::user_repository, domain::{error::BlogError, user::{FormReg, User}}};

pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_user(&self, user : &FormReg, pool: &PgPool) -> Result<User, BlogError> {
        
        if let Ok(_) = user_repository::find_user_by_email(pool, user.email.clone()).await{
            return Err(BlogError::UserAlreadyExists);
        }
        
        let mut user_new = User::new(&user.email, &user.password, &user.username)?;

        let user_id = user_repository::create_user(&pool, &user_new).await?;

		user_new.id = Some(user_id);

        Ok(user_new)

    }
}