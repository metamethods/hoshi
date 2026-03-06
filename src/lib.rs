#[macro_use]
pub mod macros;

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en-US");

use twilight_model::{guild::Role, user::User};

pub mod localization;

pub mod components;
pub mod context;
pub mod error;
pub mod interaction;
pub mod resolver;
pub mod response;

pub mod commands;
pub mod events;

pub enum Mentionable<'a> {
    User(&'a User),
    Role(&'a Role),
}
