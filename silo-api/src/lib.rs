pub mod plugin;
pub mod sql;

pub trait ConnectionData {
    fn name(&self) -> &str;

    fn host(&self) -> &str;
    fn port(&self) -> u16;
    fn store_name(&self) -> &str;
    fn user(&self) -> &str;

    fn open(&self) -> Result<Box<dyn Connection>, &'static str>;
}

pub trait Connection {}
