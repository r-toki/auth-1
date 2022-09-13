use crate::application::auth_service::AuthService;
use crate::infrastructure::user_repository_impl::UserRepositoryImpl;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Modules {
    pub auth_service: AuthService<UserRepositoryImpl>,
}

impl Modules {
    pub fn new(pool: PgPool) -> Self {
        let pool = Arc::new(pool);

        let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone()));

        let auth_service = AuthService::new(user_repository.clone());

        Modules { auth_service }
    }
}
