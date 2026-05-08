use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrinterError {
    #[error("No Bluetooth adapters found. Please check if Bluetooth is enabled.")]
    NoAdapterFound,

    #[error("Bluetooth adapter error: {0}")]
    AdapterError(String),

    #[error("Could not find D30 device. Is it turned on and in range?")]
    DeviceNotFound,

    #[error("Discovery timeout: could not find D30 within the time limit.")]
    DiscoveryTimeout,

    #[error("Failed to connect to D30: {0}")]
    ConnectionFailed(String),

    #[error("Failed to find D30 Bluetooth characteristics. The device might not be supported.")]
    CharacteristicNotFound,

    #[error("Bluetooth communication error: {0}")]
    CommunicationError(String),

    #[error("Image processing error: {0}")]
    ImageError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Unexpected error: {0}")]
    Other(String),
}
