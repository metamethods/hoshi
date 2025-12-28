#[macro_use]
extern crate rust_i18n;

#[macro_use]
mod macros;

i18n!("locales", fallback = "en-US");

use std::{collections::HashMap, error::Error, process::Output};

use bytes::Bytes;
use include_dir::{Dir, include_dir};
use reqwest::Client as ReqwestClient;
use twilight_model::{
    application::interaction::InteractionContextType, channel::Attachment,
    oauth::ApplicationIntegrationType,
};

pub mod autocompletes;
pub mod commands;
pub mod context;
pub mod event;
pub mod interaction;
pub mod session;

pub static ASSETS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");

pub const ALL_CONTEXTS: [InteractionContextType; 3] = [
    InteractionContextType::Guild,
    InteractionContextType::BotDm,
    InteractionContextType::PrivateChannel,
];

pub const ALL_INTEGRATIONS: [ApplicationIntegrationType; 2] = [
    ApplicationIntegrationType::GuildInstall,
    ApplicationIntegrationType::UserInstall,
];

pub type BotResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub fn get_localizations_of<Key: AsRef<str>>(key: Key) -> HashMap<String, String> {
    available_locales!()
        .iter()
        .map(|locale| (locale.to_string(), t!(key.as_ref(), locale = locale).into()))
        .collect()
}

pub fn get_output_as_buffer(output: Output) -> Vec<u8> {
    let mut buffer = vec![];
    buffer.extend_from_slice(&output.stdout);
    if !buffer.is_empty() {
        buffer.push(b'\n');
    }
    buffer.extend_from_slice(&output.stderr);
    buffer
}

pub async fn download_attachment(
    attachment: &Attachment,
    reqwest_client: &ReqwestClient,
) -> Result<Bytes, Box<dyn Error + Send + Sync>> {
    Ok(reqwest_client
        .get(attachment.url.as_str())
        .send()
        .await?
        .bytes()
        .await?)
}
