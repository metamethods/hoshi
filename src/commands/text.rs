use std::io::Cursor;

use ril::prelude::*;
use twilight_model::{channel::message::MessageFlags, http::attachment::Attachment};

use crate::{
    BotResult, context::BotContext, download_attachment, interaction::ApplicationCommandInteraction,
};

pub async fn command(
    command_interaction: &mut ApplicationCommandInteraction<'_, '_>,
    context: &BotContext,
) -> BotResult<()> {
    command_interaction
        .defer_reply(MessageFlags::empty())
        .await?;

    let input_image_attachment = command_interaction.get_attachment_option("input").ok_or(
        t_application_interaction_err!(
            command_interaction.application_interaction,
            "error.command.option.required_missing",
            option_name = "input"
        ),
    )?;
    let text =
        command_interaction
            .get_string_option("text")
            .ok_or(t_application_interaction_err!(
                command_interaction.application_interaction,
                "error.command.option.required_missing",
                option_name = "text"
            ))?;

    let image_buffer =
        download_attachment(&input_image_attachment, &context.reqwest_client).await?;

    let mut image = Image::<Rgba>::from_bytes_inferred(image_buffer.as_ref())?
        .with_overlay_mode(OverlayMode::Merge);
    let (image_width, image_height) = image.dimensions();

    let text_layout = TextLayout::new()
        .with_align(TextAlign::Center)
        .with_basic_text(&context.assets.fonts.impact, text, Rgba::black());
    let (text_layout_width, text_layout_height) = text_layout.dimensions();
    let mut text_layout_image =
        Image::new(text_layout_width, text_layout_height, Rgba::transparent());

    text_layout_image.draw(&text_layout);
    text_layout_image.resize(image_width, image_height / 6, ResizeAlgorithm::Lanczos3);

    image.paste(0, 0, &text_layout_image);

    let mut image_cursor = Cursor::new(vec![]);

    image.encode(ril::ImageFormat::Png, &mut image_cursor)?;

    command_interaction
        .update_response()
        .attachments(&[Attachment::from_bytes(
            "output.png".into(),
            image_cursor.into_inner(),
            1,
        )])
        .await?;

    Ok(())
}
