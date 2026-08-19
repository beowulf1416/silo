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
    pub title_label: gtk::Label,
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
        self.title_label.set_text(APP_TITLE);
        self.title_label.set_hexpand(true);
        self.header_bar.set_title_widget(Some(&self.title_label));

        let menu = gio::Menu::new();
        menu.append(Some("_Quit"), Some("win.quit"));

        let pop_menu = gtk::PopoverMenu::from_model(Some(&menu));

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
