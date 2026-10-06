use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use std::time::Duration;

use crate::error::KkhayError;
use crate::types::{
    CreateInvoiceRequest, CreateInvoiceResponse, GetInvoiceResponse, ListInvoicesResponse,
};

#[derive(Clone, Debug)]
pub struct KkhayClient {
    api_key: String,
    base_url: String,
    http: reqwest::Client,
}

impl KkhayClient {
    /// Create a new K Khay Client instance.
    pub fn new(api_key: impl Into<String>) -> Result<Self, KkhayError> {
        Self::with_options(api_key, "https://api.kkhay.com", Duration::from_secs(30))
    }

    /// Create a client with custom base URL and timeout.
    pub fn with_options(
        api_key: impl Into<String>,
        base_url: impl Into<String>,
        timeout: Duration,
    ) -> Result<Self, KkhayError> {
        let key = api_key.into().trim().to_string();
        if key.is_empty() {
            return Err(KkhayError::Config("API key cannot be empty".to_string()));
        }

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(&key)
                .map_err(|e| KkhayError::Config(format!("Invalid API key characters: {}", e)))?,
        );
        headers.insert(
            "User-Agent",
            HeaderValue::from_static("kkhay-rust/1.0.0"),
        );

        let http = reqwest::Client::builder()
            .timeout(timeout)
            .default_headers(headers)
            .build()?;

        let mut base = base_url.into();
        while base.ends_with('/') {
            base.pop();
        }

        Ok(Self {
            api_key: key,
            base_url: base,
            http,
        })
    }

    fn get_url(&self, path: &str) -> String {
        let clean_path = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{}", path)
        };

        if self.base_url.ends_with("/api") || self.base_url.contains("api.") {
            format!("{}{}", self.base_url, clean_path)
        } else {
            format!("{}/api{}", self.base_url, clean_path)
        }
    }

    /// Create a new crypto payment invoice.
    pub async fn create_invoice(
        &self,
        request: &CreateInvoiceRequest,
    ) -> Result<CreateInvoiceResponse, KkhayError> {
        if request.price_amount <= 0.0 {
            return Err(KkhayError::Config(
                "price_amount must be greater than 0".to_string(),
            ));
        }

        let url = self.get_url("/v1/merchant/invoices");
        let resp = self
            .http
            .post(&url)
            .header(CONTENT_TYPE, "application/json")
            .json(request)
            .send()
            .await?;

        let status = resp.status().as_u16();
        if status >= 400 {
            let body = resp.text().await.unwrap_or_default();
            return Err(KkhayError::Api {
                status,
                message: body,
                code: None,
            });
        }

        let result = resp.json::<CreateInvoiceResponse>().await?;
        Ok(result)
    }

    /// Retrieve an invoice and its blockchain confirmations by UUID.
    pub async fn get_invoice(&self, invoice_id: &str) -> Result<GetInvoiceResponse, KkhayError> {
        let clean_id = invoice_id.trim();
        if clean_id.is_empty() {
            return Err(KkhayError::Config("invoice_id cannot be empty".to_string()));
        }

        let url = self.get_url(&format!("/v1/merchant/invoices/{}", clean_id));
        let resp = self.http.get(&url).send().await?;

        let status = resp.status().as_u16();
        if status >= 400 {
            let body = resp.text().await.unwrap_or_default();
            return Err(KkhayError::Api {
                status,
                message: body,
                code: None,
            });
        }

        let result = resp.json::<GetInvoiceResponse>().await?;
        Ok(result)
    }

    /// List and paginate merchant invoices.
    pub async fn list_invoices(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        status: Option<&str>,
    ) -> Result<ListInvoicesResponse, KkhayError> {
        let mut url = self.get_url("/v1/merchant/invoices");
        let mut query = Vec::new();
        if let Some(p) = page {
            query.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            query.push(format!("limit={}", l));
        }
        if let Some(s) = status {
            query.push(format!("status={}", s));
        }

        if !query.is_empty() {
            url = format!("{}?{}", url, query.join("&"));
        }

        let resp = self.http.get(&url).send().await?;
        let status_code = resp.status().as_u16();
        if status_code >= 400 {
            let body = resp.text().await.unwrap_or_default();
            return Err(KkhayError::Api {
                status: status_code,
                message: body,
                code: None,
            });
        }

        let result = resp.json::<ListInvoicesResponse>().await?;
        Ok(result)
    }
}

