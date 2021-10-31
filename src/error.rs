use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ParseError(#[from] serde_json::Error),

    #[error(transparent)]
    ConfigError(#[from] toml::de::Error),

    #[error(transparent)]
    ReadError(#[from] std::io::Error),

    #[error(transparent)]
    ChannelError(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("The module `{0}` failed to init: `{1}`")]
    ModuleInit(String, String),

    #[error("Renderer crashed")]
    RenderCrash,

    #[error("Module `{0}` not found")]
    ModuleNotFound(String),

    #[error("Streamdeck error: `{0}")]
    StreamdeckError(String),
}
