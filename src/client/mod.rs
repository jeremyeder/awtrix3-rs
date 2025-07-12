pub mod http;

use crate::error::{AwtrixError, Result};
use std::time::Duration;
use url::Url;
// Models are re-exported from the main crate
use once_cell::sync::Lazy;

/// Global HTTP client for connection pooling
static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .pool_max_idle_per_host(10)
        .timeout(Duration::from_secs(30))
        .gzip(true)
        .brotli(true)
        .build()
        .expect("Failed to create HTTP client")
});

/// AWTRIX3 client for interacting with the device
#[derive(Debug, Clone)]
pub struct Client {
    base_url: Url,
    client: reqwest::Client,
}

impl Client {
    /// Create a new client with the device host
    pub fn new<S: AsRef<str>>(host: S) -> Result<Self> {
        let base_url =
            if host.as_ref().starts_with("http://") || host.as_ref().starts_with("https://") {
                Url::parse(host.as_ref())?
            } else {
                Url::parse(&format!("http://{}", host.as_ref()))?
            };

        Ok(Self {
            base_url,
            client: HTTP_CLIENT.clone(),
        })
    }

    /// Create a new client with a custom HTTP client
    pub fn with_client<S: AsRef<str>>(host: S, client: reqwest::Client) -> Result<Self> {
        let base_url =
            if host.as_ref().starts_with("http://") || host.as_ref().starts_with("https://") {
                Url::parse(host.as_ref())?
            } else {
                Url::parse(&format!("http://{}", host.as_ref()))?
            };

        Ok(Self { base_url, client })
    }

    /// Get the base URL
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Build a URL for an API endpoint
    fn build_url(&self, endpoint: &str) -> Result<Url> {
        self.base_url.join(endpoint).map_err(|e| e.into())
    }
}

/// Builder for creating a customized client
pub struct ClientBuilder {
    host: Option<String>,
    timeout: Duration,
    retry_count: u32,
    user_agent: String,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            host: None,
            timeout: Duration::from_secs(30),
            retry_count: 3,
            user_agent: format!("awtrix3-rs/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    pub fn host<S: Into<String>>(mut self, host: S) -> Self {
        self.host = Some(host.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn retry_count(mut self, count: u32) -> Self {
        self.retry_count = count;
        self
    }

    pub fn user_agent<S: Into<String>>(mut self, user_agent: S) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    pub fn build(self) -> Result<Client> {
        let host = self
            .host
            .ok_or_else(|| AwtrixError::Config("Host is required".to_string()))?;

        let http_client = reqwest::Client::builder()
            .timeout(self.timeout)
            .user_agent(self.user_agent)
            .gzip(true)
            .brotli(true)
            .build()
            .map_err(|e| AwtrixError::Config(format!("Failed to build HTTP client: {}", e)))?;

        Client::with_client(host, http_client)
    }
}
