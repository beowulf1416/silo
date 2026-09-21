use gtk::{glib, subclass::prelude::*};

#[derive(Debug, Default)]
pub struct AppImpl {}

#[glib::object_subclass]
impl ObjectSubclass for AppImpl {
    const NAME: &'static str = crate::APP_NAME;
    type Type = super::App;
    type ParentType = gtk::Application;
}

impl ObjectImpl for AppImpl {}

impl ApplicationImpl for AppImpl {}

impl GtkApplicationImpl for AppImpl {}

impl AppImpl {}
