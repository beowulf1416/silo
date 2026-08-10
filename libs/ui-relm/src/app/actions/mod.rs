use relm4::{actions::*, main_application, prelude::*};

relm4::new_action_group!(pub ApplicationActionGroup, "app");
relm4::new_action_group!(pub(super) WindowActionGroup, "win");

relm4::new_stateless_action!(pub QuitAction, ApplicationActionGroup, "quit");

relm4::new_stateless_action!(pub DataStoreAddAction, ApplicationActionGroup, "data-store-add");
relm4::new_stateless_action!(pub DataSourceAddPostgresAction, ApplicationActionGroup, "data-store-add-postgres");
