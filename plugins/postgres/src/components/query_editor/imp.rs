use gtk::gsk::PorterDuff::Source;
use tracing::{debug, error};

use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::cell::RefCell;
// use std::sync::Arc;

use sourceview5::prelude::*;

// use crate::components::main_window::MainWindow;
use silo_plugin::ApplicationMessage;

#[derive(Debug, Default)]
pub struct PostgresQueryEditor {
    // pub window: RefCell<Option<MainWindow>>,
    pub sender: RefCell<Option<async_channel::Sender<ApplicationMessage>>>,
}

impl PostgresQueryEditor {
    // pub fn set_main_window(&self, window: &MainWindow) {
    //     self.window.replace(Some(window.clone()));
    // }

    fn build_action_bar(&self) -> gtk::ActionBar {
        let btn_save = gtk::Button::builder()
            // .label("Save")
            .icon_name("save")
            .tooltip_text("Save")
            .css_classes(vec!["btn", "flat"])
            .build();
        btn_save.connect_clicked(|_button| {
            debug!("button save clicked");
        });

        let separator = gtk::Separator::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();

        let btn_execute = gtk::Button::builder()
            // .label("Execute")
            .icon_name("system-play-start")
            .tooltip_text("Execute")
            .css_classes(vec!["btn", "flat"])
            .build();
        btn_execute.connect_clicked(|_button| {
            debug!("button execute clicked");
        });

        let action_bar = gtk::ActionBar::builder()
            .hexpand(true)
            .css_classes(vec!["action-bar"])
            .build();
        action_bar.pack_start(&btn_save);
        action_bar.pack_start(&separator);
        action_bar.pack_start(&btn_execute);

        return action_bar;
    }

    fn build_editor(&self) -> gtk::ScrolledWindow {
        let buffer = sourceview5::Buffer::new(None);
        buffer.set_highlight_syntax(true);
        if let Some(ref language) = sourceview5::LanguageManager::new().language("sql") {
            buffer.set_language(Some(language));
        }

        if let Some(ref scheme) = sourceview5::StyleSchemeManager::new().scheme("solarized-light") {
            buffer.set_style_scheme(Some(scheme));
        }

        let view = sourceview5::View::with_buffer(&buffer);
        view.set_monospace(true);
        view.set_background_pattern(sourceview5::BackgroundPatternType::Grid);
        view.set_show_line_numbers(true);
        view.set_highlight_current_line(true);
        // view.set_highlight_matching_brackets(true);
        view.set_tab_width(4);
        view.set_hexpand(true);
        view.set_vexpand(true);

        let sv = gtk::ScrolledWindow::builder()
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .child(&view)
            .build();

        return sv;
    }
}

#[glib::object_subclass]
impl ObjectSubclass for PostgresQueryEditor {
    const NAME: &'static str = "PostgresQueryEditor";
    type Type = super::PostgresQueryEditor;
    type ParentType = gtk::Box;
}

impl ObjectImpl for PostgresQueryEditor {
    fn constructed(&self) {
        self.parent_constructed();

        let action_bar = self.build_action_bar();
        let editor = self.build_editor();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();
        container.append(&action_bar);
        container.append(&editor);

        let obj = self.obj();
        obj.append(&container);
    }
}

impl WidgetImpl for PostgresQueryEditor {}

impl BoxImpl for PostgresQueryEditor {}
