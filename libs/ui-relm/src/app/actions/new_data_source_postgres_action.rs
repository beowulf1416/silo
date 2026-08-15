use tracing::{debug, error};

use super::*;

use gtk::prelude::*;
use relm4::{actions::*, prelude::*};
use std::rc::Rc;

relm4::new_stateless_action!(pub NewDataSourcePostGresAction, ApplicationActionGroup, "new-data-source-action-postgres");

pub fn new_data_source_postgres_action(
    sender: Rc<ComponentSender<MainWindow>>,
    parent: adw::ApplicationWindow,
) -> RelmAction<NewDataSourcePostGresAction> {
    let window = parent.upcast::<gtk::Window>();

    return RelmAction::<NewDataSourcePostGresAction>::new_stateless(move |_| {
        debug!("new_data_source_postgres_action");
        if let Err(e) = sender
            .input_sender()
            .send(MainWindowMsg::NewDataSourcePostgres)
        {
            error!("failed to send new data source postgres message");
        }
    });
}
