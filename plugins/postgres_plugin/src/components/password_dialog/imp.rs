use gtk::{gio, gio::prelude::*, glib, prelude::*, subclass::prelude::*};
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct PasswordDialogImpl {}

impl PasswordDialogImpl {}

#[glib::object_subclass]
impl ObjectSubclass for PasswordDialogImpl {
    const NAME: &'static str = "PasswordDialogImpl";
    type Type = super::PasswordDialog;
    type ParentType = gtk::Window;
}

impl ObjectImpl for PasswordDialogImpl {
    fn constructed(&self) {
        self.parent_constructed();

        let grid = gtk::Grid::builder()
            .margin_start(12)
            .margin_end(12)
            .margin_top(12)
            .margin_bottom(12)
            .row_spacing(6)
            .column_spacing(6)
            .build();

        let lbl = gtk::Label::builder()
            .label("User")
            .tooltip_text("User")
            .build();
        let entry_user = gtk::Entry::builder().build();
        grid.attach(&lbl, 0, 0, 1, 1);
        grid.attach(&entry_user, 1, 0, 1, 1);

        let lbl = gtk::Label::builder()
            .label("Password")
            .tooltip_text("Password")
            .build();
        let entry_pw = gtk::PasswordEntry::builder().build();
        grid.attach(&lbl, 0, 1, 1, 1);
        grid.attach(&entry_pw, 1, 1, 1, 1);

        let btn_cancel = gtk::Button::builder()
            .label("Cancel")
            .tooltip_text("Cancel")
            .icon_name("cancel-symbolic")
            .build();
        let btn_ok = gtk::Button::builder()
            .label("Ok")
            .tooltip_text("Ok")
            .icon_name("ok-symbolic")
            .build();
        grid.attach(&btn_cancel, 0, 2, 1, 1);
        grid.attach(&btn_ok, 1, 2, 1, 1);

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_start(10)
            .margin_end(10)
            .margin_top(10)
            .margin_bottom(10)
            .build();

        container.append(&grid);

        let obj = self.obj();
        obj.set_modal(true);
        obj.set_child(Some(&container));
    }
}

impl WindowImpl for PasswordDialogImpl {}

impl WidgetImpl for PasswordDialogImpl {}
