use sqlx::mysql::MySqlPoolOptions;
use tracing::{debug, error};

use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::cell::{Ref, RefCell};
use std::sync::Arc;

use crate::get_runtime;
use crate::nodes::ConnectionSettings;
use crate::nodes::data_source_node::MySQLDataSourceNode;

use silo_plugin::{ApplicationMessage, node::Node};

#[derive(Debug, Default)]
pub struct MySQLConnectionEditor {
    actions: gio::SimpleActionGroup,

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

    pub(super) label_test: gtk::Label,
}

impl MySQLConnectionEditor {
    fn setup_actions(&self) {
        debug!("setup_actions");

        let action = gio::SimpleAction::new("mysql-connection-test", None);
        action.set_enabled(false);
        action.connect_activate(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_action, _target| {
                debug!("mysql-connection-test activated");

                let db = window.entry_db.text().to_string();
                let host = window.entry_host.text().to_string();
                let port = window.entry_port.value_as_int() as u32;
                let user = window.entry_user.text().to_string();
                let pw = window.entry_pw.text().to_string();

                window.test_connection_details(&db, &host, port, &user, &pw);
            }
        ));
        self.actions.add_action(&action);

        let action = gio::SimpleAction::new("mysql-connection-save", None);
        action.set_enabled(false);
        action.connect_activate(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_action, _target| {
                debug!("mysql-connection-save activated");

                let name = window.entry_name.text().to_string();
                let db = window.entry_db.text().to_string();
                let host = window.entry_host.text().to_string();
                let port = window.entry_port.value_as_int() as u16;
                let user = window.entry_user.text().to_string();
                let pw = window.entry_pw.text().to_string();

                glib::MainContext::default().spawn_local(async move {
                    let cm = crate::db::get_connection_manager().await;
                    let name_clone = name.clone();

                    let handle = get_runtime().spawn(async move {
                        cm.add_connection(
                            &name_clone,
                            &db.clone(),
                            &host.clone(),
                            port,
                            &user.clone(),
                            &pw.clone(),
                        )
                        .await
                    });

                    match handle.await {
                        Err(e) => {
                            error!("unable to add connection: {}", e);
                        }
                        Ok(result) => match result {
                            Err(e) => {
                                error!("unable to add connection: {}", e);
                            }
                            Ok(pool) => {
                                let boxed: Arc<dyn Node> = Arc::new(MySQLDataSourceNode::new(
                                    &name.clone().as_str(),
                                    &pool,
                                ));

                                let _ = window
                                    .sender
                                    .borrow()
                                    .as_ref()
                                    .expect("//todo sender")
                                    .send(ApplicationMessage::DataSourceAdd(boxed))
                                    .await;
                            }
                        },
                    }
                });
            }
        ));
        self.actions.add_action(&action);

        self.obj()
            .insert_action_group("editor", Some(&self.actions));
    }

    pub fn set_is_dirty(&mut self, value: bool) {
        self.is_dirty = value;
        self.btn_save
            .action_set_enabled("win.data-source-save::mysql", self.is_dirty);
    }

    fn test_connection_details(&self, db: &str, host: &str, port: u32, user: &str, pw: &str) {
        let db = db.to_string();
        let host = host.to_string();
        // let port = port;
        let user = user.to_string();
        let pw = pw.to_string();

        if let Some(action) = self
            .lookup_action("mysql-connection-test")
            .and_then(|a| a.downcast::<gio::SimpleAction>().ok())
        {
            let handle = get_runtime().spawn(async move {
                return attempt_connection(&db, &host, port, &user, &pw).await;
            });

            glib::MainContext::default().spawn_local(glib::clone!(
                #[weak(rename_to = this)]
                self,
                async move {
                    action.set_enabled(false);

                    // handle.await returns a Result<Result<(), sqlx::Error>, JoinError>
                    match handle.await {
                        Err(e) => {
                            error!("error when connecting to db: {}", e);
                            this.label_test.set_text(format!("{}", e).as_str());
                            this.label_test.set_css_classes(&["error"]);
                        }
                        Ok(join_result) => match join_result {
                            Err(e) => {
                                error!("error when connecting to db: {}", e);
                                this.label_test.set_text(format!("{}", e).as_str());
                                this.label_test.set_css_classes(&["error"]);
                            }
                            Ok(_) => {
                                this.label_test
                                    .set_text("Successfully connected to database");
                                this.label_test.set_css_classes(&["success"]);
                            }
                        },
                    }
                    action.set_enabled(true);
                },
            ));
        } else {
            error!("action connection-test not found");
        }
    }

    pub fn setup_handlers(&self) {
        debug!("setup_handlers");

        let entry_name_valid = std::rc::Rc::new(std::cell::Cell::new(false));
        let entry_db_valid = std::rc::Rc::new(std::cell::Cell::new(false));
        let entry_host_valid = std::rc::Rc::new(std::cell::Cell::new(false));
        let entry_port_valid = std::rc::Rc::new(std::cell::Cell::new(true));
        let entry_user_valid = std::rc::Rc::new(std::cell::Cell::new(false));
        let entry_password_valid = std::rc::Rc::new(std::cell::Cell::new(false));

        let action_test = self
            .lookup_action("mysql-connection-test")
            .and_then(|a| a.downcast::<gio::SimpleAction>().ok());

        let action_save = self
            .lookup_action("mysql-connection-save")
            .and_then(|a| a.downcast::<gio::SimpleAction>().ok());

        let action_toggle = {
            let en_valid = entry_name_valid.clone();
            let ed_valid = entry_db_valid.clone();
            let eh_valid = entry_host_valid.clone();
            let ep_valid = entry_port_valid.clone();
            let eu_valid = entry_user_valid.clone();
            let epw_valid = entry_password_valid.clone();
            move || {
                if let Some(action_test) = action_test.clone()
                    && let Some(action_save) = action_save.clone()
                {
                    if en_valid.get()
                        && ed_valid.get()
                        && eh_valid.get()
                        && ep_valid.get()
                        && eu_valid.get()
                        && epw_valid.get()
                    {
                        // debug!("action_test enabled");

                        action_test.set_enabled(true);
                        action_save.set_enabled(true);
                    } else {
                        // debug!("action_test disabled");

                        action_test.set_enabled(false);
                        action_save.set_enabled(false);
                    }
                } else {
                    error!("action not found action_toggle");
                }
            }
        };

        let action_toggle_cloned = action_toggle.clone();
        let entry_name_valid_cloned = entry_name_valid.clone();
        self.entry_name.connect_changed(move |entry| {
            let text = entry.text();
            if text.is_empty() {
                entry_name_valid_cloned.set(false);
                entry.set_icon_from_icon_name(
                    gtk::EntryIconPosition::Secondary,
                    Some("x-round-red"),
                );
                entry.add_css_class("error");
            } else {
                entry_name_valid_cloned.set(true);
                entry.set_icon_from_icon_name(
                    gtk::EntryIconPosition::Secondary,
                    Some("check-round-green"),
                );
                entry.remove_css_class("error");
            }

            action_toggle_cloned();
        });

        let action_toggle_cloned = action_toggle.clone();
        let entry_db_valid_cloned = entry_db_valid.clone();
        self.entry_db.connect_changed(move |entry| {
            let text = entry.text();
            if text.is_empty() {
                entry_db_valid_cloned.set(false);
                entry.set_icon_from_icon_name(
                    gtk::EntryIconPosition::Secondary,
                    Some("x-round-red"),
                );
                entry.add_css_class("error");
            } else {
                entry_db_valid_cloned.set(true);
                entry.set_icon_from_icon_name(
                    gtk::EntryIconPosition::Secondary,
                    Some("check-round-green"),
                );
                entry.remove_css_class("error");
            }

            action_toggle_cloned();
        });

        let action_toggle_cloned = action_toggle.clone();
        let entry_host_valid_cloned = entry_host_valid.clone();
        self.entry_host.connect_changed(move |entry| {
            let text = entry.text();
            if text.is_empty() {
                entry_host_valid_cloned.set(false);
                entry.set_icon_from_icon_name(
                    gtk::EntryIconPosition::Secondary,
                    Some("x-round-red"),
                );
                entry.add_css_class("error");
            } else {
                entry_host_valid_cloned.set(true);
                entry.set_icon_from_icon_name(
                    gtk::EntryIconPosition::Secondary,
                    Some("check-round-green"),
                );
                entry.remove_css_class("error");
            }

            action_toggle_cloned();
        });

        let action_toggle_cloned = action_toggle.clone();
        let entry_port_valid_cloned = entry_port_valid.clone();
        self.entry_port.connect_changed(move |entry| {
            let text = entry.text();
            if text.is_empty() {
                entry_port_valid_cloned.set(false);
                entry.add_css_class("error");
            } else {
                entry_port_valid_cloned.set(true);
                entry.remove_css_class("error");
            }

            action_toggle_cloned();
        });

        let action_toggle_cloned = action_toggle.clone();
        let entry_user_valid_cloned = entry_user_valid.clone();
        self.entry_user.connect_changed(move |entry| {
            let text = entry.text();
            if text.is_empty() {
                entry_user_valid_cloned.set(false);
                entry.set_icon_from_icon_name(
                    gtk::EntryIconPosition::Secondary,
                    Some("x-round-red"),
                );
                entry.add_css_class("error");
            } else {
                entry_user_valid_cloned.set(true);
                entry.set_icon_from_icon_name(
                    gtk::EntryIconPosition::Secondary,
                    Some("check-round-green"),
                );
                entry.remove_css_class("error");
            }

            action_toggle_cloned();
        });

        let action_toggle_cloned = action_toggle.clone();
        let entry_password_valid_cloned = entry_password_valid.clone();
        self.entry_pw.connect_changed(move |entry| {
            let text = entry.text();
            if text.is_empty() {
                entry_password_valid_cloned.set(false);
                // entry.set_icon_from_icon_name(gtk::EntryIconPosition::Secondary, Some("error"));
                entry.add_css_class("error");
            } else {
                entry_password_valid_cloned.set(true);
                // entry.set_icon_from_icon_name(gtk::EntryIconPosition::Secondary, Some("check"));
                entry.remove_css_class("error");
            }

            action_toggle_cloned();
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
    let uri = format!("mysql://{user}:{pw}@{host}:{port}/{db}");
    match MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&uri)
        .await
    {
        Err(e) => {
            error!("unable to connect to database: {} [{uri}]", e);
            Err(e)
        }
        Ok(_) => Ok(()),
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MySQLConnectionEditor {
    const NAME: &'static str = "MySQLConnectionEditor";
    type Type = super::MySQLConnectionEditor;
    type ParentType = gtk::Box;
}

impl ObjectImpl for MySQLConnectionEditor {
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
        self.entry_port.set_value(3306 as f64);

        self.entry_user.set_tooltip_text(Some("User"));
        self.entry_user.set_placeholder_text(Some("User"));

        self.entry_pw.set_tooltip_text(Some("Password"));
        self.entry_pw.set_placeholder_text(Some("Password"));

        self.entry_uri
            .set_tooltip_text(Some("Uniform Resource Identifier"));
        self.entry_uri
            .set_placeholder_text(Some("mysql://user:[password]@host:port/database"));
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

                let uri = format!("mysql://{user}:[password]@{host}:{port}/{db}");
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
        self.btn_save
            .set_action_name(Some("editor.mysql-connection-save"));
        // self.btn_save.connect_clicked(glib::clone!(
        //     #[weak(rename_to = editor)]
        //     self,
        //     move |_button| {
        //         debug!("//todo save button clicked");

        //         let boxed: Arc<dyn Node> = Arc::new(MySQLDataSourceNode::new(
        //             editor.entry_name.text().as_str(),
        //             ConnectionSettings {
        //                 host: editor.entry_host.text().to_string(),
        //                 port: editor.entry_port.value() as u32,
        //                 user: editor.entry_user.text().to_string(),
        //                 pw: editor.entry_pw.text().to_string(),
        //                 name: editor.entry_db.text().to_string(),
        //             },
        //         ));
        //         let _ = editor
        //             .sender
        //             .borrow()
        //             .as_ref()
        //             .expect("//todo sender")
        //             // .send(ApplicationMessage::DataSourceAdd(boxed));
        //             .send_blocking(ApplicationMessage::DataSourceAdd(boxed));
        //     }
        // ));

        self.btn_test.set_icon_name("connect");
        self.btn_test.set_tooltip_text(Some("Test connection"));
        self.btn_test
            .set_action_name(Some("editor.mysql-connection-test"));
        // self.btn_test.connect_clicked(|_button| {
        //     debug!("//todo test button clicked");
        // });

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
        // self.label_test.set_vexpand(true);
        self.label_test.set_wrap(true);
        grid.attach(&self.label_test, 0, 7, 2, 1);

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

        self.setup_actions();
        self.setup_handlers();
    }
}

impl WidgetImpl for MySQLConnectionEditor {}

impl BoxImpl for MySQLConnectionEditor {}

impl ActionMapImpl for MySQLConnectionEditor {
    fn add_action(&self, action: &gio::Action) {
        self.actions.add_action(action);
    }

    fn remove_action(&self, action_name: &str) {
        self.actions.remove_action(action_name);
    }

    fn lookup_action(&self, name: &str) -> Option<gio::Action> {
        self.actions.lookup_action(name)
    }
}
