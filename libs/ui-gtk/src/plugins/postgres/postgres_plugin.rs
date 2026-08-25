use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};
// use sourceview5::{Buffer, LanguageManager, StyleSchemeManager, View, prelude::*};

use serde_json::{Value, json};

use crate::{
    components::main_window::{MainWindow, MainWindowInputMessage},
    plugins::postgres::connection_editor::PostgresConnectionEditor,
    plugins::postgres::query_editor::PostgresQueryEditor,
};

// use super::Plugin;
use crate::plugins::Plugin;

pub const PLUGIN_NAME: &str = "postgres";

pub fn factory() -> Box<dyn Plugin> {
    Box::new(PostgresPlugin::new())
}

#[derive(Debug)]
pub struct PostgresPlugin {
    // pub entry_name: gtk::Entry,
    // pub entry_db: gtk::Entry,
    // pub entry_host: gtk::Entry,
    // pub entry_port: gtk::SpinButton,
    // pub entry_user: gtk::Entry,
    // pub entry_pw: gtk::PasswordEntry,
    // pub entry_uri: gtk::Entry,
}

impl PostgresPlugin {
    pub fn new() -> Self {
        /*
        let entry_name = gtk::Entry::builder().placeholder_text("Name").build();

        let entry_db = gtk::Entry::builder()
            .tooltip_text("Database Name")
            .placeholder_text("Database")
            .build();
        // entry_db.connect_has_focus_notify(|w| if (!w.has_focus()) {});

        let entry_host = gtk::Entry::builder()
            .tooltip_text("Host")
            .placeholder_text("Host")
            .build();

        let entry_port = gtk::SpinButton::builder()
            .tooltip_text("Port")
            .numeric(true)
            .build();

        let entry_user = gtk::Entry::builder()
            .tooltip_text("User")
            .placeholder_text("User")
            .build();

        let entry_pw = gtk::PasswordEntry::builder()
            .tooltip_text("Password")
            .placeholder_text("Password")
            .build();

        let entry_uri = gtk::Entry::builder()
            .tooltip_text("Uniform Resource Identifier")
            .placeholder_text("postgresql://user:[password]@host:port/database")
            .hexpand(true)
            .editable(false)
            .build();

        // update uri entry value when
        // db, host, port, user change
        let update_uri = {
            let eh = entry_host.clone();
            let ep = entry_port.clone();
            let eu = entry_user.clone();
            let ed = entry_db.clone();
            let euri = entry_uri.clone();

            move || {
                let user = eu.text();
                let host = eh.text();
                let port = ep.text();
                let db = ed.text();

                let uri = format!("postgresql://{user}:[password]@{host}:{port}/{db}");
                euri.set_text(&uri);
            }
        };
        let update_uri_1 = update_uri.clone();
        let update_uri_2 = update_uri.clone();
        let update_uri_3 = update_uri.clone();
        let update_uri_4 = update_uri.clone();

        entry_host.connect_changed(move |_| update_uri_1());
        entry_port.connect_changed(move |_| update_uri_2());
        entry_user.connect_changed(move |_| update_uri_3());
        entry_db.connect_changed(move |_| update_uri_4());

        return Self {
            entry_name: entry_name,
            entry_db: entry_db,
            entry_host: entry_host,
            entry_port: entry_port,
            entry_user: entry_user,
            entry_pw: entry_pw,
            entry_uri: entry_uri,
        };
        */
        return Self {};
    }

    /*
    pub fn get_configuration(&self) -> Value {
        debug!("get_configuration");

        let name = self.entry_name.text().to_string();
        let db = self.entry_db.text().to_string();
        let host = self.entry_host.text().to_string();
        let port = self.entry_port.text().to_string();
        let user = self.entry_user.text().to_string();
        let pw = self.entry_pw.text().to_string();

        let config = json!({
            "type": "postgres",
            "name": name,
            "db": db,
            "host": host,
            "port": port,
            "user": user,
            "pw": pw
        });

        return config;
    }
    */
}

impl Plugin for PostgresPlugin {
    fn name(&self) -> &str {
        return PLUGIN_NAME;
    }

    fn build_data_source_editor_widget(&self, window: &MainWindow) -> Option<gtk::Widget> {
        let editor = PostgresConnectionEditor::default();
        editor.set_main_window(window);
        return Some(editor.upcast());
    }

    fn build_query_editor_widget(&self, window: &MainWindow) -> Option<gtk::Widget> {
        let editor = PostgresQueryEditor::default();
        editor.set_main_window(window);
        return Some(editor.upcast());
    }
}
