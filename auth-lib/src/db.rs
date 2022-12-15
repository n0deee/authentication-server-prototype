use serde::{Serialize, Deserialize};
use chrono::Utc;

pub type UserID = u64;
pub type TokenKey = String;
pub type TimestampMillis = i64;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TokenPrupose {
    Nothing,
    UserAccess(UserID),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TokenInvalidationReason {
    Manual,
    EmissionAfterTheCurrentDate,
    Expired,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub key: TokenKey,
    pub prupose: TokenPrupose,
    pub emission_time_millis: TimestampMillis,
    pub lifespan_millis: TimestampMillis,
    invalidated: bool,
}
impl Token {
    pub fn new(
        key: TokenKey,
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

    /// Returns None if the token is valid and Some(_) if token is invalid
    pub fn is_invalid(&self) -> Option<TokenInvalidationReason> {
        let elapsed = Utc::now().timestamp_millis() - self.emission_time_millis;

        let expired = elapsed > self.lifespan_millis;

        if self.invalidated_manually() {
            return Some(TokenInvalidationReason::Manual);
        } else if elapsed < 0 {
            return Some(TokenInvalidationReason::EmissionAfterTheCurrentDate);
        } else if expired {
            return Some(TokenInvalidationReason::Expired);
        }

        None
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
                Utc::now().timestamp_millis(),
                10_000,
            );
            assert_eq!(token.is_invalid(), None);

            // Manually invalidate token
            token.invalidate();
            // Should fail because token was invalidated
            assert_eq!(token.is_invalid(), Some(TokenInvalidationReason::Manual));
        }

        {
            // Time invalid token
            let invalid_token = Token::new(
                String::new(),
                TokenPrupose::Nothing,
                Utc::now().timestamp_millis() - 20_000,
                10_000,
            );
            // Should fail because token has expired
            assert_eq!(
                invalid_token.is_invalid(),
                Some(TokenInvalidationReason::Expired)
            );
        }

        {
            let invalid_token = Token::new(
                String::new(),
                TokenPrupose::Nothing,
                Utc::now().timestamp_millis() + 20_000,
                10_000,
            );

            // Should fail because token was emitted in the future
            assert_eq!(
                invalid_token.is_invalid(),
                Some(TokenInvalidationReason::EmissionAfterTheCurrentDate)
            );
        }
    }
}
