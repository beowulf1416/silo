pub trait AppWindow: std::fmt::Debug {
    fn show_password_dialog(&self) -> anyhow::Result<String>;
}
