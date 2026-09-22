mod app;
mod components;

use tracing::info;

use gtk::{glib, prelude::*};

static APP_ID: &str = "org.devphilplus.silo";
static APP_NAME: &str = "Silo";

fn main() -> glib::ExitCode {
    tracing_subscriber::fmt::init();

    info!("Starting silo...");

    // let app = ui_gtk::App::new();
    // return app.run();

    let app = app::App::new();
    return app.run();
}
