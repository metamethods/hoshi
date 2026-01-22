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

use crate::{BotResult, Mentionable, error::BotError};

pub struct ApplicationInteraction<'client> {
    pub event_interaction: Interaction,
    pub interaction_client: &'client InteractionClient<'client>,
    pub is_deferred: bool,
    pub is_deferred_ephemeral: bool,
}

impl<'client> ApplicationInteraction<'client> {
    pub fn new(
        event_interaction: Interaction,
        interaction_client: &'client InteractionClient<'client>,
    ) -> Self {
        Self {
            event_interaction,
            interaction_client,
            is_deferred: false,
            is_deferred_ephemeral: false,
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

        self.is_deferred = true;

        if message_flags.contains(MessageFlags::EPHEMERAL) {
            self.is_deferred_ephemeral = true;
        }

        Ok(response)
    }

    pub async fn get_response(&self) -> BotResult<Response<Message>> {
        Ok(self
            .interaction_client
            .response(&self.event_interaction.token)
            .await?)
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
                    return Err(BotError::String(
                        t_application_interaction!(
                            application_interaction,
                            "error.interaction.command_data.missing"
                        )
                        .into(),
                    ));
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
        let CommandOptionValue::String(ref value) = self.get_option(option_name)?.value else {
            return None;
        };

        Some(value.clone())
    }

    pub fn get_integer_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<i64> {
        let CommandOptionValue::Integer(value) = self.get_option(option_name)?.value else {
            return None;
        };

        Some(value)
    }

    pub fn get_number_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<f64> {
        let CommandOptionValue::Number(value) = self.get_option(option_name)?.value else {
            return None;
        };

        Some(value)
    }

    pub fn get_boolean_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<bool> {
        let CommandOptionValue::Boolean(value) = self.get_option(option_name)?.value else {
            return None;
        };

        Some(value)
    }

    pub fn get_user_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<&User> {
        let CommandOptionValue::User(value) = self.get_option(option_name)?.value else {
            return None;
        };

        self.command_data.resolved.as_ref()?.users.get(&value)
    }

    pub fn get_channel_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<&InteractionChannel> {
        let CommandOptionValue::Channel(value) = self.get_option(option_name)?.value else {
            return None;
        };

        self.command_data.resolved.as_ref()?.channels.get(&value)
    }

    pub fn get_role_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<&Role> {
        let CommandOptionValue::Role(value) = self.get_option(option_name)?.value else {
            return None;
        };

        self.command_data.resolved.as_ref()?.roles.get(&value)
    }

    pub fn get_mentionable_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<Mentionable<'_>> {
        let CommandOptionValue::Mentionable(value) = self.get_option(option_name)?.value else {
            return None;
        };

        let resolved = self.command_data.resolved.as_ref()?;

        if let Some(user) = resolved.users.get(&value.cast()) {
            Some(Mentionable::User(user))
        } else if let Some(role) = resolved.roles.get(&value.cast()) {
            Some(Mentionable::Role(role))
        } else {
            None
        }
    }

    pub fn get_attachment_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<&Attachment> {
        let CommandOptionValue::Attachment(value) = self.get_option(option_name)?.value else {
            return None;
        };

        self.command_data.resolved.as_ref()?.attachments.get(&value)
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
