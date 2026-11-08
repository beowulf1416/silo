mod config;
mod plugin_manager;
pub mod silo;

use silo_api::plugin::Plugin;
// use silo_api::{AppWindowMessages, data_sources};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
// use std::sync::mpsc::Sender;
use async_channel::Sender;
use thiserror::Error;
use tracing::{debug, error, info};

// use silo_api::connection::Connection;
use silo_api::{
    AppWindowMessages, PasswordDialogResult, Silo, SiloError, data_sources::DataSource,
    plugin::PluginManager, window::MainWindow,
};

// use crate::config::DataSourceConfig;
use crate::plugin_manager::PluginManagerImpl;

pub struct ConnectionData {}

// #[derive(Error, Debug)]
// pub enum SiloError {
//     #[error("data source file does not exist")]
//     DataSourceFileMissing,
// }

#[derive(Debug)]
pub struct SiloImpl {
    pub workspace_path: RwLock<String>,
    pub data_sources: RwLock<Vec<DataSource>>,
    // pub connections: HashMap<String, Box<dyn Connection>>,
    pub plugin_mgr: Arc<dyn PluginManager>,
    // pub window: RwLock<Option<Arc<dyn MainWindow>>>,
    pub sender: RwLock<Option<Sender<AppWindowMessages>>>,
}

impl SiloImpl {
    pub fn new() -> Self {
        info!("SiloImpl::new()");

        let mut pm = PluginManagerImpl::new();

        let plugin = postgres_plugin::PostgresPlugin::new();

        let _result = pm.register(Arc::new(plugin));

        return SiloImpl {
            workspace_path: RwLock::new(String::from("")),
            data_sources: RwLock::new(vec![]),
            // connections: HashMap::new(),
            plugin_mgr: Arc::new(pm),
            // window: RwLock::new(None),
            sender: RwLock::new(None),
        };
    }

    fn load_data_sources(&self) {
        info!("SiloImpl::load_data_sources");

        if let Ok(rw_path) = self.workspace_path.read() {
            let path = rw_path;
            // .clone()
            // .expect("unable to retrieve workspace path");
            // .unwrap();
            let config_file = format!("{}/data_sources.json", path);
            let path = Path::new(config_file.as_str());

            match path.try_exists() {
                Err(e) => {
                    error!("data source file error: {} {:?}", config_file, e);
                }
                Ok(false) => {
                    error!("data source file does not exist {}", config_file);
                }
                Ok(true) => {
                    // parse file
                    if let Ok(rw_ds) = self.data_sources.write() {
                        let mut ds = rw_ds;
                        // .clone().expect("unable to access data sources");
                        *ds = self.load_data_source_config(&config_file);
                    } else {
                        error!("unable to obtain write access to data sources");
                    }
                }
            }
        } else {
            error!("unable to read workspace path");
        }
    }

    fn load_data_source_config(&self, file_name: &str) -> Vec<DataSource> {
        info!("SiloImpl::load_data_source_config");

        if let Ok(file) = std::fs::File::open(file_name) {
            let rdr = std::io::BufReader::new(file);
            if let Ok(sources) = serde_json::from_reader::<_, Vec<DataSource>>(rdr) {
                debug!("sources: {:?}", sources);
                return sources;
            } else {
                error!("unable to parse config file {}", file_name);
            }
        } else {
            error!("unable to load config file {}", file_name);
        }

        vec![]
    }

    // fn to_Arc(&self) -> Arc<dyn Silo> {
    //     let silo: Arc<dyn Silo> = Arc::new(self);
    //     return silo;
    // }

    fn initialize_plugins(&self) {
        info!("SiloImpl::initialize_plugins()");

        if let Ok(sources) = self.data_sources.read() {
            // self.plugin_mgr.initialize_plugins(self.to_Arc(), &sources);
        } else {
            error!("SiloImpl::initialize(): unable to read data sources");
        }
    }
}

#[async_trait::async_trait]
impl Silo for SiloImpl {
    fn initialize(&self) {
        info!("SiloImpl::initialize");
    }

    fn set_workspace_path(&mut self, path: &str) {
        info!("SiloImpl::set_workspace_path");
        // self.workspace_path.replace(Some(String::from(path)));
        if let Ok(rw_path) = self.workspace_path.write() {
            let mut workspace_path = rw_path;
            // .clone()
            // .expect("unable to obtain write access to workspace path");
            *workspace_path = String::from(path);
        } else {
            error!("unable to set workspace path");
        }
        self.load_data_sources();
        self.initialize_plugins();
    }

    //     fn set_sender(&self, sender: Sender<AppWindowMessages>) {
    //         info!("SiloImpl::set_sender()");
    //
    //         if let Ok(rw_sender) = self.sender.write() {
    //             let mut self_sender = rw_sender;
    //             *self_sender = Some(sender);
    //         } else {
    //             error!("SiloImpl::set_sender() unable to obtain write access to sender");
    //         }
    //     }

    //     fn set_window(&self, window: Arc<dyn MainWindow>) {
    //         info!("SiloImpl::set_window()");
    //
    //         if let Ok(mut self_window) = self.window.write() {
    //             *self_window = Some(window);
    //         } else {
    //             error!("SiloImpl::set_window(): unable to obtain write access to window property");
    //         }
    //     }

    fn sources(&self) -> Vec<DataSource> {
        // return self.data_sources.clone().into_inner().unwrap();
        if let Ok(rw_ds) = self.data_sources.read() {
            let ds = rw_ds.clone();
            // .clone().expect("unable to read data sources");
            return ds;
        } else {
            error!("unable to obtain read access to data sources");
            return vec![];
        }
    }

    fn plugin_manager(&self) -> Arc<dyn PluginManager> {
        info!("SiloImpl::plugin_manager()");
        return self.plugin_mgr.clone();
    }

    //     fn show_password_dialog(&self, sender: Sender<AppWindowMessages>) -> String {
    //         info!("SiloImpl::show_password_dialog()");
    //
    //         if let Ok(rw_sender) = self.sender.read() {
    //             let self_sender = rw_sender.clone();
    //             if let Some(o_sender) = self_sender {
    //                 let _result = o_sender.send(AppWindowMessages::ShowPasswordDialog);
    //             } else {
    //                 error!("SiloImpl::show_password_dialog() sender not set");
    //             }
    //         } else {
    //             error!("SiloImpl::show_password_dialog() unable to obtain read access to sender");
    //         }
    //
    //         return String::from("");
    //     }
}
