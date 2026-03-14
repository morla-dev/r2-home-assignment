use std::collections::HashSet;
use std::sync::RwLock;
use uuid::Uuid;

pub struct TokenStore {
    tokens: RwLock<HashSet<String>>,
}

impl TokenStore {
    pub fn new() -> Self {
        TokenStore {
            tokens: RwLock::new(HashSet::new()),
        }
    }

    pub fn generate(&self) -> String {
        let token = Uuid::new_v4().to_string();
        self.tokens.write().unwrap().insert(token.clone());
        token
    }

    pub fn is_valid(&self, token: &str) -> bool {
        self.tokens.read().unwrap().contains(token)
    }

    pub fn invalidate(&self, token: &str) -> bool {
        self.tokens.write().unwrap().remove(token)
    }
}
