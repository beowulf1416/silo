use tracing::{debug, error};

use async_channel::Sender;
use std::cell::Ref;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

#[derive(Debug, Default)]
pub struct EditorView {
    pub nb: gtk::Notebook,
}

impl EditorView {
    pub fn add_editor(&self, widget: gtk::Widget) {}
}

#[glib::object_subclass]
impl ObjectSubclass for EditorView {
    const NAME: &'static str = "EditorView";
    type Type = super::EditorView;
    type ParentType = gtk::Box;
}

impl ObjectImpl for EditorView {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        // page content
        let btn_save = gtk::Button::builder()
            .icon_name("document-save-symbolic")
            .tooltip_text("Save")
            .css_classes(vec!["btn", "flat"])
            .build();
        btn_save.connect_clicked(|_button| {
            debug!("//todo save button clicked");
        });

        let action_bar = gtk::ActionBar::builder()
            .hexpand(true)
            .tooltip_text("Actions")
            .build();
        action_bar.pack_start(&btn_save);

        let tv = gtk::TextView::builder()
            .tooltip_text("Editor")
            .hexpand(true)
            .vexpand(true)
            .build();

        let sw = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .hscrollbar_policy(gtk::PolicyType::Automatic)
            .child(&tv)
            .build();

        let content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();
        content.append(&action_bar);
        content.append(&sw);

        // tab
        let icon = gtk::Image::builder().icon_name("folder-visiting").build();

        let label = gtk::Label::builder().label("tab 1").build();

        let btn_close = gtk::Button::builder()
            .tooltip_text("close")
            .icon_name("window-close-symbolic")
            .css_classes(vec!["btn", "flat"])
            .build();
        btn_close.connect_clicked(|_button| {
            debug!("//todo: close button clicked");
        });

        let tab_container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(4)
            .build();
        tab_container.append(&icon);
        tab_container.append(&label);
        tab_container.append(&btn_close);

        // let nb = gtk::Notebook::builder().hexpand(true).vexpand(true).build();
        self.nb.append_page(&content, Some(&tab_container));

        let content_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();
        // content_box.append(&self.nb);

        self.nb.set_hexpand(true);
        self.nb.set_vexpand(true);
        content_box.append(&self.nb);

        obj.append(&content_box);
    }
}

impl WidgetImpl for EditorView {}

impl BoxImpl for EditorView {}
