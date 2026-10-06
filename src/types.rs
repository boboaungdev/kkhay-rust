use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvoiceRequest {
    pub price_amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_currency: Option<String>,
    pub pay_network: String,
    pub pay_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipn_callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub id: String,
    pub merchant_id: String,
    pub order_id: Option<String>,
    pub title: Option<String>,
    pub price_amount: f64,
    pub price_currency: String,
    pub pay_amount: String,
    pub pay_token: String,
    pub pay_network: String,
    pub deposit_address: String,
    pub status: String,
    pub fee_amount: String,
    pub net_amount: String,
    pub hosted_url: String,
    pub expires_at: String,
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
pub struct WebhookEvent {
    pub event: String,
    pub invoice_id: String,
    pub order_id: Option<String>,
    pub price_amount: f64,
    pub price_currency: String,
    pub pay_amount: String,
    pub pay_token: String,
    pub pay_network: String,
    pub deposit_address: String,
    pub tx_hash: Option<String>,
    pub status: String,
    pub timestamp: String,
}

