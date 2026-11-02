mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::app::App;
use crate::components::app_window::AppWindowMessages;

glib::wrapper! {
    pub struct CloseRequestDialog(ObjectSubclass<imp::CloseRequestDialogImpl>)
    @extends gtk::Widget, gtk::Window, adw::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
        gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionMap, gio::ActionGroup
    ;
}

impl CloseRequestDialog {}
