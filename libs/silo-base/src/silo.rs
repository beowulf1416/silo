use tracing::{debug, error};

use std::collections::HashMap;
use std::io::BufWriter;
use std::path::Path;
use std::{fs::File, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::connection::Connection;
use silo_plugin::plugin::{Plugin, PluginRegistry};

#[derive(Debug, Serialize, Deserialize)]
pub struct Silo {
    workspace_path: String,
    connections: Vec<Connection>,

    #[serde(skip_serializing, skip_deserializing)]
    plugins: PluginRegistry,
}

impl Silo {
    pub fn new(workspace_path: Option<String>) -> Self {
        // register plugins
        debug!("silo::new()");

        let path = if workspace_path.is_none() {
            if let Ok(cd) = std::env::current_dir() {
                cd.to_string_lossy().to_string()
            } else {
                "".to_string()
            }
        } else {
            workspace_path.unwrap()
        };

        let plugins = PluginRegistry::new();
        // plugins.register("postgres", factory);

        return Self {
            workspace_path: path,
            connections: Vec::new(),
            plugins: plugins,
        };
    }

    pub fn set_workspace(&mut self, path: String) -> Result<(), &'static str> {
        debug!("set_workspace path: {:?}", path);
        self.workspace_path = path;

        // check if config exists, if not create it
        let config_file_path = Path::new(&self.workspace_path).join("connections.conf");
        if !config_file_path.exists() {
            match File::create(&config_file_path) {
                Err(e) => {
                    error!("Failed to create config file: {}", e);
                    return Err("Failed to create config file");
                }
                Ok(file) => {
                    let writer = BufWriter::new(file);
                    if let Err(e) = serde_json::to_writer(writer, &self.connections) {
                        error!("Failed to write config file: {}", e);
                        return Err("Failed to write config file");
                    }
                }
            }
        }

        return Ok(());
    }
}

impl Default for Silo {
    fn default() -> Self {
        return Silo::new(None);
    }
}
