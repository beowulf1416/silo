mod actions;
mod app;
mod components;
mod config;

use gtk::{glib, prelude::*};
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::runtime::Runtime;
use tracing::info;

use silo_api::Silo;
use silo_base::SiloImpl;

use crate::app::App;

const APP_ID: &str = "org.devphilplus.silo";
// const PROFILE: &str = "Devel";
const APP_TITLE: &str = "silo";
static APP_NAME: &str = "Silo";

fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting silo...");

    let workspace_path = get_last_workspace_path();

    let mut silo = silo_base::silo::SiloImpl::new();
    silo.initialize();
    silo.set_workspace_path(&workspace_path);

    //     // let silo: Arc<dyn Silo> = Arc::new(SiloImpl::new());
    //     let mut silo: Arc<dyn Silo> = Arc::new(silo_base::silo::SiloImpl::new());
    //     silo.initialize();
    //
    //     // info!("workspace_path: {}", workspace_path);
    //     silo.set_workspace_path(&workspace_path);

    let app = App::new(Arc::new(silo));
    app.run();
}

fn get_last_workspace_path() -> String {
    info!("get_last_workspace_path");

    if let Some(dirs) = directories::ProjectDirs::from("com", "devphilplus", "silo") {
        let config_dir = dirs.config_dir();
        if !config_dir.exists() {
            let _ = std::fs::create_dir_all(&config_dir);
        }
        // let config_file = config_dir.join("data_sources.json");

        return String::from(config_dir.to_str().unwrap());
    } else {
        return String::from("");
    }
}

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn get_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| Runtime::new().expect("Failed to start Tokio runtime"))
}
