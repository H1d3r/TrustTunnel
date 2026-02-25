//! TrustTunnel Deep-Link Library
//!
//! This library provides encoding and decoding functionality for TrustTunnel
//! deep-link URIs (`tt://` scheme). Deep-links allow compact, shareable
//! configuration URIs that can be used across platforms (mobile, desktop, CLI).
//!

pub mod cert;
pub mod decode;
pub mod encode;
pub mod error;
pub mod types;
pub mod varint;

pub use error::{DeepLinkError, Result};
pub use types::{DeepLinkConfig, DeepLinkConfigBuilder, Protocol, TlvTag};

// Re-export varint functions for testing
pub use varint::{decode_varint, encode_varint};

/// Encode a configuration into a deep-link URI (`tt://...`).
///
/// # Errors
///
/// Returns `DeepLinkError` if encoding fails.
pub fn encode(config: &DeepLinkConfig) -> Result<String> {
    encode::encode(config)
}

/// Encode a configuration into a Unicode-string QR code that contains a deep-link URI.
///
/// # Errors
///
/// Returns `DeepLinkError` if encoding fails.
pub fn encode_to_qr(config: &DeepLinkConfig) -> Result<String> {
    encode::encode_to_qr(config)
}

/// Decode a deep-link URI into a configuration.
///
/// # Errors
///
/// Returns `DeepLinkError` if decoding fails.
pub fn decode(uri: &str) -> Result<DeepLinkConfig> {
    decode::decode(uri)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lib_exports() {
        // Verify main types are exported
        let _: fn(&DeepLinkConfig) -> Result<String> = encode;
        let _: fn(&DeepLinkConfig) -> Result<String> = encode_to_qr;
        let _: fn(&str) -> Result<DeepLinkConfig> = decode;
    }
}
