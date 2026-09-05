use tracing::{debug, error};

// use async_channel::Sender;
// use std::cell::Ref;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::APP_TITLE;
// use crate::components::header::Header;

#[derive(Debug, Clone)]
pub enum HeaderInputMessage {
    CloseRequested,
}

#[derive(Debug, Default)]
pub struct Header {
    // pub header_bar: gtk::HeaderBar,
    // pub title_label: gtk::Label,
    pub btn_menu: gtk::MenuButton,
    pub btn_settings: gtk::Button,
}

impl Header {
    fn build_main_menu(&self) -> gio::Menu {
        let menu = gio::Menu::new();
        menu.append(Some("_File"), None);

        let section = gio::Menu::new();

        let item = gio::MenuItem::new(Some("Open ..."), Some("win.workspace-open"));
        section.append_item(&item);

        let item = gio::MenuItem::new(Some("Save"), Some("win.workspace-save"));
        section.append_item(&item);

        menu.append_section(Some("Workspace"), &section);

        let section = gio::Menu::new();
        let item = gio::MenuItem::new(Some("_Quit"), Some("win.quit"));
        section.append_item(&item);
        menu.append_section(None, &section);

        return menu;
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Header {
    const NAME: &'static str = "Header";
    type Type = super::Header;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {}

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {}
}

impl ObjectImpl for Header {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        // header
        // self.title_label.set_text(APP_TITLE);
        // self.title_label.set_hexpand(true);
        // self.header_bar.set_title_widget(Some(&self.title_label));

        let icon = gtk::Image::builder().icon_name("silo").build();

        let label = gtk::Label::builder().label(APP_TITLE).build();

        let title_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(12)
            .build();
        title_box.append(&icon);
        title_box.append(&label);

        // let menu = self.build_main_menu();

        // let pop_menu = gtk::PopoverMenu::from_model(Some(&menu));

        // // menu button
        // self.btn_menu.set_icon_name("open-menu-symbolic");
        // self.btn_menu.set_tooltip_text(Some("Main Menu"));
        // self.btn_menu.set_popover(Some(&pop_menu));

        // // settings button
        // self.btn_settings
        //     .set_icon_name("preferences-system-symbolic");
        // self.btn_settings.set_tooltip_text(Some("Settings"));

        obj.append(&title_box);
    }
}

impl WidgetImpl for Header {}

impl BoxImpl for Header {}
