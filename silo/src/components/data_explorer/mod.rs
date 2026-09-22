mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct DataExplorer(ObjectSubclass<imp::DataExplorerImpl>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl DataExplorer {
    pub fn new() -> Self {
        let window: Self = glib::Object::builder().build();

        return window;
    }
}
