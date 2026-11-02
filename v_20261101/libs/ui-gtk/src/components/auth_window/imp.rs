use adw::{prelude::*, subclass::prelude::*};
use gtk::{
    gio,
    gio::prelude::*,
    glib::{self, BoxedAnyObject, clone},
    prelude::*,
    subclass::prelude::*,
};
use std::cell::RefCell;
use tracing::debug;

#[derive(Debug, Default)]
pub struct AuthWindowImp {
    pub(super) entry_name: gtk::Entry,
    pub(super) entry_pw: gtk::PasswordEntry,

    pub(super) pw: RefCell<Option<String>>,
}

impl AuthWindowImp {
    pub fn set_user(&self, user: &str) {
        self.entry_name.set_text(user);
    }

    pub fn get_password(&self) -> String {
        return self.entry_pw.text().to_string();
    }
}

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
            .hexpand(true)
            .row_spacing(12)
            .column_spacing(12)
            .margin_start(16)
            .margin_end(16)
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
        let box_actions = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .spacing(16)
            .build();

        let btn_ok = gtk::Button::builder().label("OK").build();
        btn_ok.connect_clicked(glib::clone!(
            #[weak]
            obj,
            #[weak(rename_to = this)]
            self,
            move |_btn| {
                this.pw.replace(Some(this.entry_pw.text().to_string()));

                // let obj = this.obj();
                obj.return_value(this.pw.borrow().clone());

                obj.close();
            }
        ));
        let btn_cancel = gtk::Button::builder().label("Cancel").build();
        btn_cancel.connect_clicked(glib::clone!(
            #[weak]
            obj,
            move |_btn| {
                obj.close();
            }
        ));
        box_actions.append(&btn_ok);
        box_actions.append(&btn_cancel);

        grid.attach(&box_actions, 1, 2, 1, 2);

        let title = gtk::Label::builder().label("Authenticate").build();
        let header = adw::HeaderBar::builder().title_widget(&title).build();

        let content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .spacing(16)
            // .margin_start(16)
            // .margin_end(16)
            .build();
        content.append(&header);
        content.append(&grid);

        obj.set_content(Some(&content));
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
