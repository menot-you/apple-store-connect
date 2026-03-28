//! HTTP client for the App Store Connect API v1.
//!
//! Wraps [`reqwest::Client`] with automatic JWT authentication
//! via [`Credentials`].
//!
//! Includes automatic pagination (via `links.next`), rate-limit retry
//! (HTTP 429 + `Retry-After`), and raw byte downloads for reports.
//!
//! Domain endpoints live in [`client_endpoints`](crate::client_endpoints).

use std::sync::Arc;

use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::auth::{AuthError, Credentials};
use crate::models::{JsonApiResponse, Resource};

/// Maximum number of retries on HTTP 429 (rate limited) responses.
const MAX_RATE_LIMIT_RETRIES: u32 = 3;

/// Default `Retry-After` duration in seconds when the header is missing.
const DEFAULT_RETRY_AFTER_SECS: u64 = 5;

/// Errors that can occur when calling the App Store Connect API.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// Authentication/JWT error.
    #[error("auth error: {0}")]
    Auth(#[from] AuthError),
    /// HTTP transport error.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    /// Non-success HTTP status from the API.
    #[error("api error: status={status}, body={body}")]
    Status {
        /// HTTP status code.
        status: u16,
        /// Response body text.
        body: String,
    },
    /// Failed to deserialize the response body.
    #[error("deserialization error: {0}")]
    Deserialize(String),
    /// Rate limited after exhausting all retries.
    #[error("rate limited: exhausted {MAX_RATE_LIMIT_RETRIES} retries")]
    RateLimited,
    /// Sales report parsing error.
    #[error("sales report error: {0}")]
    SalesReport(String),
}

/// HTTP client for App Store Connect API.
#[derive(Clone)]
pub struct AscClient {
    http: Client,
    credentials: Arc<Credentials>,
    pub(crate) base_url: String,
}

impl AscClient {
    /// Creates a new client with the given credentials.
    pub fn new(credentials: Arc<Credentials>) -> Self {
        Self {
            http: Client::new(),
            credentials,
            base_url: "https://api.appstoreconnect.apple.com/v1".into(),
        }
    }

    /// Creates a client with a custom base URL (for testing).
    pub fn with_base_url(credentials: Arc<Credentials>, base_url: String) -> Self {
        Self {
            http: Client::new(),
            credentials,
            base_url,
        }
    }

    // -----------------------------------------------------------------
    // Core HTTP methods
    // -----------------------------------------------------------------

    /// Performs an authenticated GET request and deserializes the response.
    ///
    /// Retries up to 3 times on HTTP 429, respecting `Retry-After`.
    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        self.get_url(&url).await
    }

    /// Performs an authenticated GET on a full URL (for pagination).
    pub(crate) async fn get_url<T: DeserializeOwned>(&self, url: &str) -> Result<T, ApiError> {
        let token = self.credentials.token()?;
        for attempt in 0..=MAX_RATE_LIMIT_RETRIES {
            let resp = self.http.get(url).bearer_auth(&token).send().await?;
            let status = resp.status();
            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                if attempt == MAX_RATE_LIMIT_RETRIES {
                    return Err(ApiError::RateLimited);
                }
                let retry_after = Self::parse_retry_after(&resp);
                tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
                continue;
            }
            return self.handle_response(resp).await;
        }
        Err(ApiError::RateLimited)
    }

    /// Performs an authenticated POST request and deserializes the response.
    pub(crate) async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        let token = self.credentials.token()?;
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&token)
            .json(body)
            .send()
            .await?;
        self.handle_response(resp).await
    }

    /// Performs an authenticated GET returning raw bytes with a custom
    /// `Accept` header and optional query parameters.
    ///
    /// Used for non-JSON endpoints such as sales reports.
    pub(crate) async fn get_raw(
        &self,
        path: &str,
        query: &[(String, String)],
        accept: &str,
    ) -> Result<Vec<u8>, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        let token = self.credentials.token()?;
        for attempt in 0..=MAX_RATE_LIMIT_RETRIES {
            let resp = self
                .http
                .get(&url)
                .bearer_auth(&token)
                .header("Accept", accept)
                .query(query)
                .send()
                .await?;
            let status = resp.status();
            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                if attempt == MAX_RATE_LIMIT_RETRIES {
                    return Err(ApiError::RateLimited);
                }
                let retry_after = Self::parse_retry_after(&resp);
                tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
                continue;
            }
            if !status.is_success() {
                let body = resp.text().await.unwrap_or_default();
                return Err(ApiError::Status {
                    status: status.as_u16(),
                    body,
                });
            }
            return Ok(resp.bytes().await?.to_vec());
        }
        Err(ApiError::RateLimited)
    }

    // -----------------------------------------------------------------
    // Pagination
    // -----------------------------------------------------------------

    /// Fetches all pages of a paginated resource, following `links.next`.
    ///
    /// Collects every `data` item across all pages into a single `Vec`.
    pub async fn get_all_pages<A: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<Vec<Resource<A>>, ApiError> {
        let mut all_items = Vec::new();
        let first: JsonApiResponse<Vec<Resource<A>>> = self.get(path).await?;
        all_items.extend(first.data);
        let mut next_url = first.links.and_then(|l| l.next);
        while let Some(url) = next_url {
            let page: JsonApiResponse<Vec<Resource<A>>> = self.get_url(&url).await?;
            all_items.extend(page.data);
            next_url = page.links.and_then(|l| l.next);
        }
        Ok(all_items)
    }

    // -----------------------------------------------------------------
    // Response helpers
    // -----------------------------------------------------------------

    /// Handles an HTTP response: checks status, deserializes body.
    async fn handle_response<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T, ApiError> {
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_else(|e| {
                tracing::warn!("failed to read error response body: {e}");
                String::new()
            });
            return Err(ApiError::Status {
                status: status.as_u16(),
                body,
            });
        }
        let text = resp.text().await?;
        serde_json::from_str(&text).map_err(|e| ApiError::Deserialize(e.to_string()))
    }

    /// Parses the `Retry-After` header (in seconds) from a response.
    fn parse_retry_after(resp: &reqwest::Response) -> u64 {
        resp.headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(DEFAULT_RETRY_AFTER_SECS)
    }
}

#[cfg(test)]
#[path = "client_tests.rs"]
mod tests;
