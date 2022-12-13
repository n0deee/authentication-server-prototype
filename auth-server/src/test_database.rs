use std::time::{Duration, Instant};

use serde::{Serialize, Deserialize};

pub type UserID = u64;
pub type TimestampMillis = i64;

#[derive(Serialize, Deserialize)]
pub enum PermissionLevel {
    No,
    Normal,
    Privileged,
}

#[derive(Serialize, Deserialize)]
pub enum TokenPrupose {
    Nothing,
    UserAccess(UserID),
}

#[derive(Serialize, Deserialize)]
pub struct RegisteredUser {
    id: UserID,
    username: String,
    hashed_password: String,
    permission_level: PermissionLevel,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub key: String,
    pub prupose: TokenPrupose,
    pub emission_time_millis: TimestampMillis,
    pub lifespan_millis: TimestampMillis,
    invalidated: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TokenGuard {
    token: Token
}

impl Token {
    pub fn new(
        key: String,
        prupose: TokenPrupose,
        emission_time: TimestampMillis,
        lifespan: TimestampMillis,
    ) -> Token {
        Token {
            key,
            prupose,
            lifespan_millis: lifespan,
            emission_time_millis: emission_time,
            invalidated: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        // let elapsed = self.emission_time_millis.checked_duration_since(Instant::now());

        // let is_time_valid = match elapsed {
        //     Some(elapsed) => elapsed >= Duration::from_secs(0) && elapsed < self.lifespan_millis,
        //     None => false,
        // };
        let elapsed = chrono::Utc::now().timestamp_millis() - self.emission_time_millis;

        let is_time_valid = elapsed >= 0 && elapsed < self.lifespan_millis;
        !self.invalidated_manually() && is_time_valid
    }

    pub fn invalidate(&mut self) {
        self.invalidated = true;
    }

    pub fn invalidated_manually(&self) -> bool {
        self.invalidated
    }
}

impl TokenGuard {
    pub fn new(token: Token) -> TokenGuard {
        TokenGuard { token }
    }

    /// Returns Ok() if the token is valid and Err() if the token is invalid
    pub fn get_token(&self) -> Result<&Token, ()>{
        if !self.token.is_valid() {
            return Err(());
        }

        Ok(&self.token)
    }
}

impl Into<Token> for TokenGuard {
    fn into(self) -> Token {
        self.token
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn token_validation() {
        {
            // Normal valid token
            let mut token = Token::new(
                String::new(),
                TokenPrupose::Nothing,
                chrono::Utc::now().timestamp_millis(),
                10_000,
            );
            assert!(token.is_valid());

            // Manually invalidate token
            token.invalidate();
            // Should fail because token was invalidated
            assert!(!token.is_valid());
        }

        {
            // Time invalid token
            let invalid_token = Token::new(
                String::new(),
                TokenPrupose::Nothing,
                chrono::Utc::now().timestamp_millis() - 20_000,
                10_000,
            );
            // Should fail because token has expired
            assert!(!invalid_token.is_valid());
        }

        {
            let invalid_token = Token::new(
                String::new(),
                TokenPrupose::Nothing,
                chrono::Utc::now().timestamp_millis() + 20_000,
                10_000,
            );

            // Should fail because token was emitted in the future
            assert!(!invalid_token.is_valid());
        }
    }
}
