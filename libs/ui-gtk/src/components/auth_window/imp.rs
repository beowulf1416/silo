use tracing::debug;

use adw::{prelude::*, subclass::prelude::*};
use gtk::{
    gio,
    gio::prelude::*,
    glib::{self, BoxedAnyObject, clone},
    prelude::*,
    subclass::prelude::*,
};

#[derive(Debug, Default)]
pub struct AuthWindowImp {
    pub(super) entry_name: gtk::Entry,
    pub(super) entry_pw: gtk::PasswordEntry,
}

impl AuthWindowImp {}

#[glib::object_subclass]
impl ObjectSubclass for AuthWindowImp {
    const NAME: &'static str = "AuthWindowImp";
    type Type = super::AuthWindow;
    type ParentType = adw::Window;
}

impl ObjectImpl for AuthWindowImp {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        let grid = gtk::Grid::builder()
            .row_spacing(12)
            .column_spacing(12)
            .hexpand(true)
            .build();

        // row 0
        let label = gtk::Label::builder().label("User").build();
        self.entry_name.set_placeholder_text(Some("User"));
        self.entry_name.set_tooltip_text(Some("User"));

        grid.attach(&label, 0, 0, 1, 1);
        grid.attach(&self.entry_name, 1, 0, 1, 1);

        // row 1
        let label = gtk::Label::builder().label("Password").build();
        self.entry_pw.set_placeholder_text(Some("Password"));
        self.entry_pw.set_tooltip_text(Some("Password"));

        grid.attach(&label, 0, 1, 1, 1);
        grid.attach(&self.entry_pw, 1, 1, 1, 1);

        // row 2
        let btn_ok = gtk::Button::builder().label("OK").build();
        let btn_cancel = gtk::Button::builder().label("Cancel").build();

        grid.attach(&btn_ok, 1, 2, 1, 1);
        grid.attach(&btn_cancel, 2, 2, 1, 1);

        obj.set_child(Some(&grid));
    }
}

impl WidgetImpl for AuthWindowImp {}

impl WindowImpl for AuthWindowImp {
    // fn close_request(&self) -> glib::Propagation {
    //     debug!("close_request");
    //     glib::Propagation::Stop
    // }
}

impl AdwWindowImpl for AuthWindowImp {}
