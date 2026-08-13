use tracing::debug;

use gtk::prelude::*;
use gtk::prelude::{
    ActionableExt, ApplicationExt, ButtonExt, GtkWindowExt, OrientableExt, SettingsExt, WidgetExt,
};
use gtk::{gio, glib};

use relm4::{
    Component, ComponentParts, ComponentSender, SimpleComponent,
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    adw, gtk, main_application,
    prelude::*,
};

pub enum DataSourceDialogInputMsg {}

pub struct DataSourceDialog {}

#[relm4::component(pub)]
impl SimpleComponent for DataSourceDialog {
    type Init = ();
    type Input = DataSourceDialogInputMsg;
    type Output = MainWindowMsg;
    type Widgets = AppWidgets;
}
