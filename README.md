# K Khay Rust SDK 🦀

[![Crates.io](https://img.shields.io/crates/v/kkhay.svg)](https://crates.io/crates/kkhay)
[![Documentation](https://docs.rs/kkhay/badge.svg)](https://docs.rs/kkhay)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI Status](https://img.shields.io/github/actions/workflow/status/boboaungdev/kkhay-rust/ci.yml?branch=main&label=CI)](https://github.com/boboaungdev/kkhay-rust/actions)

Official Rust asynchronous SDK for the **[K Khay Gateway](https://kkhay.com)** — the sovereign cryptocurrency payment gateway.

Accept sovereign, non-custodial crypto payments (**USDT, USDC, BTC, ETH, SOL, TON, BNB, TRX, DOGE, XRP, LTC** across **190+ blockchain networks**) directly in your Rust backends, microservices, Axum, Actix-web, or Rocket applications with **instant on-chain verification**.

---

## 🌟 Features

* **High Performance & Async Native**: Built on `tokio` and `reqwest` with zero-allocation cryptographic checks.
* **190+ Blockchain Networks**: Native support for EVM chains (BNB Chain, Polygon, Arbitrum, Base, Ethereum), Solana, TON, Tron, Bitcoin, XRPL, Dogecoin, and Litecoin.
* **Database Enum Alignment**: Strictly aligned with gateway invoice states (`WAITING`, `PAID`, `EXPIRED`, `CANCELED`).
* **Timing-Safe HMAC Verification**: Constant-time HMAC-SHA256 signature verification preventing timing side-channel attacks via `subtle`.
* **Flexible Deserialization**: Bidirectional camelCase and snake_case mapping for seamless serde JSON parsing.

---

## 📦 Installation

Add `kkhay` to your `Cargo.toml`:

```toml
[dependencies]
kkhay = "1.0.4"
tokio = { version = "1", features = ["full"] }
```

Or using `cargo add`:

```bash
cargo add kkhay
```

---

## ⚡ Quick Start

### 1. Initialize the Client

```rust
use kkhay::KkhayClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KkhayClient::new("your_gateway_api_key")?;
    
    // Check Gateway health
    let health = client.check_health().await?;
    println!("Gateway status: {}", health.status);

    Ok(())
}
```

---

### 2. Create a Crypto Checkout Invoice

```rust
use kkhay::{KkhayClient, CreateInvoiceRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KkhayClient::new("your_gateway_api_key")?;

    let request = CreateInvoiceRequest {
        price_amount: 49.99,
        price_currency: Some("USD".to_string()),
        pay_network: Some("solana".to_string()),
        pay_token: Some("USDC".to_string()),
        order_id: Some("ORDER-9921".to_string()),
        title: Some("1 Year Pro Plan".to_string()),
        customer_email: Some("customer@example.com".to_string()),
        redirect_url: Some("https://myshop.com/success".to_string()),
        cancel_url: Some("https://myshop.com/cart".to_string()),
        ..Default::default()
    };

    let response = client.create_invoice(&request).await?;
    let invoice = response.invoice;

    println!("Invoice ID: {}", invoice.id);
    println!("Status: {}", invoice.status);
    println!("Checkout URL: {}", invoice.hosted_url);

    Ok(())
}
```

---

### 3. Retrieve and Check Invoice Status

```rust
use kkhay::{KkhayClient, InvoiceStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KkhayClient::new("your_gateway_api_key")?;

    let response = client.get_invoice("inv_83a1b2c").await?;
    println!("Status: {}", response.invoice.status);

    for payment in response.payments {
        println!("Tx: {} (Confirmations: {})", payment.tx_hash, payment.confirmations);
    }

    Ok(())
}
```

---

## 🔐 Webhook / IPN Verification (Axum / Actix-web)

Validate incoming webhook notifications with timing-safe HMAC-SHA256 signatures:

```rust
use kkhay::parse_webhook_event;

// Example Axum handler
async fn handle_kkhay_webhook(
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let signature = headers
        .get("x-kkhay-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let secret = std::env::var("KKHAY_WEBHOOK_SECRET").unwrap();

    let event = parse_webhook_event(&body, signature, &secret)
        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;

    match event.status.as_str() {
        "PAID" => {
            println!("Order {:?} settled on-chain!", event.order_id);
        }
        "EXPIRED" => {
            println!("Invoice {} expired.", event.invoice_id);
        }
        _ => {}
    }

    Ok(axum::http::StatusCode::OK)
}
```

---

## 🌐 Supported Cryptocurrencies & Networks

Supports 190+ blockchain networks and 500+ cryptocurrencies:

| Ecosystem | Networks | Supported Assets |
| :--- | :--- | :--- |
| **BNB Smart Chain** | BSC Mainnet, BSC Testnet | USDT, USDC, BNB, and BEP-20 tokens |
| **Polygon** | Polygon PoS, Amoy Testnet | USDT, USDC, POL, and ERC-20 tokens |
| **Arbitrum** | Arbitrum One, Nova, Sepolia | USDT, USDC, ETH, ARB |
| **Base** | Base Mainnet, Base Sepolia | USDC, ETH, and Base tokens |
| **Ethereum** | Ethereum Mainnet, Sepolia | USDT, USDC, ETH, and ERC-20 tokens |
| **Solana** | Solana Mainnet, Devnet | SOL, USDT, USDC, and SPL tokens |
| **TON** | TON Mainnet, Testnet | TON, USDT, and Jettons |
| **Tron** | Tron Mainnet, Shasta | TRX, USDT (TRC-20), USDC |
| **Bitcoin** | Bitcoin Mainnet, Testnet | BTC |
| **XRPL** | XRP Ledger Mainnet, Testnet | XRP |
| **Dogecoin** | Dogecoin Mainnet | DOGE |
| **Litecoin** | Litecoin Mainnet | LTC |

---

## 📄 License

MIT © [K Khay](https://kkhay.com)
