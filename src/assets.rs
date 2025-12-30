use std::{error::Error, io};

use include_dir::{Dir, File};
use ril::prelude::*;

pub struct SpeechBubbleAssets {
    pub tail_left: Image<Rgba>,
}

pub struct SoupAssets {
    pub bowl: Image<Rgba>,
    pub mask: Image<BitPixel>,
}

pub struct SnowmanAssets {
    pub snowman: Image<Rgba>,
    pub mask: Image<BitPixel>,
}

pub struct Fonts {
    pub impact: Font,
}

pub struct Assets {
    pub speech_bubble: SpeechBubbleAssets,
    pub soup: SoupAssets,
    pub snowman: SnowmanAssets,
    pub fonts: Fonts,
}

fn get_asset<'d, 'f, Path: AsRef<str>>(
    assets_dir: &Dir<'d>,
    path: Path,
) -> Result<&'f File<'f>, io::Error>
where
    'd: 'f,
{
    assets_dir
        .get_file(path.as_ref())
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "test"))
}

pub fn load_assets(assets_dir: &Dir<'_>) -> Result<Assets, Box<dyn Error>> {
    Ok(Assets {
        speech_bubble: SpeechBubbleAssets {
            tail_left: Image::from_bytes_inferred(
                get_asset(assets_dir, "bubbles/tail_left.png")?.contents(),
            )?,
        },
        soup: SoupAssets {
            bowl: Image::from_bytes_inferred(
                get_asset(assets_dir, "soup_bowl/bowl.png")?.contents(),
            )?,
            mask: Image::from_bytes_inferred(
                get_asset(assets_dir, "soup_bowl/mask.png")?.contents(),
            )?,
        },
        snowman: SnowmanAssets {
            snowman: Image::from_bytes_inferred(
                get_asset(assets_dir, "snowman/snowman.png")?.contents(),
            )?,
            mask: Image::from_bytes_inferred(
                get_asset(assets_dir, "snowman/mask.png")?.contents(),
            )?,
        },
        fonts: Fonts {
            impact: Font::from_bytes(get_asset(assets_dir, "fonts/impact.ttf")?.contents(), 32.)?,
        },
    })
}
