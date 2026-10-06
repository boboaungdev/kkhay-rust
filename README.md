# K Khay Rust SDK 🦀

Official Rust SDK for the **[K Khay Sovereign Crypto Payment Gateway](https://kkhay.com)**.

High-performance, async-first Rust client for accepting USDT, USDC, BNB, ETH across BSC, Polygon, Arbitrum, Base, and Ethereum.

---

## 📦 Installation

Add to `Cargo.toml`:

```toml
[dependencies]
kkhay = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

---

## ⚡ Quick Start

```rust
use kkhay::{CreateInvoiceRequest, KkhayClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KkhayClient::new("kkhay_live_your_api_key_here")?;

    let req = CreateInvoiceRequest {
        price_amount: 49.99,
        price_currency: Some("USD".to_string()),
        pay_network: "bsc".to_string(),
        pay_token: "USDT".to_string(),
        order_id: Some("ORDER-9912".to_string()),
        title: Some("Rust Game License".to_string()),
        customer_name: None,
        customer_email: Some("gamer@example.com".to_string()),
        redirect_url: Some("https://myshop.com/orders/success".to_string()),
        cancel_url: None,
        ipn_callback_url: None,
        metadata: None,
    };

    let response = client.create_invoice(&req).await?;
    println!("Invoice ID: {}", response.invoice.id);
    println!("Checkout URL: {}", response.invoice.hosted_url);

    Ok(())
}
```

---

## 🔐 Webhook / IPN Verification (Axum Example)

```rust
use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use kkhay::parse_webhook_event;

async fn webhook_handler(headers: HeaderMap, body: Bytes) -> impl IntoResponse {
    let signature = headers
        .get("x-kkhay-signature")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    let secret = std::env::var("KKHAY_IPN_SECRET").unwrap_or_default();

    match parse_webhook_event(&body, signature, &secret) {
        Ok(event) => {
            println!("Received event: {} for invoice: {}", event.event, event.invoice_id);
            StatusCode::OK
        }
        Err(_) => StatusCode::UNAUTHORIZED,
    }
}
```

---

## 📄 License

MIT © [K Khay](https://kkhay.com)

