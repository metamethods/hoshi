use std::borrow::Cow;

use twilight_model::http::interaction::InteractionResponseData;

#[derive(Debug)]
pub enum BotError {
    Response(InteractionResponseData),
    Message(String),
    String(String),
    Any(Box<dyn std::error::Error + Send + Sync>),
}

impl From<String> for BotError {
    fn from(value: String) -> Self {
        BotError::String(value)
    }
}

impl From<&str> for BotError {
    fn from(value: &str) -> Self {
        BotError::String(value.into())
    }
}

impl<'a> From<Cow<'a, str>> for BotError {
    fn from(value: Cow<'a, str>) -> Self {
        BotError::String(value.into())
    }
}

impl From<std::io::Error> for BotError {
    fn from(value: std::io::Error) -> Self {
        BotError::Any(value.into())
    }
}

impl From<reqwest::Error> for BotError {
    fn from(value: reqwest::Error) -> Self {
        BotError::Any(value.into())
    }
}

impl From<twilight_http::Error> for BotError {
    fn from(value: twilight_http::Error) -> Self {
        BotError::Any(value.into())
    }
}

impl From<ril::Error> for BotError {
    fn from(value: ril::Error) -> Self {
        BotError::Any(value.into())
    }
}

impl From<dotenvy::Error> for BotError {
    fn from(value: dotenvy::Error) -> Self {
        BotError::Any(value.into())
    }
}

impl From<std::env::VarError> for BotError {
    fn from(value: std::env::VarError) -> Self {
        BotError::Any(value.into())
    }
}

impl From<twilight_http::response::DeserializeBodyError> for BotError {
    fn from(value: twilight_http::response::DeserializeBodyError) -> Self {
        BotError::Any(value.into())
    }
}

impl From<twilight_gateway::error::StartRecommendedError> for BotError {
    fn from(value: twilight_gateway::error::StartRecommendedError) -> Self {
        BotError::Any(value.into())
    }
}

impl From<shell_words::ParseError> for BotError {
    fn from(value: shell_words::ParseError) -> Self {
        BotError::Any(value.into())
    }
}

impl From<std::string::FromUtf8Error> for BotError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        BotError::Any(value.into())
    }
}
