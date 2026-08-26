use tracing::{debug, error, info};

use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::cell::RefCell;
// use std::sync::Arc;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::nodes::data_source_node::PostgresDataSourceNode;
use silo_plugin::{ApplicationMessage, node::Node};

enum TestMessage {
    Success(String),
    Error(String),
}

#[derive(Debug, Default)]
pub struct PostgresConnectionEditor {
    // pub window: RefCell<Option<MainWindow>>,
    pub sender: RefCell<Option<async_channel::Sender<ApplicationMessage>>>,

    pub is_dirty: bool,
    pub is_value: bool,

    pub btn_save: gtk::Button,
    pub btn_test: gtk::Button,

    pub entry_name: gtk::Entry,
    pub entry_db: gtk::Entry,
    pub entry_host: gtk::Entry,
    pub entry_port: gtk::SpinButton,
    pub entry_user: gtk::Entry,
    pub entry_pw: gtk::PasswordEntry,
    pub entry_uri: gtk::Entry,

    pub label_test: gtk::Label,
}

impl PostgresConnectionEditor {
    // pub fn set_main_window(&self, window: &MainWindow) {
    //     self.window.replace(Some(window.clone()));
    // }

    pub fn set_is_dirty(&mut self, value: bool) {
        self.is_dirty = value;
        self.btn_save
            .action_set_enabled("win.data-source-save::postgres", self.is_dirty);
    }

    // pub fn save_configuration(&self) {
    //     debug!("save_configuration");

    //     // let imp = self.imp();

    //     let name = self.entry_name.text().to_string();
    //     let host = self.entry_host.text().to_string();
    //     let port = self.entry_port.text().to_string();
    //     let db = self.entry_db.text().to_string();
    //     let user = self.entry_user.text().to_string();
    //     let pw = self.entry_pw.text().to_string();

    //     // let config = serde_json::json!({
    //     //     "name": name,
    //     //     "db": db,
    //     //     "host": host,
    //     //     "port": port,
    //     //     "user": user,
    //     //     "pw": pw
    //     // });
    // }
    pub fn test_connection_details(&self, db: &str, host: &str, port: u32, user: &str, pw: &str) {
        debug!("test_connection_details");

        let (sender, receiver) = async_channel::unbounded::<String>();
        glib::MainContext::default().spawn_local(glib::clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                while let Ok(msg) = receiver.recv().await {
                    window.label_test.set_text(&msg);
                }
            }
        ));

        let db = db.to_string();
        let host = host.to_string();
        // let port = port;
        let user = user.to_string();
        let pw = pw.to_string();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                match attempt_connection(&db, &host, port, &user, &pw).await {
                    Err(e) => {
                        let _ = sender.send(format!("{}", e)).await;
                    }
                    Ok(_) => {
                        let _ = sender
                            .send("Successfully connected to database".to_string())
                            .await;
                    }
                }
            });
        });
    }
}

async fn attempt_connection(
    db: &String,
    host: &String,
    port: u32,
    user: &String,
    pw: &String,
) -> Result<(), sqlx::Error> {
    let uri = format!("postgres://{user}:{pw}@{host}:{port}/{db}");
    match PgPoolOptions::new().max_connections(1).connect(&uri).await {
        Err(e) => {
            error!("unable to connect to database: {} [{uri}]", e);
            Err(e)
        }
        Ok(_) => Ok(()),
    }
}

#[glib::object_subclass]
impl ObjectSubclass for PostgresConnectionEditor {
    const NAME: &'static str = "PostgresConnectionEditor";
    type Type = super::PostgresConnectionEditor;
    type ParentType = gtk::Box;
}

impl ObjectImpl for PostgresConnectionEditor {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        self.entry_name.set_placeholder_text(Some("Name"));

        self.entry_db.set_tooltip_text(Some("Database Name"));
        self.entry_db.set_placeholder_text(Some("Database"));

        self.entry_host.set_tooltip_text(Some("Host"));
        self.entry_host.set_placeholder_text(Some("Host"));

        self.entry_port.set_tooltip_text(Some("Port"));
        self.entry_port.set_adjustment(&gtk::Adjustment::new(
            0.0,
            0.0,
            u16::MAX as f64,
            1.0,
            1.0,
            0.0,
        ));
        self.entry_port.set_numeric(true);

        self.entry_user.set_tooltip_text(Some("User"));
        self.entry_user.set_placeholder_text(Some("User"));

        self.entry_pw.set_tooltip_text(Some("Password"));
        self.entry_pw.set_placeholder_text(Some("Password"));

        self.entry_uri
            .set_tooltip_text(Some("Uniform Resource Identifier"));
        self.entry_uri
            .set_placeholder_text(Some("postgresql://user:[password]@host:port/database"));
        self.entry_uri.set_hexpand(true);
        self.entry_uri.set_editable(false);

        // update uri entry value when
        // db, host, port, user change
        let update_uri = {
            let eh = self.entry_host.clone();
            let ep = self.entry_port.clone();
            let eu = self.entry_user.clone();
            let ed = self.entry_db.clone();
            let euri = self.entry_uri.clone();

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

        self.entry_host.connect_changed(move |_| update_uri_1());
        self.entry_port.connect_changed(move |_| update_uri_2());
        self.entry_user.connect_changed(move |_| update_uri_3());
        self.entry_db.connect_changed(move |_| update_uri_4());

        self.btn_save.set_icon_name("save");
        self.btn_save.set_tooltip_text(Some("Save"));
        self.btn_save.connect_clicked(glib::clone!(
            #[weak(rename_to = editor)]
            self,
            move |_button| {
                let boxed: Box<dyn Node> = Box::new(PostgresDataSourceNode::new(
                    editor.entry_name.text().as_str(),
                ));
                let _ = editor
                    .sender
                    .borrow()
                    .as_ref()
                    .expect("//todo sender")
                    // .send(ApplicationMessage::DataSourceAdd(boxed));
                    .send_blocking(ApplicationMessage::DataSourceAdd(boxed));
            }
        ));

        self.btn_test.set_icon_name("connect");
        self.btn_test.set_tooltip_text(Some("Test connection"));
        self.btn_test.connect_clicked(glib::clone!(
            #[weak(rename_to = editor)]
            self,
            move |_button| {
                debug!("//todo test button clicked");
                editor.test_connection_details(
                    editor.entry_db.text().as_str(),
                    editor.entry_host.text().as_str(),
                    editor.entry_port.value() as u32,
                    editor.entry_user.text().as_str(),
                    editor.entry_pw.text().as_str(),
                );
            }
        ));

        let bar = gtk::ActionBar::builder().hexpand(true).build();
        bar.pack_start(&self.btn_save);
        bar.pack_start(&self.btn_test);

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

        // row 7
        self.label_test.set_halign(gtk::Align::Start);
        self.label_test.set_hexpand(true);
        self.label_test.set_vexpand(true);
        self.label_test.set_wrap(true);
        grid.attach(&self.label_test, 0, 7, 2, 2);

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

        obj.append(&container);
    }
}

impl WidgetImpl for PostgresConnectionEditor {}

impl BoxImpl for PostgresConnectionEditor {}
