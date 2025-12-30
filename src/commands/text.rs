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

    let input_image_attachment =
        command_interaction
            .get_attachment_option("input")
            .ok_or(t_application_interaction!(
                command_interaction.application_interaction,
                "error.command.option.required_missing",
                option_name = "input"
            ))?;
    let text = command_interaction
        .get_string_option("text")
        .ok_or(t_application_interaction!(
            command_interaction.application_interaction,
            "error.command.option.required_missing",
            option_name = "input"
        ))?;
    let font_size = command_interaction
        .get_number_option("fontsize")
        .unwrap_or(32.);

    let image_buffer =
        download_attachment(&input_image_attachment, &context.reqwest_client).await?;

    let mut image = Image::<Rgba>::from_bytes_inferred(image_buffer.as_ref())?;
    let (image_width, _) = image.dimensions();

    let text_segment = TextSegment::new(&context.assets.fonts.impact, text, Rgba::black())
        .with_size(font_size as f32);
    let text_layout = TextLayout::new()
        .with_align(TextAlign::Center)
        .with_horizontal_anchor(HorizontalAnchor::Center)
        .with_position(image_width / 2, 0)
        .with_segment(&text_segment);

    image.draw(&text_layout);

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
