use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use sourceview5::{Buffer, LanguageManager, StyleSchemeManager, View, prelude::*};

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
        let btn_save = gtk::Button::builder()
            .icon_name("document-save-symbolic")
            .tooltip_text("Save")
            .build();

        let bar = gtk::ActionBar::builder().hexpand(true).build();
        bar.pack_start(&btn_save);

        let buffer = Buffer::new(None);
        let lm = LanguageManager::default();
        if let Some(l) = lm.language("postgresql") {
            buffer.set_language(Some(&l));
        }

        let sm = StyleSchemeManager::default();
        if let Some(s) = sm.scheme("classic") {
            buffer.set_style_scheme(Some(&s));
        }

        let sv = View::with_buffer(&buffer);
        sv.set_show_line_numbers(true);
        sv.set_highlight_current_line(true);
        sv.set_monospace(true);
        sv.set_tab_width(4);

        let sw = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .has_frame(true)
            .child(&sv)
            .build();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();
        container.append(&bar);
        container.append(&sw);

        return container.upcast::<gtk::Widget>();
    }
}
