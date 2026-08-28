use std::collections::HashMap;

use crate::ApplicationMessage;
// use sqlx::Pool;

pub trait Plugin: std::fmt::Debug {
    fn name(&self) -> &str;

    fn build_data_source_editor_widget(
        &self,
        sender: async_channel::Sender<ApplicationMessage>,
    ) -> Option<gtk::Widget>;
    // fn build_query_editor_widget(
    //     &self,
    //     sender: async_channel::Sender<ApplicationMessage>,
    // ) -> Option<gtk::Widget>;
    //

    // fn get_pool(&self) -> Result<sqlx::Pool, &'static str>;
}

pub type PluginFactory = fn() -> Box<dyn Plugin>;

#[derive(Debug, Clone)]
pub struct PluginRegistry {
    plugins: HashMap<String, PluginFactory>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        return Self {
            plugins: HashMap::new(),
        };
    }

    pub fn register(&mut self, name: &str, factory: PluginFactory) {
        self.plugins.insert(name.to_string(), factory);
    }

    pub fn registered_plugins(&self) -> Vec<String> {
        return self.plugins.keys().cloned().collect::<Vec<String>>();
    }

    pub fn create_plugin(&self, name: &str) -> Option<Box<dyn Plugin>> {
        let factory = self
            .plugins
            .get(name)
            .expect("//todo should return a factory");
        return Some(factory());
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        return Self::new();
    }
}
