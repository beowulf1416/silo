use tracing::{debug, error};

// use async_channel::Sender;
use std::cell::RefCell;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

// use crate::plugins::Plugin;
use silo_plugin::ApplicationMessage;

#[derive(Debug, Default)]
pub struct EditorView {
    pub nb: gtk::Notebook,
    pub sender: RefCell<Option<async_channel::Sender<ApplicationMessage>>>,
}

impl EditorView {
    pub fn add_editor(
        &self,
        display_name: &str,
        editor: gtk::Widget,
        sender: &async_channel::Sender<ApplicationMessage>,
    ) {
        self.sender.replace(Some(sender.clone()));

        // tab header
        let icon = gtk::Image::builder()
            .icon_name("folder-visiting-symbolic")
            .build();

        let suffix = self.nb.n_pages();
        let label = gtk::Label::builder()
            .label(format!("{} ({})", display_name, suffix))
            .build();

        let btn_close = gtk::Button::builder()
            .tooltip_text("close")
            .icon_name("window-close-symbolic")
            .css_classes(vec!["btn", "flat"])
            .build();
        btn_close.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_button| {
                debug!("close window requested {:?}", window.nb.current_page());

                let _ = window.sender.borrow().as_ref().unwrap().send_blocking(
                    ApplicationMessage::CloseEditorRequested(window.nb.current_page()),
                );
            }
        ));

        let th = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .build();
        th.append(&icon);
        th.append(&label);
        th.append(&btn_close);

        // content
        // let widget = plugin.build_widget();
        editor.set_hexpand(true);
        editor.set_vexpand(true);
        editor.set_widget_name("query_editor");

        let content = gtk::Box::builder()
            .hexpand(true)
            .vexpand(true)
            .margin_bottom(2)
            .margin_top(2)
            .margin_start(2)
            .margin_end(2)
            .build();
        content.append(&editor);

        let page_id = self.nb.append_page(&content, Some(&th));
        self.nb.set_current_page(Some(page_id));
    }

    pub fn remove_editor(&self, page: Option<u32>) {
        if let Some(page_id) = page {
            self.nb.remove_page(Some(page_id));
        }
    }
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

        let content_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();

        self.nb.set_hexpand(true);
        self.nb.set_vexpand(true);
        content_box.append(&self.nb);

        obj.append(&content_box);
    }
}

impl WidgetImpl for EditorView {}

impl BoxImpl for EditorView {}
