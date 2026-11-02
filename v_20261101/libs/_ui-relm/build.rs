fn main() {
    glib_build_tools::compile_resources(
        &["assets/images"],
        "assets/resources.gresource.xml",
        "org.devphilplus.silo.gresource",
    );

    // relm4_icons_build::bundle_icons(
    //     "icons.rs",
    //     Some("org.devphilplus.silo"),
    //     Some("/org/devphilplus/silo"),
    //     Some("assets/images/"),
    //     ["silo.svg"],
    // );
}
