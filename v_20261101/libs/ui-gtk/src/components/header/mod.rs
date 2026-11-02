mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct Header(ObjectSubclass<imp::Header>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl Header {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl Default for Header {
    fn default() -> Self {
        return Self::new();
    }
}
