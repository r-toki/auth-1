use crate::domain::user::{User, UserRepository};
use crate::lib::jwt::{generate_tokens, Auth, Tokens};
use crate::lib::password_hashing::hash;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AuthService<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> AuthService<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn sign_up(&self, email: &str, password: &str) -> anyhow::Result<Tokens> {
        let hashed_password = hash(&password)?;
        let mut user = User::new(email, &hashed_password)?;

        let tokens = generate_tokens(&user.id)?;
        let hashed_refresh_token = hash(&tokens.refresh_token)?;
        user.set_hashed_refresh_token(Some(hashed_refresh_token))?;

        self.user_repository.insert(user).await?;

        Ok(tokens)
    }

    pub fn sign_in(&self) {}

    pub fn sign_out(&self) {}

    pub fn refresh(&self) {}

    pub async fn delete_user(&self, auth: Auth) -> anyhow::Result<()> {
        self.user_repository.delete(&auth.user_id).await?;

        Ok(())
    }
}
