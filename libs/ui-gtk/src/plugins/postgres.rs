use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use super::Plugin;

pub const PLUGIN_NAME: &str = "postgres";

pub fn factory() -> Box<dyn Plugin> {
    Box::new(PostgresPlugin::new())
}

#[derive(Debug)]
pub struct PostgresPlugin {}

impl PostgresPlugin {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Plugin for PostgresPlugin {
    fn name(&self) -> &str {
        return PLUGIN_NAME;
    }

    fn build_widget(&self) -> gtk::Widget {
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();

        let btn_save = gtk::Button::builder()
            .icon_name("document-save-symbolic")
            .tooltip_text("Save")
            .build();

        let bar = gtk::ActionBar::builder().hexpand(true).build();
        bar.pack_start(&btn_save);

        container.append(&bar);

        return container.upcast::<gtk::Widget>();
    }
}
