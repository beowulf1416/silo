fn main() {
    glib_build_tools::compile_resources(
        &["assets/images"],
        "assets/resources.gresource.xml",
        "org.devphilplus.silo.gresource",
    );
}
