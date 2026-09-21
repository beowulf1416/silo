mod imp;

use adw::prelude::*;
use gtk::{gdk, gio, glib, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct App(ObjectSubclass<imp::AppImpl>)
    @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl App {}
