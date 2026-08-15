use silo_plugin::Plugin;
use tracing::{debug, error};

use std::collections::HashMap;
use std::io::BufWriter;
use std::path::Path;
use std::{fs::File, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::connection::Connection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Silo {
    workspace_path: String,
    connections: Vec<Connection>,

    #[serde(skip_serializing, skip_deserializing)]
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl Silo {
    pub fn new() -> Self {
        // register plugins

        return Self {
            workspace_path: String::from(""),
            connections: Vec::new(),
            plugins: HashMap::new(),
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
