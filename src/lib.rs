//! Official Rust SDK for the Pigeon email API.
//!
//! Layout follows [resend-rust](https://github.com/resend/resend-rust): `Pigeon::new(api_key).emails.send(...)`.

mod emails;

pub use emails::{SendEmailRequest, SendEmailResponse};

use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{message}")]
    Api {
        status: u16,
        name: Option<String>,
        message: String,
    },
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

#[derive(Clone)]
pub struct Pigeon {
    pub emails: Emails,
}

#[derive(Clone)]
pub struct Emails {
    inner: Inner,
}

#[derive(Clone)]
struct Inner {
    api_key: String,
    base_url: String,
    http: Client,
}

impl Pigeon {
    pub fn new(api_key: impl Into<String>) -> Self {
        let api_key = api_key.into();
        let base_url = env::var("PIGEON_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:4005".into())
            .trim_end_matches('/')
            .to_string();
        let http = Client::new();
        let inner = Inner {
            api_key: api_key.clone(),
            base_url: base_url.clone(),
            http: http.clone(),
        };
        Self {
            emails: Emails { inner },
        }
    }
}

impl Emails {
    pub async fn send(&self, params: &SendEmailRequest) -> Result<SendEmailResponse, Error> {
        self.inner.post("/api/emails", params).await
    }
}

impl Inner {
    async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T, Error> {
        let response = self
            .http
            .post(format!("{}{path}", self.base_url))
            .bearer_auth(&self.api_key)
            .header("User-Agent", format!("pigeon-rust/{VERSION}"))
            .json(body)
            .send()
            .await?;

        let status = response.status();
        let value: serde_json::Value = response.json().await.unwrap_or(serde_json::Value::Null);
        if !status.is_success() {
            return Err(Error::Api {
                status: status.as_u16(),
                name: value.get("name").and_then(|v| v.as_str()).map(str::to_string),
                message: value
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("request failed")
                    .to_string(),
            });
        }
        Ok(serde_json::from_value(value).map_err(|err| Error::Api {
            status: status.as_u16(),
            name: Some("decode".into()),
            message: err.to_string(),
        })?)
    }
}
