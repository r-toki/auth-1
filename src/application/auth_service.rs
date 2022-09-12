use crate::domain::user::{User, UserRepository};
use crate::lib::jwt::{create_tokens, Claims, Tokens};
use crate::lib::password_hashing::{hash, verify};
use std::sync::Arc;

#[derive(Debug)]
pub struct SignUpInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct SingInInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct SignOutInput {
    pub claims: Claims,
}

#[derive(Debug)]
pub struct RefreshInput {
    pub token: String,
    pub claims: Claims,
}

#[derive(Debug, Clone)]
pub struct AuthService<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> AuthService<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn sign_up(&self, input: SignUpInput) -> anyhow::Result<Tokens> {
        let hashed_password = hash(&input.password)?;
        let mut user = User::new(&input.email, &hashed_password)?;

        let tokens = create_tokens(&user.id, &user.email)?;
        let hashed_refresh_token = hash(&tokens.refresh_token)?;
        user.set_hashed_refresh_token(Some(hashed_refresh_token))?;

        self.user_repository.insert(user).await?;

        Ok(tokens)
    }

    pub fn sign_in(&self) {
        todo!()
    }

    pub fn sign_out(&self) {
        todo!()
    }

    pub fn refresh(&self) {
        todo!()
    }

    pub fn delete_user(&self) {
        todo!()
    }
}
