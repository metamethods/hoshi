use std::io::Cursor;

use ril::prelude::*;
use twilight_model::{channel::message::MessageFlags, http::attachment::Attachment};

use crate::{
    BotResult, context::BotContext, download_from_url, get_avatar_url,
    interaction::ApplicationCommandInteraction,
};

const FACE_MASK: (u32, u32, u32, u32) = (254, 137, 167, 123);

pub async fn command(
    command_interaction: &mut ApplicationCommandInteraction<'_, '_>,
    context: &BotContext,
) -> BotResult<()> {
    command_interaction
        .defer_reply(MessageFlags::empty())
        .await?;

    let user_input =
        command_interaction
            .get_user_option("user")
            .ok_or(t_application_interaction!(
                command_interaction.application_interaction,
                "error.command.option.required_missing",
                option_name = "user",
            ))?;

    let avatar_url = get_avatar_url(user_input).ok_or(t_application_interaction!(
        command_interaction.application_interaction,
        "error.command.soupify.missing_avatar",
        username = user_input.name
    ))?;

    let mut avatar_image = Image::<Rgba>::from_bytes_inferred(
        download_from_url(avatar_url, &context.reqwest_client)
            .await?
            .as_ref(),
    )?;
    avatar_image.resize(FACE_MASK.2, FACE_MASK.3, ResizeAlgorithm::Lanczos3);

    let mut snowman_image = context.assets.snowman.snowman.clone();
    snowman_image.paste_with_mask(
        FACE_MASK.0,
        FACE_MASK.1,
        &avatar_image,
        &context.assets.snowman.mask,
    );

    let mut snowman_image_cursor = Cursor::new(vec![]);

    snowman_image.encode(ImageFormat::Png, &mut snowman_image_cursor)?;

    command_interaction
        .update_response()
        .attachments(&[Attachment::from_bytes(
            "output.png".into(),
            snowman_image_cursor.into_inner(),
            1,
        )])
        .await?;

    Ok(())
}
