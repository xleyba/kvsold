pub mod error;
pub use crate::error::{KvsError, Result};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {

    pub fn new(path: PathBuf)  -> Result<KvStore> {
        let store = HashMap::new();
        Ok(
            KvStore {
                store,
            }
        )
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        Ok(())
    }

    pub fn rm(&mut self, key: String) -> Result<()> {
        Ok(())
    }

    
}


