mod cli;
mod commands;

use api_client::apis::configuration::Configuration;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let conf = Configuration::new();

    match cli.command.run(&conf).await {
        Ok(_) => {}
        Err(error) => println!("{}", error),
    }
}
