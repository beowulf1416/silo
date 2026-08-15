use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    uri: String,
}

impl Connection {
    pub fn new(uri: String) -> Self {
        return Self { uri };
    }
}
