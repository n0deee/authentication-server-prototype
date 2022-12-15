use chrono::Utc;
use serde::{Deserialize, Serialize};
use auth_lib::db::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PermissionLevel {
    No,
    Normal,
    Privileged,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TokenInsertionError {
    AlreadyExists,
}

#[derive(Serialize, Deserialize)]
pub struct RegisteredUser {
    id: UserID,
    username: String,
    hashed_password: String,
    permission_level: PermissionLevel,
}

pub struct MemoryTokenManager {
    tokens: Vec<Token>,
}

impl MemoryTokenManager {
    pub fn new() -> MemoryTokenManager {
        MemoryTokenManager { tokens: vec![] }
    }

    pub fn insert(&mut self, token: Token) -> Result<&Token, TokenInsertionError> {
        if self.key_exists(&token.key) {
            return Err(TokenInsertionError::AlreadyExists);
        }

        self.tokens.push(token);
        let token_red = self.tokens.last().unwrap();
        return Ok(token_red);
    }

    // TODO: Otimize
    pub fn key_exists(&self, key: &TokenKey) -> bool {
        for token in self.tokens.iter() {
            if token.key == *key {
                return true;
            }
        }

        false
    }

    pub fn get_token_by_key(&self, key: &TokenKey) -> Option<&Token> {
        for token in self.tokens.iter() {
            if token.key == *key {
                return Some(&token);
            }
        }

        None
    }
}