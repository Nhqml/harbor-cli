use clap::Parser;

use api_client::apis::{configuration::Configuration, health_api};

use super::AnyResult;

#[derive(Parser, Debug)]
pub(crate) struct HealthCommand {}

impl HealthCommand {
    pub(crate) async fn run(&self, configuration: &Configuration) -> AnyResult<()> {
        let health = health_api::get_health(configuration, None).await?;

        for component in health.components.expect("components to be present") {
            let name = component.name.expect("component name");
            let status = component.status.expect("component status");

            if let Some(error) = component.error {
                println!("{}: {} ({})", name, status, error);
            } else {
                println!("{}: {}", name, status);
            }
        }

        Ok(())
    }
}
