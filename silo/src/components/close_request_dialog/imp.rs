use gtk::{gio, gio::prelude::*, glib, prelude::*, subclass::prelude::*};
use std::cell::RefCell;

use crate::components::main_window::{MainWindow, MainWindowMessages};

#[derive(Debug, Default)]
pub struct CloseRequestDialogImpl {
    pub(super) parent: RefCell<Option<MainWindow>>,
}

#[glib::object_subclass]
impl ObjectSubclass for CloseRequestDialogImpl {
    const NAME: &'static str = "CloseRequestDialogImpl";
    type Type = super::CloseRequestDialog;
    type ParentType = gtk::Window;
}

impl ObjectImpl for CloseRequestDialogImpl {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        let lbl = gtk::Label::builder()
            .label("Are you sure you want to exit Silo?")
            .build();

        let btn_ok = gtk::Button::builder().label("OK").build();
        btn_ok.connect_clicked(glib::clone!(
            #[weak]
            obj,
            #[weak(rename_to = this)]
            self,
            move |_btn| {
                this.parent
                    .borrow_mut()
                    .as_ref()
                    .unwrap()
                    .send(MainWindowMessages::Close);
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

        let container_actions = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .vexpand(true)
            .build();

        container_actions.append(&btn_ok);
        container_actions.append(&btn_cancel);

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_start(10)
            .margin_end(10)
            .margin_top(10)
            .margin_bottom(10)
            .build();

        container.append(&lbl);
        container.append(&container_actions);

        obj.set_modal(true);
        obj.set_child(Some(&container));
    }
}

impl WindowImpl for CloseRequestDialogImpl {}

impl WidgetImpl for CloseRequestDialogImpl {}

impl CloseRequestDialogImpl {
    pub fn set_parent(&self, parent: &MainWindow) {
        self.parent.replace(Some(parent.clone()));
    }
}
