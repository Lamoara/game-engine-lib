use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error loading resource: {0}")]
    ErrorLoadingResource(anyhow::Error),
    #[error("Error loading texture: {0}")]
    ErrorReadingTexture(anyhow::Error),
}
