use clap::{Parser, Subcommand};

use crate::commands::*;

use clap::builder::styling::{AnsiColor, Effects, Styles};

fn styles() -> Styles {
    Styles::styled()
        .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .valid(AnsiColor::BrightGreen.on_default() | Effects::BOLD)
        .invalid(AnsiColor::Red.on_default() | Effects::BOLD)
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::BrightBlue.on_default())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
#[command(disable_help_subcommand = true)]
#[command(help_template = r#"
{name} {version} - {about-with-newline}
{before-help}{usage-heading} {usage}

{all-args}{after-help}"#)]
#[command(styles = styles())]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    #[command(about = "Ping the API")]
    Ping(PingCommand),
    #[command(about = "Get the health status of Harbor")]
    Health(HealthCommand),
}

impl Commands {
    pub(crate) async fn run(
        &self,
        configuration: &api_client::apis::configuration::Configuration,
    ) -> AnyResult<()> {
        match self {
            Commands::Ping(command) => command.run(configuration).await,
            Commands::Health(command) => command.run(configuration).await,
        }
    }
}
