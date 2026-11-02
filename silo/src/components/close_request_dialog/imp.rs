use gtk::{gio, gio::prelude::*, glib, prelude::*, subclass::prelude::*};
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct CloseRequestDialogImpl {}

impl CloseRequestDialogImpl {}

#[glib::object_subclass]
impl ObjectSubclass for CloseRequestDialogImpl {
    const NAME: &'static str = "CloseRequestDialogImpl";
    type Type = super::CloseRequestDialog;
    type ParentType = gtk::Window;
}

impl ObjectImpl for CloseRequestDialogImpl {
    fn constructed(&self) {
        self.parent_constructed();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_start(10)
            .margin_end(10)
            .margin_top(10)
            .margin_bottom(10)
            .build();

        let obj = self.obj();
        obj.set_modal(true);
        obj.set_child(Some(&container));
    }
}

impl WindowImpl for CloseRequestDialogImpl {}

impl WidgetImpl for CloseRequestDialogImpl {}
