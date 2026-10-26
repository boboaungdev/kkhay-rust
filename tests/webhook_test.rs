use hmac::{Hmac, Mac};
use sha2::Sha256;

use kkhay::{
    parse_webhook_event, verify_webhook_signature, Invoice, InvoiceStatus, KkhayClient,
};

type HmacSha256 = Hmac<Sha256>;

#[test]
fn test_webhook_verification() {
    let secret = "whsec_test_secret_123";
    let payload = br#"{"event":"invoice.paid","invoice_id":"inv_123","order_id":"ORD-1"}"#;

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
    assert!(!verify_webhook_signature(payload, "", secret));
    assert!(!verify_webhook_signature(payload, &valid_sig, ""));
}

#[test]
fn test_parse_webhook() {
    let secret = "whsec_test_secret";
    let payload = br#"{
        "event": "invoice.paid",
        "invoice_id": "inv_abc",
        "order_id": "ORD-55",
        "price_amount": 50.0,
        "price_currency": "USD",
        "pay_amount": "50.00",
        "pay_token": "USDT",
        "pay_network": "bsc",
        "deposit_address": "0x123",
        "tx_hash": "0xabc",
        "status": "PAID",
        "timestamp": "2026-10-26T00:00:00Z"
    }"#;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload);
    let valid_sig = hex::encode(mac.finalize().into_bytes());

    let event = parse_webhook_event(payload, &valid_sig, secret).unwrap();
    assert_eq!(event.event, "invoice.paid");
    assert_eq!(event.invoice_id, "inv_abc");
    assert_eq!(event.pay_token.as_deref(), Some("USDT"));
    assert_eq!(event.status, "PAID");
}

#[test]
fn test_invoice_status_serialization() {
    assert_eq!(
        serde_json::to_string(&InvoiceStatus::Waiting).unwrap(),
        "\"WAITING\""
    );
    assert_eq!(
        serde_json::to_string(&InvoiceStatus::Paid).unwrap(),
        "\"PAID\""
    );
    assert_eq!(
        serde_json::to_string(&InvoiceStatus::Expired).unwrap(),
        "\"EXPIRED\""
    );
    assert_eq!(
        serde_json::to_string(&InvoiceStatus::Canceled).unwrap(),
        "\"CANCELED\""
    );

    let parsed: InvoiceStatus = serde_json::from_str("\"WAITING\"").unwrap();
    assert_eq!(parsed, InvoiceStatus::Waiting);

    let parsed_unknown: InvoiceStatus = serde_json::from_str("\"SOMETHING_ELSE\"").unwrap();
    assert_eq!(parsed_unknown, InvoiceStatus::Unknown);
}

#[test]
fn test_invoice_deserialization_gateway_id_alias() {
    let json_data = r#"{
        "id": "inv_999",
        "gatewayId": "gw_456",
        "priceAmount": 25.5,
        "priceCurrency": "USD",
        "status": "WAITING",
        "hostedUrl": "https://kkhay.com/gateway/invoice/inv_999",
        "createdAt": "2026-10-26T00:00:00Z"
    }"#;

    let inv: Invoice = serde_json::from_str(json_data).unwrap();
    assert_eq!(inv.id, "inv_999");
    assert_eq!(inv.gateway_id.as_deref(), Some("gw_456"));
    assert_eq!(inv.price_amount, 25.5);
    assert_eq!(inv.price_currency, "USD");
    assert_eq!(inv.status, "WAITING");
    assert_eq!(inv.hosted_url, "https://kkhay.com/gateway/invoice/inv_999");
}

#[test]
fn test_client_url_resolution() {
    let client = KkhayClient::new("test_pk_key").unwrap();
    assert_eq!(client.base_url(), "https://api.kkhay.com");
    assert_eq!(
        client.get_url("/v1/gateway/invoices"),
        "https://api.kkhay.com/v1/gateway/invoices"
    );
    assert_eq!(
        client.get_url("/health"),
        "https://api.kkhay.com/health"
    );

    let custom_client = KkhayClient::with_options(
        "test_pk_key",
        "https://custom-host.com/api/",
        std::time::Duration::from_secs(10),
    )
    .unwrap();
    assert_eq!(
        custom_client.get_url("/v1/gateway/invoices"),
        "https://custom-host.com/api/v1/gateway/invoices"
    );
}
