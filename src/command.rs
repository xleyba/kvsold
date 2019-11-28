use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set { key: String, value: String},
    Get { key: String },
    Rm { key: String },
}

impl Command {
    pub fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }

    pub fn remove(key: String) -> Command {
        Command::Rm { key }
    }
}