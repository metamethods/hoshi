use std::fmt::Display;

use twilight_model::http::interaction::InteractionResponseData;

#[derive(Debug)]
pub enum BotError {
    Response(InteractionResponseData),
    Message(String),
    String(String),
}

impl<E: Display> From<E> for BotError {
    fn from(value: E) -> Self {
        BotError::String(value.to_string())
    }
}
