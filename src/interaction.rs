use twilight_http::{
    Response,
    client::InteractionClient,
    request::application::interaction::{CreateFollowup, UpdateResponse},
    response::marker::EmptyBody,
};
use twilight_model::{
    application::interaction::Interaction as EventInteraction,
    channel::{Message, message::MessageFlags},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{error::BotResult, resolver::InteractionDataResolver, response::BotResponse};

#[derive(Debug)]
pub struct ApplicationInteraction<'client> {
    pub event_interaction: EventInteraction,
    pub interaction_client: InteractionClient<'client>,
    pub has_replied: bool,
    pub is_deferred: bool,
}

impl<'client> ApplicationInteraction<'client> {
    pub fn new(
        event_interaction: EventInteraction,
        interaction_client: InteractionClient<'client>,
    ) -> Self {
        Self {
            event_interaction,
            interaction_client,
            has_replied: false,
            is_deferred: false,
        }
    }

    pub async fn create_response(
        &self,
        kind: InteractionResponseType,
        data_option: Option<BotResponse>,
    ) -> BotResult<Response<EmptyBody>> {
        Ok(self
            .interaction_client
            .create_response(
                self.event_interaction.id,
                &self.event_interaction.token,
                &InteractionResponse {
                    kind,
                    data: match data_option {
                        Some(data) => Some(match data {
                            BotResponse::ResponseData(data) => data,
                            BotResponse::Content(content) => InteractionResponseDataBuilder::new()
                                .content(content)
                                .build(),
                            BotResponse::Error(error) => {
                                InteractionResponseDataBuilder::new().content(error).build()
                            }
                        }),
                        None => None,
                    },
                },
            )
            .await?)
    }

    pub async fn get_response(&self) -> BotResult<Response<Message>> {
        Ok(self
            .interaction_client
            .response(&self.event_interaction.token)
            .await?)
    }

    pub async fn reply(&mut self, data: BotResponse) -> BotResult<Response<EmptyBody>> {
        let response = self
            .create_response(
                InteractionResponseType::ChannelMessageWithSource,
                Some(data),
            )
            .await?;

        self.has_replied = true;

        Ok(response)
    }

    pub fn followup_builder(&self) -> CreateFollowup<'_> {
        self.interaction_client
            .create_followup(&self.event_interaction.token)
    }

    pub async fn followup_from_response_data(
        &self,
        response_data: InteractionResponseData,
    ) -> BotResult<()> {
        self.followup_builder()
            .content(response_data.content.as_deref().unwrap_or_default())
            .embeds(response_data.embeds.as_deref().unwrap_or_default())
            .components(response_data.components.as_deref().unwrap_or_default())
            .attachments(response_data.attachments.as_deref().unwrap_or_default())
            .allowed_mentions(response_data.allowed_mentions.as_ref())
            .flags(response_data.flags.unwrap_or(MessageFlags::empty()))
            .await?;

        Ok(())
    }

    pub fn update_response_builder(&self) -> UpdateResponse<'_> {
        self.interaction_client
            .update_response(&self.event_interaction.token)
    }

    pub async fn update_response_from_response_data(
        &self,
        response_data: InteractionResponseData,
    ) -> BotResult<()> {
        self.update_response_builder()
            .content(response_data.content.as_deref())
            .embeds(response_data.embeds.as_deref())
            .components(response_data.components.as_deref())
            .attachments(response_data.attachments.as_deref().unwrap_or_default())
            .allowed_mentions(response_data.allowed_mentions.as_ref())
            .flags(response_data.flags.unwrap_or(MessageFlags::empty()))
            .await?;
        Ok(())
    }

    pub async fn defer_reply(
        &mut self,
        message_flags: MessageFlags,
    ) -> BotResult<Response<EmptyBody>> {
        let response = self
            .create_response(
                InteractionResponseType::DeferredChannelMessageWithSource,
                Some(BotResponse::ResponseData(
                    InteractionResponseDataBuilder::new()
                        .flags(message_flags)
                        .build(),
                )),
            )
            .await?;

        self.is_deferred = true;

        Ok(response)
    }

    pub fn locale(&self) -> String {
        self.event_interaction
            .locale
            .clone()
            .unwrap_or(String::from("en-US"))
    }
}

#[derive(Debug)]
pub struct ApplicationCommandInteraction<'interaction, 'client, R: InteractionDataResolver> {
    pub application_interaction: &'interaction mut ApplicationInteraction<'client>,
    pub data_resolver: R,
}

impl<'interaction, 'client, R: InteractionDataResolver>
    ApplicationCommandInteraction<'interaction, 'client, R>
{
    pub fn new(
        application_interaction: &'interaction mut ApplicationInteraction<'client>,
        data_resolver: R,
    ) -> Self {
        Self {
            application_interaction,
            data_resolver,
        }
    }

    pub async fn get_response(&self) -> BotResult<Response<Message>> {
        self.application_interaction.get_response().await
    }

    pub async fn reply(&mut self, data: BotResponse) -> BotResult<Response<EmptyBody>> {
        self.application_interaction.reply(data).await
    }

    pub fn followup_builder(&self) -> CreateFollowup<'_> {
        self.application_interaction.followup_builder()
    }

    pub async fn followup_from_response_data(
        &self,
        response_data: InteractionResponseData,
    ) -> BotResult<()> {
        self.application_interaction
            .followup_from_response_data(response_data)
            .await
    }

    pub fn update_response_builder(&self) -> UpdateResponse<'_> {
        self.application_interaction.update_response_builder()
    }

    pub async fn update_response_from_response_data(
        &self,
        response_data: InteractionResponseData,
    ) -> BotResult<()> {
        self.application_interaction
            .update_response_from_response_data(response_data)
            .await
    }

    pub async fn defer_reply(
        &mut self,
        message_flags: MessageFlags,
    ) -> BotResult<Response<EmptyBody>> {
        self.application_interaction
            .defer_reply(message_flags)
            .await
    }

    pub fn locale(&self) -> String {
        self.application_interaction.locale()
    }
}
