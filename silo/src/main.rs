use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting silo...");

    let silo = silo_base::Silo::new();
    // let rc_silo = Rc::new(silo);

    ui_relm::App::run(silo);
}
