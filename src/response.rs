use twilight_model::http::interaction::InteractionResponseData;

#[derive(Debug)]
pub enum BotResponse {
    ResponseData(InteractionResponseData),
    Content(String),
    Error(String),
}

impl<T: Into<String>> From<T> for BotResponse {
    fn from(value: T) -> Self {
        BotResponse::Content(value.into())
    }
}
