pub type BotError = Box<dyn std::error::Error + Send + Sync>;

pub type BotResult<T> = std::result::Result<T, BotError>;
