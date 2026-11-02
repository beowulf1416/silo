mod imp;

use gtk::{glib, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct EditorPane(ObjectSubclass<imp::EditorPaneImpl>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl EditorPane {
    pub fn new() -> Self {
        let window: Self = glib::Object::builder().build();

        return window;
    }
}
