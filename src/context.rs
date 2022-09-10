use crate::application::auth_service::AuthService;
use crate::infra::user_repository_impl::UserRepositoryImpl;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Context {
    pub auth_service: AuthService<UserRepositoryImpl>,
}

impl Context {
    pub fn new(pool: PgPool) -> Self {
        let pool = Arc::new(pool);

        let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone()));

        let auth_service = AuthService::new(user_repository.clone());

        Context { auth_service }
    }
}
