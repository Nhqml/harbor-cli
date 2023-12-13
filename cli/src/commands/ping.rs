use clap::Parser;

use api_client::apis::{configuration::Configuration, ping_api};

use super::AnyResult;

#[derive(Parser, Debug)]
pub(crate) struct PingCommand {}

impl PingCommand {
    pub(crate) async fn run(&self, configuration: &Configuration) -> AnyResult<()> {
        let pong = ping_api::get_ping(configuration, None).await?;
        println!("{}", pong);
        Ok(())
    }
}
