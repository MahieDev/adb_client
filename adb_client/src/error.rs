use thiserror::Error;

/// Custom Result type thrown by this crate.
pub type Result<T> = std::result::Result<T, RustADBError>;

/// Represents all error types that can be thrown by the crate.
#[derive(Error, Debug)]
pub enum RustADBError {
    /// Indicates that an error occurred with I/O.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    /// Indicates that an error occurred when sending ADB request.
    #[error("ADB request failed - {0}")]
    ADBRequestFailed(String),
    /// Indicates that ADB server responded an unknown response type.
    #[error("Unknown response type {0}")]
    UnknownResponseType(String),
    /// Indicates that ADB server responses an unknown device state.
    #[error("Unknown device state {0}")]
    UnknownDeviceState(String),
    /// Indicates that an error occurred during UTF-8 parsing.
    #[error(transparent)]
    Utf8StrError(#[from] std::str::Utf8Error),
    /// Indicates that an error occurred during UTF-8 parsing.
    #[error(transparent)]
    Utf8StringError(#[from] std::string::FromUtf8Error),
    /// Indicates that the provided address is not a correct IP address.
    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),
    /// Indicates an error with regexps.
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    /// Indicates that parsing regex did not worked.
    #[error("Regex parsing error: missing field")]
    RegexParsingError,
    /// Indicates an error with the integer conversion.
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    /// Indicates that an error occurred when converting a value.
    #[error("Conversion error")]
    ConversionError,
    /// Remote ADB server does not support shell feature.
    #[error("Remote ADB server does not support shell feature")]
    ADBShellNotSupported,
    /// Desired device has not been found
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    /// Indicates that the device must be paired before attempting a connection over WI-FI
    #[error("Device not paired before attempting to connect")]
    ADBDeviceNotPaired,
    /// An error occurred when getting device's framebuffer image
    #[error(transparent)]
    FramebufferImageError(#[from] image::error::ImageError),
    /// An error occurred when converting framebuffer content
    #[error("Cannot convert framebuffer into image")]
    FramebufferConversionError,
    /// Unimplemented framebuffer image version
    #[error("Unimplemented framebuffer image version: {0}")]
    UnimplementedFramebufferImageVersion(u32),
    /// An error occurred while getting user's home directory
    #[error(transparent)]
    HomeError(#[from] homedir::GetHomeError),
    /// Cannot get home directory
    #[error("Cannot get home directory")]
    NoHomeDirectory,
    /// Generic USB error
    #[error(transparent)]
    UsbError(#[from] rusb::Error),
    /// USB device not found
    #[error("USB Device not found: {0} {1}")]
    USBDeviceNotFound(u16, u16),
    /// No descriptor found
    #[error("No USB descriptor found")]
    USBNoDescriptorFound,
    /// Integrity of the received message cannot be validated
    #[error("Invalid integrity. Expected CRC32 {0}, got {1}")]
    InvalidIntegrity(u32, u32),
    /// Error while decoding base64 data
    #[error(transparent)]
    Base64DecodeError(#[from] base64::DecodeError),
    /// Error while encoding base64 data
    #[error(transparent)]
    Base64EncodeError(#[from] base64::EncodeSliceError),
    /// An error occurred with RSA engine
    #[error(transparent)]
    RSAError(#[from] rsa::errors::Error),
    /// Cannot convert given data from slice
    #[error(transparent)]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
}
