#![deny(missing_docs)]
//! A simple key/value store.
use std::collections::HashMap;

/// Defines the structure where the data will be store
#[derive(Default)]
pub struct KvStore {
    /// The key value will be stored
    pub store: HashMap<String, String>,
}

impl KvStore {
    /// Comentario
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }

    /// Comentario
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Comentario
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    /// Comentario
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
