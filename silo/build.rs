use std::process::Command;

fn main() {
    glib_build_tools::compile_resources(
        &["assets"],
        "assets/resources.gresource.xml",
        "org.devphilplus.silo.gresource",
    );

    let status = Command::new("glib-compile-schemas").arg(".").status();

    if let Ok(status) = status {
        if !status.success() {
            panic!("Failed to compile schemas");
        }
    }
}
