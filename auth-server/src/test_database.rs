use std::time::{Duration, Instant};

pub type UserID = u64;

pub enum PermissionLevel {
    No,
    Normal,
    Privileged,
}

pub struct RegisteredUser {
    id: UserID,
    username: String,
    hashed_password: String,
    permission_level: PermissionLevel,
}

pub enum TokenPrupose {
    Nothing,
    UserAccess(UserID),
}

pub struct Token {
    pub key: String,
    pub prupose: TokenPrupose,
    pub emission_time: Instant,
    pub lifespan: Duration,
    invalidated: bool,
}

impl Token {
    pub fn new(
        key: String,
        prupose: TokenPrupose,
        emission_time: Instant,
        lifespan: Duration,
    ) -> Token {
        Token {
            key,
            prupose,
            lifespan,
            emission_time,
            invalidated: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        let elapsed = self.emission_time.checked_duration_since(Instant::now());

        let is_time_valid = match elapsed {
            Some(elapsed) => elapsed >= Duration::from_secs(0) && elapsed < self.lifespan,
            None => false,
        };

        !self.invalidated_manually() && is_time_valid
    }

    pub fn invalidate(&mut self) {
        self.invalidated = true;
    }

    pub fn invalidated_manually(&self) -> bool {
        self.invalidated
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
                Instant::now(),
                Duration::from_secs(10),
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
                Instant::now() - Duration::from_secs(20),
                Duration::from_secs(10),
            );
            // Should fail because token has expired
            assert!(!invalid_token.is_valid());
        }

        {
            let invalid_token = Token::new(
                String::new(),
                TokenPrupose::Nothing,
                Instant::now() + Duration::from_secs(20),
                Duration::from_secs(10),
            );

            // Should fail because token was emitted in the future
            assert!(!invalid_token.is_valid());
        }
    }
}
