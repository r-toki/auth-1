use crate::domain::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: User) -> anyhow::Result<()>;
    async fn delete(&self, id: &str) -> anyhow::Result<()>;
    async fn find_one(&self, id: &str) -> anyhow::Result<User>;
    async fn find_one_by_email(&self, email: &str) -> anyhow::Result<User>;
}
