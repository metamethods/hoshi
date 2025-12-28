use twilight_http::{
    Response,
    client::InteractionClient,
    request::application::interaction::{CreateFollowup, UpdateResponse},
    response::marker::EmptyBody,
};
use twilight_model::{
    application::interaction::{
        Interaction, InteractionChannel, InteractionData,
        application_command::{CommandData, CommandDataOption, CommandOptionValue},
    },
    channel::{Attachment, Message, message::MessageFlags},
    guild::Role,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
    user::User,
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::BotResult;

pub enum Mentionable<'a> {
    User(&'a User),
    Role(&'a Role),
}

pub struct ApplicationInteraction<'client> {
    pub event_interaction: Interaction,
    pub interaction_client: &'client InteractionClient<'client>,
    pub deferred: bool,
}

impl<'client> ApplicationInteraction<'client> {
    pub fn new(
        event_interaction: Interaction,
        interaction_client: &'client InteractionClient<'client>,
    ) -> Self {
        Self {
            event_interaction,
            interaction_client,
            deferred: false,
        }
    }

    pub async fn create_response(
        &self,
        kind: InteractionResponseType,
        data: Option<InteractionResponseData>,
    ) -> BotResult<Response<EmptyBody>> {
        Ok(self
            .interaction_client
            .create_response(
                self.event_interaction.id,
                &self.event_interaction.token,
                &InteractionResponse { kind, data },
            )
            .await?)
    }

    pub fn update_response(&self) -> UpdateResponse<'_> {
        self.interaction_client
            .update_response(&self.event_interaction.token)
    }

    pub fn followup(&self) -> CreateFollowup<'_> {
        self.interaction_client
            .create_followup(&self.event_interaction.token)
    }

    pub async fn reply(&self, data: InteractionResponseData) -> BotResult<Response<EmptyBody>> {
        Ok(self
            .create_response(
                InteractionResponseType::ChannelMessageWithSource,
                Some(data),
            )
            .await?)
    }

    pub async fn defer_reply(
        &mut self,
        message_flags: MessageFlags,
    ) -> BotResult<Response<EmptyBody>> {
        let response = self
            .create_response(
                InteractionResponseType::DeferredChannelMessageWithSource,
                Some(
                    InteractionResponseDataBuilder::new()
                        .flags(message_flags)
                        .build(),
                ),
            )
            .await?;

        self.deferred = true;

        Ok(response)
    }

    pub async fn get_response(&self) -> BotResult<Response<Message>> {
        Ok(self
            .interaction_client
            .response(&self.event_interaction.token)
            .await?)
    }

    pub async fn is_defered(&self) -> BotResult<bool> {
        Ok(true)
    }
}

pub struct ApplicationCommandInteraction<'app, 'client> {
    pub application_interaction: &'app mut ApplicationInteraction<'client>,
    pub command_data: Box<CommandData>,
}

impl<'app, 'client> ApplicationCommandInteraction<'app, 'client> {
    pub fn new(
        application_interaction: &'app mut ApplicationInteraction<'client>,
    ) -> BotResult<Self> {
        let command_data: Box<CommandData> =
            match &mut application_interaction.event_interaction.data {
                Some(InteractionData::ApplicationCommand(command_data)) => command_data.clone(),
                _ => {
                    return Err(t_application_interaction!(
                        application_interaction,
                        "error.interaction.command_data.missing"
                    )
                    .into());
                }
            };

        Ok(Self {
            application_interaction,
            command_data,
        })
    }

    pub fn get_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<&CommandDataOption> {
        let option_name = option_name.as_ref();

        self.command_data
            .options
            .iter()
            .find(|option| option.name == option_name)
    }

    pub fn get_string_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<String> {
        if let Some(CommandDataOption {
            value: CommandOptionValue::String(value),
            ..
        }) = self.get_option(option_name)
        {
            Some(value.clone())
        } else {
            None
        }
    }

    pub fn get_integer_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<i64> {
        if let Some(CommandDataOption {
            value: CommandOptionValue::Integer(value),
            ..
        }) = self.get_option(option_name)
        {
            Some(*value)
        } else {
            None
        }
    }

    pub fn get_number_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<f64> {
        if let Some(CommandDataOption {
            value: CommandOptionValue::Number(value),
            ..
        }) = self.get_option(option_name)
        {
            Some(*value)
        } else {
            None
        }
    }

    pub fn get_boolean_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<bool> {
        if let Some(CommandDataOption {
            value: CommandOptionValue::Boolean(value),
            ..
        }) = self.get_option(option_name)
        {
            Some(*value)
        } else {
            None
        }
    }

    pub fn get_user_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<&User> {
        let Some(CommandDataOption {
            value: CommandOptionValue::User(value),
            ..
        }) = self.get_option(option_name)
        else {
            return None;
        };

        if let Some(ref resolved) = self.command_data.resolved {
            resolved.users.get(value)
        } else {
            None
        }
    }

    pub fn get_channel_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<&InteractionChannel> {
        let Some(CommandDataOption {
            value: CommandOptionValue::Channel(value),
            ..
        }) = self.get_option(option_name)
        else {
            return None;
        };

        if let Some(ref resolved) = self.command_data.resolved {
            resolved.channels.get(value)
        } else {
            None
        }
    }

    pub fn get_role_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<&Role> {
        let Some(CommandDataOption {
            value: CommandOptionValue::Role(value),
            ..
        }) = self.get_option(option_name)
        else {
            return None;
        };

        if let Some(ref resolved) = self.command_data.resolved {
            resolved.roles.get(value)
        } else {
            None
        }
    }

    pub fn get_mentionable_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<Mentionable<'_>> {
        let Some(CommandDataOption {
            value: CommandOptionValue::Mentionable(value),
            ..
        }) = self.get_option(option_name)
        else {
            return None;
        };

        if let Some(ref resolved) = self.command_data.resolved {
            if let Some(user) = resolved.users.get(&value.cast()) {
                Some(Mentionable::User(user))
            } else if let Some(role) = resolved.roles.get(&value.cast()) {
                Some(Mentionable::Role(role))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_attachment_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<&Attachment> {
        let Some(CommandDataOption {
            value: CommandOptionValue::Attachment(value),
            ..
        }) = self.get_option(option_name)
        else {
            return None;
        };

        if let Some(ref resolved) = self.command_data.resolved {
            resolved.attachments.get(value)
        } else {
            None
        }
    }

    pub async fn reply(&self, data: InteractionResponseData) -> BotResult<Response<EmptyBody>> {
        self.application_interaction.reply(data).await
    }

    pub async fn defer_reply(
        &mut self,
        message_flags: MessageFlags,
    ) -> BotResult<Response<EmptyBody>> {
        self.application_interaction
            .defer_reply(message_flags)
            .await
    }

    pub fn update_response(&self) -> UpdateResponse<'_> {
        self.application_interaction.update_response()
    }
}
