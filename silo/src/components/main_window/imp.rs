use adw::{prelude::*, subclass::prelude::*};
use gtk::prelude::GtkWindowExt;
use gtk::{gio, gio::prelude::*, glib, prelude::*, subclass::prelude::*};

#[derive(Debug, Default)]
pub struct MainWindowImpl {}

#[glib::object_subclass]
impl ObjectSubclass for MainWindowImpl {
    const NAME: &'static str = "MainWindowImpl";
    type Type = super::MainWindow;
    type ParentType = adw::ApplicationWindow;
}

impl ObjectImpl for MainWindowImpl {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WindowImpl for MainWindowImpl {}

// impl GtkWindowExt for MainWindowImpl {}

impl WidgetImpl for MainWindowImpl {}

impl ApplicationWindowImpl for MainWindowImpl {}

impl AdwApplicationWindowImpl for MainWindowImpl {}

impl MainWindowImpl {}
