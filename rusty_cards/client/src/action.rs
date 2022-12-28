use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    // Information that the game starts with the greetings message
    Start(String),
}