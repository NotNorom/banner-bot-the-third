use std::str::FromStr;

use image::{guess_format, ImageFormat};
use reqwest::{IntoUrl, Url};

use crate::errors::BannerBotError;

pub async fn get_image<U: IntoUrl>(
    client: &reqwest::Client,
    url: U,
    image_type: DiscordImage,
) -> Result<String, BannerBotError> {
    let url = url.into_url()?;

    let (data, format) = download(client, url).await?;

    if !image_type.has_valid_format(format) {
        return Err(BannerBotError::InvalidImageType);
    }

    Ok(encode(&data, format))
}

pub async fn download(
    client: &reqwest::Client,
    url: Url,
) -> Result<(Vec<u8>, ImageFormat), BannerBotError> {
    let res = client.get(url).send().await?;
    let data = res.bytes().await?.to_vec();
    let image_format = guess_format(&data)?;
    Ok((data, image_format))
}

pub fn encode(data: &[u8], format: ImageFormat) -> String {
    let b64 = base64::encode(data);
    format!(
        "data:image/{};base64,{}",
        format.extensions_str().get(0).unwrap_or(&""),
        b64
    )
}

#[non_exhaustive]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum DiscordImage {
    GuildIcon,
    GuildBanner,
}

impl DiscordImage {
    fn has_valid_format(&self, format: ImageFormat) -> bool {
        use image::ImageFormat::*;
        match self {
            DiscordImage::GuildIcon => match format {
                Png => true,
                Jpeg => true,
                Gif => true,
                WebP => true,
                _ => false,
            },
            DiscordImage::GuildBanner => match format {
                Png => true,
                Jpeg => true,
                WebP => true,
                _ => false,
            },
        }
    }
}

impl FromStr for DiscordImage {
    type Err = BannerBotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "icon" => Ok(Self::GuildIcon),
            "banner" => Ok(Self::GuildBanner),
            _ => Err(BannerBotError::InvalidGuildImageType),
        }
    }
}
