use hmac::{Hmac, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq;

use crate::error::KkhayError;
use crate::types::WebhookEvent;

type HmacSha256 = Hmac<Sha256>;

/// Verifies the cryptographic HMAC-SHA256 signature sent with incoming K Khay IPN webhooks.
pub fn verify_webhook_signature(payload: &[u8], signature: &str, secret: &str) -> bool {
    let sig = signature.trim();
    let sec = secret.trim();
    if sig.is_empty() || sec.is_empty() {
        return false;
    }

    let mut mac = match HmacSha256::new_from_slice(sec.as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };

    mac.update(payload);
    let expected_bytes = mac.finalize().into_bytes();
    let expected_hex = hex::encode(expected_bytes);

    expected_hex.as_bytes().ct_eq(sig.as_bytes()).into()
}

/// Validates the HMAC signature and deserializes the incoming JSON webhook payload.
pub fn parse_webhook_event(
    raw_body: &[u8],
    signature: &str,
    secret: &str,
) -> Result<WebhookEvent, KkhayError> {
    if !verify_webhook_signature(raw_body, signature, secret) {
        return Err(KkhayError::InvalidSignature);
    }

    let event: WebhookEvent = serde_json::from_slice(raw_body)?;
    Ok(event)
}
