pub mod plugin;

use silo_api::plugin::{Plugin, PluginFactory};

pub fn create_plugin() -> Box<dyn Plugin> {
    return Box::new(plugin::PostgresPlugin {});
}
