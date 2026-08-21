use tracing::{debug, error};

use async_channel::Sender;
use std::cell::Ref;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::APP_TITLE;
// use crate::components::header::Header;

#[derive(Debug, Clone)]
pub enum HeaderInputMessage {
    CloseRequested,
}

#[derive(Debug, Default)]
pub struct Header {
    pub header_bar: gtk::HeaderBar,
    // pub title_label: gtk::Label,
    pub btn_menu: gtk::MenuButton,
    pub btn_settings: gtk::Button,
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

        self.header_bar.set_title_widget(Some(&title_box));

        let menu_main = gio::Menu::new();
        menu_main.append(Some("_File"), None);

        let sub_menu_workspace = gio::Menu::new();
        sub_menu_workspace.append(Some("Open"), Some("win.workspace-open"));
        menu_main.append_submenu(Some("Workspace"), &sub_menu_workspace);

        let menu_section = gio::Menu::new();

        let menu_item = gio::MenuItem::new(Some("_Quit"), Some("win.quit"));
        menu_section.append_item(&menu_item);

        menu_main.insert_section(2, None, &menu_section);

        let pop_menu = gtk::PopoverMenu::from_model(Some(&menu_main));

        // menu button
        self.btn_menu.set_icon_name("open-menu-symbolic");
        self.btn_menu.set_tooltip_text(Some("Main Menu"));
        self.btn_menu.set_popover(Some(&pop_menu));
        // self.btn_menu.set_action_name(Some("win.quit"));
        self.header_bar.pack_start(&self.btn_menu);

        // settings button
        self.btn_settings
            .set_icon_name("preferences-system-symbolic");
        self.btn_settings.set_tooltip_text(Some("Settings"));
        self.header_bar.pack_end(&self.btn_settings);

        self.header_bar.set_hexpand(true);

        // self.add_actions();
        // self.setup_action_handlers();

        obj.append(&self.header_bar);
    }
}

impl WidgetImpl for Header {}

impl BoxImpl for Header {}
