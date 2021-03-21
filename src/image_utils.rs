use image::{guess_format, ImageFormat};
use reqwest::{IntoUrl, Url};

// pub fn read_image<P: AsRef<Path>>(path: P) -> Result<String> {
//     _read_image(path.as_ref())
// }

// fn _read_image(path: &Path) -> Result<String> {
//     let mut v = Vec::default();
//     let mut f = File::open(path)?;

//     // errors here are intentionally ignored
//     #[allow(clippy::let_underscore_must_use)]
//     let _ = f.read_to_end(&mut v);

//     let b64 = base64::encode(&v);
//     let ext = if path.extension() == Some(OsStr::new("png")) { "png" } else { "jpg" };

//     Ok(format!("data:image/{};base64,{}", ext, b64))
// }

pub async fn get_image<U: IntoUrl>(
    client: &reqwest::Client,
    url: U,
    image_type: ImageType,
) -> Result<String, String> {
    let url = match url.into_url() {
        Ok(url) => url,
        Err(e) => return Err(e.to_string()),
    };

    let (data, format) = download(client, url).await?;

    if !is_valid(image_type, format) {
        return Err("Invalid image type".into());
    }

    Ok(encode(&data, format))
}

pub async fn download(
    client: &reqwest::Client,
    url: Url,
) -> Result<(Vec<u8>, ImageFormat), String> {
    let res = match client.get(url).send().await {
        Ok(res) => res,
        Err(e) => return Err(e.to_string()),
    };
    let data = match res.bytes().await {
        Ok(bytes) => bytes.to_vec(),
        Err(e) => return Err(e.to_string()),
    };
    let image_format = match guess_format(&data) {
        Ok(format) => format,
        Err(e) => return Err(e.to_string()),
    };
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

pub fn is_valid(image_type: ImageType, format: ImageFormat) -> bool {
    use self::discord_specific::*;
    match image_type {
        ImageType::GuildIcon => valid_guild_icon(format),
        ImageType::GuildBanner => valid_guild_banner(format),
    }
}

#[non_exhaustive]
pub enum ImageType {
    GuildIcon,
    GuildBanner,
}

mod discord_specific {
    use image::ImageFormat::{self, *};

    pub fn valid_guild_icon(format: ImageFormat) -> bool {
        match format {
            Png => true,
            Jpeg => true,
            Gif => true,
            WebP => true,
            _ => false,
        }
    }

    pub fn valid_guild_banner(format: ImageFormat) -> bool {
        match format {
            Png => true,
            Jpeg => true,
            WebP => true,
            _ => false,
        }
    }
}
