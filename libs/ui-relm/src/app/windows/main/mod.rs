use std::convert::identity;

use tracing::debug;

use std::collections::HashMap;
use std::rc::Rc;

// use gtk::{Application, glib};
use gtk::gio::File;
use gtk::prelude::*;
use gtk::prelude::{
    ActionableExt, ApplicationExt, ButtonExt, GtkWindowExt, OrientableExt, SettingsExt, WidgetExt,
};
use gtk::{gio, glib};

use crate::{APP_ID, PROFILE};
// use relm4::gtk4::prelude::ActionableExt;
use relm4::{
    Component, ComponentParts, ComponentSender, SimpleComponent,
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    adw, gtk, main_application,
    prelude::*,
};

use crate::app::components::data_source_view::DataSourceView;
use crate::app::components::editor_view::{
    EditorView,
    editor::{EditorPlugin, TextEditorPlugin},
};
use crate::app::models::data_source::DataSourcePlugin;
use crate::app::models::plugins::postgres::PostgresDataSourcePlugin;
// use crate::app::components::data_source_dialog::DataSourceDialog;

use crate::app::actions::*;

type FilePath = String;

#[derive(Debug)]
pub enum MainWindowMsg {
    Quit,
    WorkspaceChanged(FilePath),
    NewDataSourcePostgres,
}

#[derive(Debug)]
pub struct MainWindow {
    silo: silo_base::Silo,
    window: adw::ApplicationWindow,
    dsv: Controller<DataSourceView>,
    ed: Controller<EditorView>,
    data_source_plugins: HashMap<String, Box<dyn DataSourcePlugin>>,
    editor_plugins: Vec<Box<dyn EditorPlugin>>,
}

impl MainWindow {
    pub fn register_data_source_plugins() -> HashMap<String, Box<dyn DataSourcePlugin>> {
        let mut plugins: HashMap<String, Box<dyn DataSourcePlugin>> = HashMap::new();

        let plugin: Box<dyn DataSourcePlugin> = Box::new(PostgresDataSourcePlugin::new());
        plugins.insert("postgres".to_string(), plugin);

        return plugins;
    }

    pub fn register_editor_plugins() -> Vec<Box<dyn EditorPlugin>> {
        let mut plugins: Vec<Box<dyn EditorPlugin>> = Vec::new();

        let plugin: Box<dyn EditorPlugin> = Box::new(TextEditorPlugin::new());
        plugins.push(plugin);

        return plugins;
    }
}

#[relm4::component(pub)]
impl SimpleComponent for MainWindow {
    type Init = silo_base::Silo;
    type Input = MainWindowMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            "_File" {
                section! {
                    "_New Workspace" => NewWorkspaceAction,
                }
            },
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About GTK Rust Template" => AboutAction,
            },
            section! {
                "_Quit" => QuitAction,
            }
        }
    }

    view! {
        main_window = adw::ApplicationWindow::new(&main_application()) {
            set_visible: true,
            set_default_size: (800, 600),

            connect_close_request[sender] => move |_| {
                sender.input(MainWindowMsg::Quit);
                glib::Propagation::Stop
            },

            add_css_class?: if PROFILE == "Devel" {
                    Some("devel")
                } else {
                    None
                },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    pack_start = &gtk::MenuButton {
                        // set_icon_name: "open-menu-symbolic",
                        set_icon_name: "silo",
                        set_menu_model: Some(&primary_menu),
                    },
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu),
                    }
                },

                gtk::ActionBar {
                    set_hexpand: true,

                    pack_start = &gtk::Button {
                        set_label: "Open",
                        set_icon_name: "document-open",
                        set_action_name: Some("win.database-new")
                    },
                    pack_start = &gtk::Button {
                        set_label: "Save",
                        set_icon_name: "document-save",
                        set_action_name: Some("win.database-new")
                    }
                },

                gtk::Paned {
                    set_orientation: gtk::Orientation::Horizontal,
                    add_css_class: "paned",
                    set_hexpand: true,
                    set_vexpand: true,
                    set_width_request: 200,
                    set_resize_start_child: false,
                    set_shrink_start_child: false,

                    #[wrap(Some)]
                    set_start_child = &gtk::Box {
                        set_hexpand: true,
                        set_vexpand: true,
                        append = model.dsv.widget(),
                    },

                    #[wrap(Some)]
                    set_end_child = &gtk::Box {
                        set_hexpand: true,
                        set_vexpand: true,
                        append = model.ed.widget(),
                    },
                }
            }

        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let data_source_plugins = Self::register_data_source_plugins();
        let editor_plugins = Self::register_editor_plugins();

        let dsv = DataSourceView::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let ed = EditorView::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let model = Self {
            silo: init,
            window: root.clone(),
            dsv: dsv,
            ed: ed,
            data_source_plugins: data_source_plugins,
            editor_plugins: editor_plugins,
        };
        let widgets = view_output!();

        let app = root.application().unwrap();
        // let root_clone = root.clone();
        // let window = root_clone.upcast_ref();

        let rc_sender = Rc::new(sender);

        let mut actions = RelmActionGroup::<ApplicationActionGroup>::new();

        let preferences_action = preferences_action(rc_sender.clone());
        actions.add_action(preferences_action);

        let about_action = about_action(rc_sender.clone());
        actions.add_action(about_action);

        let shortcuts_action = shortcuts_action(rc_sender.clone());
        actions.add_action(shortcuts_action);

        let quit_action = quit_action(rc_sender.clone());
        actions.add_action(quit_action);
        app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

        let new_workspace_action = new_workspace_action(rc_sender.clone(), model.window.clone());
        actions.add_action(new_workspace_action);

        let new_data_source_action =
            new_data_source_postgres_action(rc_sender.clone(), model.window.clone());
        actions.add_action(new_data_source_action);

        // Connect action with hotkeys
        app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

        // actions.add_action(data_store_add_action);
        // actions.add_action(data_source_add_postgres_action);
        actions.register_for_widget(&widgets.main_window);

        // widgets.load_window_size();

        return ComponentParts { model, widgets };
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            MainWindowMsg::Quit => {
                debug!("quit action triggered");
                // need to disambiguate between widgetext and actiongroupexit
                // self.window.activate_action("app.quit", None).unwrap();
                gtk::prelude::WidgetExt::activate_action(&self.window, "app.quit", None).unwrap();
            }
            MainWindowMsg::WorkspaceChanged(path) => {
                debug!("workspace changed: {:?}", path);
                self.silo.set_workspace(path);
            }
            MainWindowMsg::NewDataSourcePostgres => {
                debug!("new data source postgres action triggered");
                let plugin: &Box<dyn DataSourcePlugin> =
                    self.data_source_plugins.get("postgres").unwrap();
                debug!("data source postgres plugin: {:?}", plugin.name());
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        // widgets.save_window_size().unwrap();
        debug!("//todo shutdown");
    }
}

impl AppWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = gio::Settings::new(APP_ID);

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}
