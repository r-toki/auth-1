use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
    #[validate(length(min = 1))]
    pub id: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1))]
    pub hashed_password: String,

    #[validate(length(min = 1))]
    pub hashed_refresh_token: Option<String>,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: &str, hashed_password: &str) -> anyhow::Result<Self> {
        let now = Utc::now();
        let user = Self {
            id: Ulid::new().to_string(),
            email: email.to_owned(),
            hashed_password: hashed_password.to_owned(),
            hashed_refresh_token: None,
            created_at: now,
            updated_at: now,
        };
        user.validate()?;
        Ok(user)
    }

    pub fn set_hashed_refresh_token(
        &mut self,
        hashed_refresh_token: Option<String>,
    ) -> anyhow::Result<()> {
        self.hashed_refresh_token = hashed_refresh_token;
        self.updated_at = Utc::now();
        self.validate()?;
        Ok(())
    }
}
