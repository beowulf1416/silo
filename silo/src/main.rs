use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting silo...");

    ui_relm::App::run();
}
