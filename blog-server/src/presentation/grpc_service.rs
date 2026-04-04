use sqlx::PgPool;
use tonic::{Request, Response, Status};
use crate::application::{auth_service::AuthService, blog_service::BlogService};
use crate::domain::error::BlogError;
use crate::domain::user::{FormAuth, FormReg, verify_password};
use crate::domain::post::PostCreateOrUpdate;
use crate::infrastructure::jwt::JwtService;
use crate::presentation::middleware::AuthenticatedUser;

tonic::include_proto!("blog");

use self::blog_service_server::BlogService as BlogServiceTrait;


pub struct BlogGrpcService {
    pool: PgPool,
    auth_service: AuthService,
    blog_service: BlogService,
    jwt_service: JwtService,
}

impl Clone for BlogGrpcService {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            auth_service: self.auth_service.clone(),
            blog_service: self.blog_service.clone(),
            jwt_service: self.jwt_service.clone(),
        }
    }
}

impl BlogGrpcService {
    pub fn new(pool: PgPool, jwt_secret: &str) -> Self {
        Self {
            pool,
            auth_service: AuthService::new(),
            blog_service: BlogService::new(),
            jwt_service: JwtService::new(jwt_secret),
        }
    }

    fn extract_bearer_token(metadata: &tonic::metadata::MetadataMap) -> Result<String, Status> {
        let auth_value: &tonic::metadata::MetadataValue<tonic::metadata::Ascii> = metadata
            .get("authorization")
            .ok_or_else(|| Status::unauthenticated("Authorization header missing"))?;

        let auth_str = auth_value
            .to_str()
            .map_err(|_| Status::unauthenticated("Invalid authorization header"))?;

        auth_str.strip_prefix("Bearer ")
            .ok_or_else(|| Status::unauthenticated("Invalid authorization header format"))
            .map(String::from)
    }

    fn verify_token(&self, token: &str) -> Result<crate::infrastructure::jwt::Claims, Status> {
        self.jwt_service
            .verify_token(token)
            .map_err(|_| Status::unauthenticated("Invalid or expired token"))
    }

    fn post_to_proto(post: &crate::domain::post::Post) -> Post {
        Post {
            id: post.id.unwrap_or(0),
            title: post.title.clone(),
            content: post.content.clone(),
            author_id: post.author_id,
            created_at: post.created_at.timestamp(),
            updated_at: post.updated_at.timestamp(),
        }
    }

    fn user_to_auth_user(user: &crate::domain::user::User) -> AuthUser {
        AuthUser {
            username: user.username.clone(),
            email: user.email.clone(),
        }
    }

    fn map_error(err: BlogError) -> Status {
        match err {
            BlogError::UserAlreadyExists => Status::already_exists("User already exists"),
            BlogError::UserNotFound => Status::unauthenticated("User not found"),
            BlogError::InvalidCredentials => Status::unauthenticated("Invalid credentials"),
            BlogError::PostNotFound(_) => Status::not_found("Post not found"),
            BlogError::Forbidden => Status::permission_denied("Forbidden"),
            _ => Status::internal(err.to_string()),
        }
    }
}

#[tonic::async_trait]
impl BlogServiceTrait for BlogGrpcService {

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();

        let form_reg = FormReg {
            username: req.username,
            email: req.email,
            password: req.password,
        };

		if form_reg.username.is_empty() || form_reg.password.is_empty() || form_reg.email.is_empty() {
			return Err(Status::invalid_argument("The fields must be filled in"));	
		}

        let user = self.auth_service
            .create_user(&form_reg, &self.pool)
            .await
            .map_err(Self::map_error)?;

        let token = self.jwt_service
            .generate_token(user.id.unwrap_or(0), user.username.clone())
            .map_err(|e| Status::internal(e.to_string()))?;

        let auth_user = Self::user_to_auth_user(&user);

        Ok(Response::new(AuthResponse {
            token,
            user: Some(auth_user),
        }))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();

        let form_auth = FormAuth {
            username: req.username,
            password: req.password,
        };

		if form_auth.username.is_empty() || form_auth.password.is_empty() {
			return Err(Status::invalid_argument("The fields must be filled in"));
		}

        let user = self.auth_service
            .login_user(&form_auth, &self.pool)
            .await
            .map_err(|_| Status::unauthenticated("User not found"))?;

        let password_valid = verify_password(&form_auth.password, &user.password_hash)
            .map_err(|e| Status::internal(e.to_string()))?;

        if !password_valid {
            return Err(Status::unauthenticated("Invalid credentials"));
        }

        let token = self.jwt_service
            .generate_token(user.id.unwrap_or(0), user.username.clone())
            .map_err(|e| Status::internal(e.to_string()))?;

        let auth_user = Self::user_to_auth_user(&user);

        Ok(Response::new(AuthResponse {
            token,
            user: Some(auth_user),
        }))
    }

    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<Post>, Status> {
        let metadata = request.metadata();
        let token = Self::extract_bearer_token(metadata)?;
        let claims = self.verify_token(&token)?;

        let req = request.into_inner();
        let post_data = PostCreateOrUpdate {
            title: req.title,
            content: req.content,
        };

        let user = AuthenticatedUser {
            user_id: claims.user_id,
            username: claims.username,
        };

        let post = self.blog_service
            .create_post(&post_data, &user, &self.pool)
            .await
            .map_err(Self::map_error)?;

        Ok(Response::new(Self::post_to_proto(&post)))
    }

    async fn get_post(
        &self,
        request: Request<PostId>,
    ) -> Result<Response<Post>, Status> {
        let req = request.into_inner();
        let post_id = req.id;

        let post = self.blog_service
            .get_post(post_id, &self.pool)
            .await
            .map_err(Self::map_error)?;

        Ok(Response::new(Self::post_to_proto(&post)))
    }

    async fn update_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<Post>, Status> {
        let metadata = request.metadata();
        let token = Self::extract_bearer_token(metadata)?;
        let claims = self.verify_token(&token)?;

        let req = request.into_inner();
        let post_data = PostCreateOrUpdate {
            title: req.title,
            content: req.content,
        };

        let user = AuthenticatedUser {
            user_id: claims.user_id,
            username: claims.username,
        };

        let post = self.blog_service
            .update_post(req.id, &post_data, &user, &self.pool)
            .await
            .map_err(Self::map_error)?;

        Ok(Response::new(Self::post_to_proto(&post)))
    }

    async fn delete_post(
        &self,
        request: Request<PostId>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let metadata = request.metadata();
        let token = Self::extract_bearer_token(metadata)?;
        let claims = self.verify_token(&token)?;

        let req = request.into_inner();
        let post_id = req.id;

        let user = AuthenticatedUser {
            user_id: claims.user_id,
            username: claims.username,
        };

        self.blog_service
            .delete_post(post_id, &user, &self.pool)
            .await
            .map_err(Self::map_error)?;

        Ok(Response::new(EmptyResponse {}))
    }

    
    async fn list_posts(
        &self,
        request: Request<ListParams>,
    ) -> Result<Response<PostResponse>, Status> {
        let req = request.into_inner();
        let mut limit = req.limit;
		if limit == 0 {
			limit = 10;
		}

        let offset = req.offset;

        let (posts, total) = self.blog_service
            .get_posts(limit, offset, &self.pool)
            .await
            .map_err(Self::map_error)?;

        let proto_posts: Vec<Post> = posts.iter().map(Self::post_to_proto).collect();

        Ok(Response::new(PostResponse {
            posts: proto_posts,
            total: total as i64,
            limit,
            offset,
        }))
    }
}
