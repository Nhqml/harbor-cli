mod health;
mod ping;

pub(crate) use anyhow::Result as AnyResult;
pub(crate) use {health::HealthCommand, ping::PingCommand};
