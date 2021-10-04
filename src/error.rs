use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ParseError(#[from] serde_json::Error),

    #[error(transparent)]
    ReadError(#[from] std::io::Error),

    #[error(transparent)]
    ChannelError(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("Renderer crashed")]
    RenderCrash,
}
