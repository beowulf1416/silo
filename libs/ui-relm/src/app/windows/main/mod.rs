use std::convert::identity;

use gtk::AccessibleRole::Main;
use tracing::debug;

// use gtk::{Application, glib};
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

// use crate::app::data_store_window::DataStoreWindow;
// use crate::app::components::data_store_view::DataStoreView;
use crate::app::components::data_source_view::DataSourceView;

use crate::app::actions::WindowActionGroup;
// relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
// relm4::new_stateless_action!(QuitAction, WindowActionGroup, "quit");
use crate::app::actions::QuitAction;

// relm4::new_stateless_action!(DataStoreAddAction, WindowActionGroup, "data-store-add");
// relm4::new_stateless_action!(
//     pub(super) DataSourceAddPostgresAction,
//     WindowActionGroup,
//     "data-source-add-postgres"
// );

#[derive(Debug)]
pub enum MainWindowMsg {
    Quit,
}

#[derive(Debug)]
pub struct MainWindow {
    dsv: Controller<DataSourceView>,
}

#[relm4::component(pub)]
impl SimpleComponent for MainWindow {
    type Init = ();
    type Input = MainWindowMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About GTK Rust Template" => AboutAction,
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
                    set_vexpand: true,

                    #[wrap(Some)]
                    set_start_child = &gtk::Box {
                        append = model.dsv.widget(),
                    },

                    #[wrap(Some)]
                    set_end_child = &gtk::Label {
                        set_label: "right",
                    },
                }
            }

        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let dsv = DataSourceView::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let model = Self { dsv: dsv };
        let widgets = view_output!();

        let app = root.application().unwrap();
        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        let shortcuts_action = {
            RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                // ShortcutsDialog::builder().launch(()).detach();
                debug!("shortcuts action");
            })
        };

        let about_action = {
            RelmAction::<AboutAction>::new_stateless(move |_| {
                // AboutDialog::builder().launch(()).detach();
                debug!("about action");
            })
        };

        let quit_action = {
            RelmAction::<QuitAction>::new_stateless(move |_| {
                sender.input(MainWindowMsg::Quit);
            })
        };

        // let data_store_add_action = {
        //     RelmAction::<DataStoreAddAction>::new_stateless(move |_| {
        //         debug!("data store add action");
        //     })
        // };

        // let data_source_add_postgres_action = {
        //     RelmAction::<DataSourceAddPostgresAction>::new_stateless(move |_| {
        //         debug!("data source add postgres action");
        //     })
        // };

        // Connect action with hotkeys
        app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

        actions.add_action(shortcuts_action);
        actions.add_action(about_action);
        actions.add_action(quit_action);
        // actions.add_action(data_store_add_action);
        // actions.add_action(data_source_add_postgres_action);
        actions.register_for_widget(&widgets.main_window);

        // widgets.load_window_size();

        return ComponentParts { model, widgets };
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            // MainWindowMsg::Quit => main_application().quit(),
            MainWindowMsg::Quit => {
                relm4::main_adw_application().activate_action("quit", None);
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
