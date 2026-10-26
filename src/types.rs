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
    #[serde(alias = "amount")]
    pub price_amount: f64,
    #[serde(skip_serializing_if = "Option::is_none", alias = "currency")]
    pub price_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "returnUrl")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "callbackUrl")]
    pub ipn_callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub id: String,
    #[serde(alias = "gatewayId")]
    pub gateway_id: Option<String>,
    pub order_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(alias = "amount")]
    pub price_amount: f64,
    #[serde(alias = "currency")]
    pub price_currency: String,
    pub pay_amount: Option<String>,
    pub pay_token: Option<String>,
    pub pay_network: Option<String>,
    pub deposit_address: Option<String>,
    pub status: String,
    pub fee_amount: Option<String>,
    pub net_amount: Option<String>,
    #[serde(alias = "checkout_url", alias = "checkoutUrl")]
    pub hosted_url: String,
    pub expires_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRecord {
    pub id: String,
    pub tx_hash: String,
    pub amount_received: String,
    pub confirmations: u32,
    pub status: String,
    pub forwarded_tx_hash: Option<String>,
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
    pub total_count: u64,
    pub total_pages: u64,
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
    #[serde(alias = "id", alias = "invoice_id")]
    pub invoice_id: String,
    pub order_id: Option<String>,
    #[serde(alias = "amount")]
    pub price_amount: Option<f64>,
    #[serde(alias = "currency")]
    pub price_currency: Option<String>,
    pub pay_amount: Option<String>,
    pub pay_token: Option<String>,
    pub pay_network: Option<String>,
    pub deposit_address: Option<String>,
    pub tx_hash: Option<String>,
    pub status: String,
    pub timestamp: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}
