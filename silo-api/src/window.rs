pub trait MainWindow: std::fmt::Debug + Send + Sync {
    fn show_password_dialog(&self);
}
