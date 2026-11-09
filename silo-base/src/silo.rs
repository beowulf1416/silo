use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, error, info};

use silo_api::{
    Silo,
    data_sources::DataSource,
    plugin::{Plugin, PluginManager},
};

use crate::plugin_manager::PluginManagerImpl;

#[derive(Debug, Clone)]
pub struct SiloImpl {
    pub workspace_path: String,
    pub data_sources: Vec<DataSource>,
    pub plugin_mgr: Arc<dyn PluginManager>,
}

impl SiloImpl {
    pub fn new() -> Self {
        let mut pm = PluginManagerImpl::new();

        // todo: register plugins
        let plugin = postgres_plugin::PostgresPlugin::new();
        let _result = pm.register(Arc::new(plugin));

        return Self {
            workspace_path: String::from(""),
            data_sources: vec![],
            plugin_mgr: Arc::new(pm),
        };
    }

    fn load_data_sources(&mut self) {
        info!("SiloImpl::load_data_sources()");

        let config_file = format!("{}/data_sources.json", self.workspace_path);
        let path = Path::new(config_file.as_str());

        match path.try_exists() {
            Err(e) => {
                error!(
                    "SiloImpl::load_data_sources() error checking path {:?}",
                    path
                );
            }
            Ok(false) => {
                error!("SiloImpl::load_data_sources() data sources file does not exist");
            }
            Ok(true) => {
                // parse file
                if let Ok(file) = File::open(path) {
                    let rdr = BufReader::new(file);
                    if let Ok(sources) = serde_json::from_reader::<_, Vec<DataSource>>(rdr) {
                        debug!("SiloImpl::load_data_sources() sources {:?}", sources);
                        self.data_sources = sources;
                    } else {
                        error!(
                            "SiloImpl::load_data_sources() unable to parse data sources config file"
                        );
                    }
                }
            }
        }
    }

    fn initialize_plugins(&self) {
        info!("SiloImpl::initialize_plugins()");

        let silo: Arc<dyn Silo> = Arc::new(self.clone());
        for plugin in self.plugin_mgr.plugins() {
            plugin.set_silo(silo.clone());
        }

        for s in self.data_sources.clone() {
            if let Some(plugin) = self.plugin_mgr.get_plugin(&s.plugin) {
                plugin.register_source(&s);
            } else {
                error!("SiloImpl::initialize_plugins() unknown plugin {}", s.plugin);
            }
        }
    }
}

impl Silo for SiloImpl {
    fn initialize(&self) {
        info!("SiloImpl::initialize()");
    }

    fn set_workspace_path(&mut self, path: &str) {
        self.workspace_path = String::from(path);
        self.load_data_sources();
        self.initialize_plugins();
    }

    fn sources(&self) -> Vec<DataSource> {
        return self.data_sources.clone();
    }

    fn plugin_manager(&self) -> Arc<dyn PluginManager> {
        return self.plugin_mgr.clone();
    }
}
