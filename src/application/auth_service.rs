use crate::domain::UserRepository;
use crate::lib::jwt::Claims;

#[derive(Debug)]
pub struct SignUpInput {
    email: String,
    password: String,
}

#[derive(Debug)]
pub struct SingInInput {
    email: String,
    password: String,
}

#[derive(Debug)]
pub struct SignOutInput {
    claims: Claims,
}

#[derive(Debug)]
pub struct RefreshInput {
    token: String,
    claims: Claims,
}

#[derive(Debug)]
pub struct Tokens {
    access_token: String,
    refresh_token: String,
}

#[derive(Debug)]
pub struct AuthService<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> AuthService<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub fn sign_up(&self, input: SignUpInput) -> anyhow::Result<Tokens> {
        todo!()
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
