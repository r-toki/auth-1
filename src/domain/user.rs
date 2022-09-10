
use anyhow::Result;
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
    pub fn new(
        email: &str,
        hashed_password: &str,
        hashed_refresh_token: Option<&str>,
    ) -> Result<Self> {
        let now = Utc::now();
        let user = Self  {
            id: Ulid::new().to_string(),
            email: email.to_string(),
            hashed_password: hashed_password.to_string(),
            hashed_refresh_token: hashed_refresh_token.map(|hrt| hrt.to_string()),
            created_at: now,
            updated_at: now,
        };
        user.validate()?;
        Ok(user)
    }
}
