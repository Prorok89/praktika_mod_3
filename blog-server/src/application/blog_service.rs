use sqlx::PgPool;

use crate::{
    data::post_repository,
    domain::{
        error::BlogError,
        post::{Post, PostCreateOrUpdate},
    },
    presentation::middleware::AuthenticatedUser,
};

pub struct BlogService;

impl BlogService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_post(
        &self,
        post: &PostCreateOrUpdate,
        user: &AuthenticatedUser,
        pool: &PgPool,
    ) -> Result<Post, BlogError> {
        let mut post_new = Post::new(post.title.clone(), post.content.clone(), user.user_id);

        let post_id = post_repository::create_post(&pool, &post_new).await?;

        post_new.id = Some(post_id);

        Ok(post_new)
    }

    pub async fn update_post(
        &self,
        id: i64,
        post: &PostCreateOrUpdate,
        user: &AuthenticatedUser,
        pool: &PgPool,
    ) -> Result<Post, BlogError> {
        match post_repository::find_post_by_id(&pool, id).await {
            Ok(mut post_bd) => {
                if post_bd.author_id != user.user_id {
                    return Err(BlogError::Forbidden);
                }

                post_bd.title = post.title.clone();
                post_bd.content = post.content.clone();
                post_bd.updated_at = chrono::Utc::now();

                let updated_post = post_repository::update_post(&pool, &post_bd).await?;
                Ok(updated_post)
            }
            Err(e) => Err(BlogError::PostNotFound(e.to_string())),
        }
    }

    pub async fn delete_post(
        &self,
        id: i64,
        user: &AuthenticatedUser,
        pool: &PgPool,
    ) -> Result<(), BlogError> {
        match post_repository::find_post_by_id(&pool, id).await {
            Ok(post_bd) => {
                if post_bd.author_id != user.user_id {
                    return Err(BlogError::Forbidden);
                }

                if let Err(e) = post_repository::delete_post(&pool, id).await {
                    return Err(BlogError::ErrorNotKnow(e.to_string()));
                }

                Ok(())
            }
            Err(e) => Err(BlogError::PostNotFound(e.to_string())),
        }
    }

    pub async fn get_post(&self, id: i64, pool: &PgPool) -> Result<Post, BlogError> {
        let post = post_repository::find_post_by_id(&pool, id).await?;

        Ok(post)
    }
}
