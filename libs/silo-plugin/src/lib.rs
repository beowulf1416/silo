pub trait Plugin: std::fmt::Debug {
    fn register(&self) -> Result<(), &'static str>;
}
