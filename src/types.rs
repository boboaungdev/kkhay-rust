use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum InvoiceStatus {
    Waiting,
    Paid,
    Expired,
    Canceled,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Waiting => write!(f, "WAITING"),
            Self::Paid => write!(f, "PAID"),
            Self::Expired => write!(f, "EXPIRED"),
            Self::Canceled => write!(f, "CANCELED"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvoiceRequest {
    #[serde(alias = "amount", alias = "price_amount")]
    pub price_amount: f64,
    #[serde(skip_serializing_if = "Option::is_none", alias = "currency", alias = "price_currency")]
    pub price_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "pay_network")]
    pub pay_network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "pay_token")]
    pub pay_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "order_id")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "customer_name")]
    pub customer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "customer_email")]
    pub customer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "returnUrl", alias = "redirect_url")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "cancel_url")]
    pub cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "callbackUrl", alias = "ipn_callback_url")]
    pub ipn_callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub id: String,
    #[serde(alias = "gateway_id", alias = "merchantId", alias = "merchant_id")]
    pub gateway_id: Option<String>,
    #[serde(alias = "order_id")]
    pub order_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(alias = "amount", alias = "price_amount")]
    pub price_amount: f64,
    #[serde(alias = "currency", alias = "price_currency")]
    pub price_currency: String,
    #[serde(alias = "pay_amount")]
    pub pay_amount: Option<String>,
    #[serde(alias = "pay_token")]
    pub pay_token: Option<String>,
    #[serde(alias = "pay_network")]
    pub pay_network: Option<String>,
    #[serde(alias = "deposit_address")]
    pub deposit_address: Option<String>,
    pub status: String,
    #[serde(alias = "fee_amount")]
    pub fee_amount: Option<String>,
    #[serde(alias = "net_amount")]
    pub net_amount: Option<String>,
    #[serde(alias = "checkout_url", alias = "checkoutUrl", alias = "hosted_url")]
    pub hosted_url: String,
    #[serde(alias = "expires_at")]
    pub expires_at: Option<String>,
    #[serde(alias = "created_at")]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRecord {
    pub id: String,
    #[serde(alias = "tx_hash")]
    pub tx_hash: String,
    #[serde(alias = "amount_received")]
    pub amount_received: String,
    pub confirmations: u32,
    pub status: String,
    #[serde(alias = "forwarded_tx_hash")]
    pub forwarded_tx_hash: Option<String>,
    #[serde(alias = "created_at")]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceResponse {
    pub ok: bool,
    pub invoice: Invoice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInvoiceResponse {
    pub ok: bool,
    pub invoice: Invoice,
    #[serde(default)]
    pub payments: Vec<PaymentRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListInvoicesResponse {
    pub ok: bool,
    pub items: Vec<Invoice>,
    #[serde(alias = "total_count")]
    pub total_count: u64,
    #[serde(alias = "total_pages")]
    pub total_pages: u64,
    #[serde(alias = "current_page")]
    pub current_page: u64,
    pub limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookEvent {
    pub event: String,
    #[serde(alias = "id", alias = "invoice_id", alias = "invoiceId")]
    pub invoice_id: String,
    #[serde(alias = "order_id")]
    pub order_id: Option<String>,
    #[serde(alias = "amount", alias = "price_amount")]
    pub price_amount: Option<f64>,
    #[serde(alias = "currency", alias = "price_currency")]
    pub price_currency: Option<String>,
    #[serde(alias = "pay_amount")]
    pub pay_amount: Option<String>,
    #[serde(alias = "pay_token")]
    pub pay_token: Option<String>,
    #[serde(alias = "pay_network")]
    pub pay_network: Option<String>,
    #[serde(alias = "deposit_address")]
    pub deposit_address: Option<String>,
    #[serde(alias = "tx_hash")]
    pub tx_hash: Option<String>,
    pub status: String,
    pub timestamp: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}
