use crate::domain::user::{User, UserRepository};
use crate::lib::jwt::{generate_tokens, Auth, Tokens};
use crate::lib::password_hashing::{hash, verify};
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

    pub async fn sign_in(&self, email: &str, password: &str) -> anyhow::Result<Tokens> {
        let mut user = self.user_repository.find_one_by_email(email).await?;
        verify(password, &user.hashed_password)?;

        let tokens = generate_tokens(&user.id)?;
        let hashed_refresh_token = hash(&tokens.refresh_token)?;
        user.set_hashed_refresh_token(Some(hashed_refresh_token))?;
        self.user_repository.update(user).await?;

        Ok(tokens)
    }

    pub async fn sign_out(&self, auth: Auth) -> anyhow::Result<()> {
        let mut user = self.user_repository.find_one(&auth.user_id).await?;
        user.set_hashed_refresh_token(None)?;
        self.user_repository.update(user).await?;

        Ok(())
    }

    pub async fn refresh(&self, auth: Auth, refresh_token: &str) -> anyhow::Result<Tokens> {
        let mut user = self.user_repository.find_one(&auth.user_id).await?;
        let hashed_refresh_token = user
            .hashed_refresh_token
            .clone()
            .ok_or(anyhow::anyhow!("could not refresh"))?;
        verify(&refresh_token, &hashed_refresh_token)?;

        let tokens = generate_tokens(&user.id)?;
        let hashed_refresh_token = hash(&tokens.refresh_token)?;
        user.set_hashed_refresh_token(Some(hashed_refresh_token))?;
        self.user_repository.update(user).await?;

        Ok(tokens)
    }

    pub async fn delete_user(&self, auth: Auth) -> anyhow::Result<()> {
        self.user_repository.delete(&auth.user_id).await?;

        Ok(())
    }
}
