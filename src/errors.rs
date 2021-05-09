use serenity::framework::standard::Reason;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BannerBotError {
    #[error("Image doesn't have the right type. png, jpg, ...")]
    InvalidImageType,

    #[error("Guild doesn't have this image type. icon, banner, ...")]
    InvalidGuildImageType,

    #[error("Storage not initialized")]
    StorageNotInitialized,

    #[error("Storage empty")]
    StorageEmpty,

    #[error(transparent)]
    DownloadError(#[from] reqwest::Error),

    #[error(transparent)]
    SerenityError(#[from] serenity::Error),

    #[error(transparent)]
    ImageError(#[from] image::error::ImageError),
}

impl From<BannerBotError> for Reason {
    fn from(e: BannerBotError) -> Self {
        Reason::Log(e.to_string())
    }
}
