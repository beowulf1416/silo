mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct DataSourceView(ObjectSubclass<imp::DataSourceView>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl DataSourceView {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl Default for DataSourceView {
    fn default() -> Self {
        return Self::new();
    }
}
