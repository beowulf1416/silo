mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct StatusBar(ObjectSubclass<imp::StatusBarImpl>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl StatusBar {
    pub fn new() -> Self {
        let obj: Self = glib::Object::builder().build();

        return obj;
    }
}
