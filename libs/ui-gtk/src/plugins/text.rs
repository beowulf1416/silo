use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use super::Plugin;

use crate::components::main_window::{MainWindow, MainWindowInputMessage};

pub const PLUGIN_NAME: &str = "text";

pub fn factory() -> Box<dyn Plugin> {
    Box::new(TextPlugin::new())
}

#[derive(Debug)]
pub struct TextPlugin {}

impl TextPlugin {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Plugin for TextPlugin {
    fn name(&self) -> &str {
        return PLUGIN_NAME;
    }

    // fn build_widget(&self) -> gtk::Widget {
    //     let container = gtk::Box::builder()
    //         .orientation(gtk::Orientation::Vertical)
    //         .hexpand(true)
    //         .vexpand(true)
    //         .build();

    //     let btn_save = gtk::Button::builder()
    //         .icon_name("document-save-symbolic")
    //         .tooltip_text("Save")
    //         .build();

    //     let bar = gtk::ActionBar::builder().hexpand(true).build();
    //     bar.pack_start(&btn_save);

    //     container.append(&bar);

    //     return container.upcast::<gtk::Widget>();
    // }

    fn build_data_source_editor_widget(&self, window: &MainWindow) -> Option<gtk::Widget> {
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();

        return Some(container.upcast::<gtk::Widget>());
    }
}
