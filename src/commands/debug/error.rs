use twilight_model::application::command::{Command, CommandType};
use twilight_util::builder::command::CommandBuilder;

use crate::{
    commands::{ALL_CONTEXTS, ALL_INTEGRATIONS},
    error::BotResult,
    response::BotResponse,
};

pub fn schema() -> Command {
    CommandBuilder::new("error", "error", CommandType::ChatInput)
        .integration_types(ALL_INTEGRATIONS)
        .contexts(ALL_CONTEXTS)
        .build()
}

pub async fn command() -> BotResult<Option<BotResponse>> {
    Err("Hello World!")?;
    Ok(None)
}
