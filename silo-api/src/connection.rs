use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("unable to create connection")]
    Create,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionData {
    pub connection_name: &'static str,
    pub plugin: &'static str,
    pub host: &'static str,
    pub port: &'static str,
    pub name: &'static str,
    pub user: &'static str,
}

pub trait Connection: std::fmt::Debug {
    fn get_connection(&self, name: &str) -> Result<Box<dyn Connection>, ConnectionError>;
}
