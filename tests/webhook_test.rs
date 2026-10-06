use hmac::{Hmac, Mac};
use sha2::Sha256;

use kkhay::{parse_webhook_event, verify_webhook_signature};

type HmacSha256 = Hmac<Sha256>;

#[test]
fn test_webhook_verification() {
    let secret = "whsec_test_secret_123";
    let payload = br#"{"event":"payment.finished","invoice_id":"inv_123","order_id":"ORD-1"}"#;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload);
    let valid_sig = hex::encode(mac.finalize().into_bytes());

    assert!(verify_webhook_signature(payload, &valid_sig, secret));
    assert!(!verify_webhook_signature(
        br#"{"tampered":true}"#,
        &valid_sig,
        secret
    ));
    assert!(!verify_webhook_signature(payload, &valid_sig, "wrong_secret"));
}

#[test]
fn test_parse_webhook() {
    let secret = "whsec_test_secret";
    let payload = br#"{
        "event": "payment.finished",
        "invoice_id": "inv_abc",
        "order_id": "ORD-55",
        "price_amount": 50.0,
        "price_currency": "USD",
        "pay_amount": "50.00",
        "pay_token": "USDT",
        "pay_network": "bsc",
        "deposit_address": "0x123",
        "tx_hash": "0xabc",
        "status": "paid",
        "timestamp": "2026-10-07T00:00:00Z"
    }"#;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload);
    let valid_sig = hex::encode(mac.finalize().into_bytes());

    let event = parse_webhook_event(payload, &valid_sig, secret).unwrap();
    assert_eq!(event.event, "payment.finished");
    assert_eq!(event.invoice_id, "inv_abc");
    assert_eq!(event.pay_token, "USDT");
}

