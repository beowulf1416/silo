use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use sourceview5::{Buffer, LanguageManager, StyleSchemeManager, View, prelude::*};

use serde_json::{Value, json};

use crate::{
    components::main_window::{MainWindow, MainWindowInputMessage},
    plugins::postgres::PostgresConnectionEditor,
};

// use super::Plugin;
use crate::plugins::Plugin;

pub const PLUGIN_NAME: &str = "postgres";

pub fn factory() -> Box<dyn Plugin> {
    Box::new(PostgresPlugin::new())
}

#[derive(Debug)]
pub struct PostgresPlugin {
    pub entry_name: gtk::Entry,
    pub entry_db: gtk::Entry,
    pub entry_host: gtk::Entry,
    pub entry_port: gtk::SpinButton,
    pub entry_user: gtk::Entry,
    pub entry_pw: gtk::PasswordEntry,
    pub entry_uri: gtk::Entry,
}

impl PostgresPlugin {
    pub fn new() -> Self {
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
    }

    pub fn get_configuration(&self) -> Value {
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
}

impl Plugin for PostgresPlugin {
    fn name(&self) -> &str {
        return PLUGIN_NAME;
    }

    // fn build_widget(&self) -> gtk::Widget {
    //     let btn_save = gtk::Button::builder()
    //         .icon_name("document-save-symbolic")
    //         .tooltip_text("Save")
    //         .build();

    //     let bar = gtk::ActionBar::builder().hexpand(true).build();
    //     bar.pack_start(&btn_save);

    //     let buffer = Buffer::new(None);
    //     let lm = LanguageManager::default();
    //     if let Some(l) = lm.language("postgresql") {
    //         buffer.set_language(Some(&l));
    //     }

    //     let sm = StyleSchemeManager::default();
    //     if let Some(s) = sm.scheme("classic") {
    //         buffer.set_style_scheme(Some(&s));
    //     }

    //     let sv = View::with_buffer(&buffer);
    //     sv.set_show_line_numbers(true);
    //     sv.set_highlight_current_line(true);
    //     sv.set_monospace(true);
    //     sv.set_tab_width(4);

    //     let sw = gtk::ScrolledWindow::builder()
    //         .hexpand(true)
    //         .vexpand(true)
    //         .has_frame(true)
    //         .child(&sv)
    //         .build();

    //     let container = gtk::Box::builder()
    //         .orientation(gtk::Orientation::Vertical)
    //         .hexpand(true)
    //         .vexpand(true)
    //         .build();
    //     container.append(&bar);
    //     container.append(&sw);

    //     return container.upcast::<gtk::Widget>();
    // }

    fn build_data_source_editor_widget(&self, window: &MainWindow) -> Option<gtk::Widget> {
        let editor = PostgresConnectionEditor::default();
        editor.set_main_window(window);
        return Some(editor.upcast());
    }

    /*
    fn build_data_source_editor_widget(&self, window: &MainWindow) -> Option<gtk::Widget> {
        let btn_save = gtk::Button::builder()
            // .icon_name("document-save-symbolic")
            .icon_name("save")
            .tooltip_text("Save")
            // .child(&icon_save)
            .build();

        // let cloned = self.clone();
        // btn_save.connect_clicked(glib::clone!(
        //     #[weak(rename_to = plugin)]
        //     self,
        //     move |_button| {
        //         debug!("//todo save button clicked");

        //         // let config = plugin.get_configuration();
        //     }
        // ));

        let btn_test = gtk::Button::builder()
            // .icon_name("system-run-symbolic")
            .icon_name("connect")
            .tooltip_text("Test connection")
            .build();
        btn_test.connect_clicked(|_button| {
            debug!("//todo test button clicked");
        });

        let bar = gtk::ActionBar::builder().hexpand(true).build();
        bar.pack_start(&btn_save);
        bar.pack_start(&btn_test);

        let grid = gtk::Grid::builder()
            .row_spacing(12)
            .column_spacing(12)
            .hexpand(true)
            .build();

        // row 0
        let label_name = gtk::Label::builder()
            .label("Name")
            .halign(gtk::Align::End)
            .build();

        grid.attach(&label_name, 0, 0, 1, 1);
        grid.attach(&self.entry_name, 1, 0, 1, 1);

        // row 1
        let label_db = gtk::Label::builder()
            .label("Database Name")
            .halign(gtk::Align::End)
            .build();

        grid.attach(&label_db, 0, 1, 1, 1);
        grid.attach(&self.entry_db, 1, 1, 1, 1);

        // row 2
        let label_host = gtk::Label::builder()
            .label("Host")
            .halign(gtk::Align::End)
            .build();

        grid.attach(&label_host, 0, 2, 1, 1);
        grid.attach(&self.entry_host, 1, 2, 1, 1);

        // row 3
        let label_port = gtk::Label::builder()
            .label("Port")
            .halign(gtk::Align::End)
            .build();

        grid.attach(&label_port, 0, 3, 1, 1);
        grid.attach(&self.entry_port, 1, 3, 1, 1);

        // row 4
        let label_user = gtk::Label::builder()
            .label("User")
            .halign(gtk::Align::End)
            .build();

        grid.attach(&label_user, 0, 4, 1, 1);
        grid.attach(&self.entry_user, 1, 4, 1, 1);

        // row 5
        let label_pw = gtk::Label::builder()
            .label("Password")
            .halign(gtk::Align::End)
            .build();

        grid.attach(&label_pw, 0, 5, 1, 1);
        grid.attach(&self.entry_pw, 1, 5, 1, 1);

        // row 6
        let label_uri = gtk::Label::builder()
            .label("URI")
            .halign(gtk::Align::End)
            .build();

        grid.attach(&label_uri, 0, 6, 1, 1);
        grid.attach(&self.entry_uri, 1, 6, 1, 1);

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        container.append(&bar);
        container.append(&grid);

        return Some(container.upcast::<gtk::Widget>());
    }
    */
}
