use std::io::Cursor;

use ril::prelude::*;
use twilight_model::{channel::message::MessageFlags, http::attachment::Attachment};

use crate::{
    BotResult, context::BotContext, download_from_url, get_avatar_url,
    interaction::ApplicationCommandInteraction,
};

const INNER_SOUP_TRANSFORM: (u32, u32, u32, u32) = (96, 38, 273, 110);

pub async fn command(
    command_interaction: &mut ApplicationCommandInteraction<'_, '_>,
    context: &BotContext,
) -> BotResult<()> {
    command_interaction
        .defer_reply(MessageFlags::empty())
        .await?;

    let user_input = t_ok_or_err!(
        command_interaction.get_user_option("user"),
        command_interaction.application_interaction,
        "error.command.option.required_missing",
        option_name = "user"
    )?;

    let avatar_url = t_ok_or_err!(
        get_avatar_url(user_input),
        command_interaction.application_interaction,
        "error.command.soupify.missing_avatar",
        username = user_input.name
    )?;

    let mut avatar_image = Image::<Rgba>::from_bytes_inferred(
        download_from_url(avatar_url, &context.reqwest_client)
            .await?
            .as_ref(),
    )?;
    avatar_image.resize(
        INNER_SOUP_TRANSFORM.2,
        INNER_SOUP_TRANSFORM.3,
        ResizeAlgorithm::Lanczos3,
    );

    let mut soup_bowl_image = context.assets.soup.bowl.clone();
    soup_bowl_image.paste_with_mask(
        INNER_SOUP_TRANSFORM.0,
        INNER_SOUP_TRANSFORM.1,
        &avatar_image,
        &context.assets.soup.mask,
    );

    let mut soup_bowl_cursor = Cursor::new(vec![]);

    soup_bowl_image.encode(ImageFormat::Png, &mut soup_bowl_cursor)?;

    command_interaction
        .update_response()
        .attachments(&[Attachment::from_bytes(
            "output.png".into(),
            soup_bowl_cursor.into_inner(),
            1,
        )])
        .await?;

    Ok(())
}
