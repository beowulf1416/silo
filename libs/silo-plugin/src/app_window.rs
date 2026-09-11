pub trait AppWindow {
    fn show_password_dialog(&self) -> anyhow::Result<String>;
}
