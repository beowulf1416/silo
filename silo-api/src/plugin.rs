use crate::{Connection, ConnectionData};

pub trait Plugin: std::fmt::Debug {
    fn name(&self) -> &str;
    fn init(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn create_connection(&self, data: Box<dyn ConnectionData>) -> Box<dyn Connection>;
}

pub type PluginFactory = fn() -> Box<dyn Plugin>;
