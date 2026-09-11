use tracing::info;

use gtk::{glib, prelude::*};

use std::env;
use std::path::Path;

fn main() -> glib::ExitCode {
    tracing_subscriber::fmt::init();

    info!("Starting silo...");

    // let silo = silo_base::Silo::new();
    // let rc_silo = Rc::new(silo);

    // ui_relm::App::run(silo);
    // ui_gtk::App::run();
    let app = ui_gtk::App::new();
    return app.run();
}
