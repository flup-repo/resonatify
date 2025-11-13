use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioValidationError {
    #[error("audio file '{0}' was not found")]
    NotFound(String),
    #[error(
        "audio file '{path}' exceeds maximum size of {file_bytes} bytes (limit {max_bytes} bytes)"
    )]
    FileTooLarge {
        path: String,
        file_bytes: u64,
        max_bytes: u64,
    },
    #[error("unsupported audio format '{extension}'")]
    UnsupportedFormat { extension: String },
    #[error("failed to decode audio file: {0}")]
    Decode(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("no audio output device available")]
    NoOutputDevice,
    #[error("failed to initialise audio output stream: {0}")]
    Stream(#[source] rodio::StreamError),
    #[error("failed to create audio sink: {0}")]
    Sink(#[source] rodio::PlayError),
    #[error("failed to decode audio stream: {0}")]
    Decoder(String),
    #[error("failed to read audio file: {0}")]
    Io(#[from] std::io::Error),
    #[error("audio engine is not running")]
    EngineUnavailable,
    #[error(transparent)]
    Validation(#[from] AudioValidationError),
}
