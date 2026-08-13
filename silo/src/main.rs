use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting silo...");

    let silo = silo_base::Silo::new();

    ui_relm::App::run(&silo);
}
