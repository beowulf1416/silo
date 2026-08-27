mod actions;
mod app;
mod components;
mod plugins;

use std::sync::OnceLock;
use tokio::runtime::Runtime;

pub use app::App;

pub const APP_ID: &str = "org.devphilplus.silo";
pub const PROFILE: &str = "Devel";
pub const APP_TITLE: &str = "silo";

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn get_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| Runtime::new().expect("Failed to start Tokio runtime"))
}
