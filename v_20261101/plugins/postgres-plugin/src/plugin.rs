use tracing::debug;

use silo_api::plugin::Plugin;

#[derive(Debug)]
pub struct PostgresPlugin {}

impl Plugin for PostgresPlugin {
    fn name(&self) -> &str {
        "postgres"
    }

    fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("PostgresPlugin::init");

        Ok(())
    }
}
