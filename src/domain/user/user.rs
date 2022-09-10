use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use validator::Validate;

#[derive(Debug)]
pub struct NewInput {
    email: String,
    hashed_password: String,
    hashed_refresh_token: Option<String>,
}

#[derive(Debug)]
pub struct SetHashedRefreshTokenInput {
    hashed_refresh_token: String,
}

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
    pub fn new(input: NewInput) -> anyhow::Result<Self> {
        let now = Utc::now();
        let user = Self {
            id: Ulid::new().to_string(),
            email: input.email,
            hashed_password: input.hashed_password,
            hashed_refresh_token: input.hashed_refresh_token,
            created_at: now,
            updated_at: now,
        };
        user.validate()?;
        Ok(user)
    }

    pub fn set_hashed_refresh_token(
        &mut self,
        input: SetHashedRefreshTokenInput,
    ) -> anyhow::Result<()> {
        self.hashed_refresh_token = Some(input.hashed_refresh_token);
        self.validate()?;
        Ok(())
    }

    pub fn unset_hashed_refresh_token(&mut self) {
        self.hashed_refresh_token = None;
    }
}
