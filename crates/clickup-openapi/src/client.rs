use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION},
    Method,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use url::Url;

use crate::{
    auth::{ClickUpAuth, ClickUpConfig},
    error::{ClickUpError, Result},
};

/// An async HTTP client for the ClickUp API.
///
/// `ClickUpClient` wraps a `reqwest::Client` and a [`ClickUpConfig`] to provide
/// typed methods for interacting with ClickUp tasks and workspaces.
///
/// # Examples
///
/// Load credentials from environment variables:
///
/// ```no_run
/// # use clickup_openapi::ClickUpClient;
/// let client = ClickUpClient::from_env()?;
/// # Ok::<_, clickup_openapi::ClickUpError>(())
/// ```
///
/// Build a client explicitly:
///
/// ```no_run
/// # use clickup_openapi::{ClickUpAuth, ClickUpClient};
/// let client = ClickUpClient::builder()
///     .auth(ClickUpAuth::personal("pk_..."))
///     .build()?;
/// # Ok::<_, clickup_openapi::ClickUpError>(())
/// ```
#[derive(Clone, Debug)]
pub struct ClickUpClient {
    http: reqwest::Client,
    config: ClickUpConfig,
}

impl ClickUpClient {
    /// Creates a new client from the given [`ClickUpConfig`].
    ///
    /// # Errors
    ///
    /// Returns an error if the authorization header value is invalid.
    pub fn new(config: ClickUpConfig) -> Result<Self> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        default_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&config.auth().header_value())?,
        );

        let http = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()?;

        Ok(Self { http, config })
    }

    /// Creates a client by reading credentials from environment variables.
    ///
    /// Checks `CLICKUP_API_TOKEN`, `CLICKUP_ACCESS_TOKEN`, and
    /// `CLICKUP_OAUTH_ACCESS_TOKEN` in that order. An optional
    /// `CLICKUP_API_BASE_URL` overrides the default API base URL.
    ///
    /// # Errors
    ///
    /// Returns [`ClickUpError::MissingAuthToken`] if no credential env var is set.
    pub fn from_env() -> Result<Self> {
        Self::new(ClickUpConfig::from_env()?)
    }

    /// Returns a [`ClickUpClientBuilder`] for configuring a client step by step.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clickup_openapi::{ClickUpAuth, ClickUpClient};
    /// let client = ClickUpClient::builder()
    ///     .auth(ClickUpAuth::personal("pk_..."))
    ///     .base_url("https://api.clickup.com/")?
    ///     .build()?;
    /// # Ok::<_, clickup_openapi::ClickUpError>(())
    /// ```
    pub fn builder() -> ClickUpClientBuilder {
        ClickUpClientBuilder::new()
    }

    /// Returns the configuration used by this client.
    pub fn config(&self) -> &ClickUpConfig {
        &self.config
    }

    /// Sends a GET request and deserializes the JSON response.
    ///
    /// # Errors
    ///
    /// Returns an error on network failures, non-2xx responses, or JSON decode errors.
    pub async fn get_json<T>(&self, path: &str, query: &[(String, String)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request_json::<T, Value>(Method::GET, path, query, None)
            .await
    }

    /// Sends a POST request with a JSON body and deserializes the JSON response.
    ///
    /// # Errors
    ///
    /// Returns an error on network failures, non-2xx responses, or JSON decode errors.
    pub async fn post_json<T, B>(
        &self,
        path: &str,
        query: &[(String, String)],
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        self.request_json(Method::POST, path, query, Some(body))
            .await
    }

    /// Sends an HTTP request and deserializes the JSON response.
    ///
    /// # Errors
    ///
    /// Returns [`ClickUpError::Http`] for non-2xx responses and
    /// [`ClickUpError::Decode`] for deserialization failures.
    pub async fn request_json<T, B>(
        &self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<&B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = self.build_url(path, query)?;
        let mut request = self.http.request(method, url);

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(ClickUpError::Http {
                status,
                message: extract_error_message(&body),
            });
        }

        serde_json::from_str(&body).map_err(|source| ClickUpError::Decode { source, body })
    }

    fn build_url(&self, path: &str, query: &[(String, String)]) -> Result<Url> {
        let mut url = self.config.base_url().join(path.trim_start_matches('/'))?;

        if !query.is_empty() {
            let mut pairs = url.query_pairs_mut();
            for (name, value) in query {
                pairs.append_pair(name, value);
            }
        }

        Ok(url)
    }
}

fn extract_error_message(body: &str) -> String {
    if let Ok(value) = serde_json::from_str::<Value>(body) {
        if let Some(message) = value.get("message").and_then(Value::as_str) {
            return message.to_owned();
        }
        if let Some(message) = value.get("err").and_then(Value::as_str) {
            return message.to_owned();
        }
        if let Some(message) = value.get("error").and_then(Value::as_str) {
            return message.to_owned();
        }
    }

    body.trim().to_owned()
}

/// A builder for [`ClickUpClient`].
///
/// Obtain one via [`ClickUpClient::builder()`].
///
/// # Examples
///
/// ```no_run
/// # use clickup_openapi::{ClickUpAuth, ClickUpClient};
/// let client = ClickUpClient::builder()
///     .auth(ClickUpAuth::personal("pk_..."))
///     .build()?;
/// # Ok::<_, clickup_openapi::ClickUpError>(())
/// ```
#[derive(Debug, Default)]
pub struct ClickUpClientBuilder {
    auth: Option<ClickUpAuth>,
    base_url: Option<String>,
}

impl ClickUpClientBuilder {
    fn new() -> Self {
        Self::default()
    }

    /// Sets the authentication credentials.
    pub fn auth(mut self, auth: ClickUpAuth) -> Self {
        self.auth = Some(auth);
        self
    }

    /// Sets a custom base URL for the ClickUp API.
    ///
    /// Defaults to `https://api.clickup.com/`. The URL is validated eagerly.
    ///
    /// # Errors
    ///
    /// Returns an error if `base_url` is not a valid URL.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Result<Self> {
        let raw = base_url.into();
        url::Url::parse(&raw).map_err(ClickUpError::InvalidBaseUrl)?;
        self.base_url = Some(raw);
        Ok(self)
    }

    /// Builds the [`ClickUpClient`].
    ///
    /// If no auth was provided, falls back to reading from environment variables.
    ///
    /// # Errors
    ///
    /// Returns [`ClickUpError::MissingAuthToken`] if no auth was provided and
    /// no auth env var is set.
    pub fn build(self) -> Result<ClickUpClient> {
        let config = match (self.auth, self.base_url) {
            (Some(auth), Some(base_url)) => ClickUpConfig::with_base_url(auth, base_url)?,
            (Some(auth), None) => ClickUpConfig::new(auth)?,
            (None, Some(base_url)) => {
                let auth = ClickUpAuth::from_env()?;
                ClickUpConfig::with_base_url(auth, base_url)?
            }
            (None, None) => ClickUpConfig::from_env()?,
        };
        ClickUpClient::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::ClickUpClient;
    use crate::auth::{ClickUpAuth, ClickUpConfig};

    const TEST_WORKSPACE_ID: &str = "42";
    const TEST_CUSTOM_TASK_ID: &str = "TASK-123";

    #[test]
    fn build_url_keeps_clickup_api_prefix_for_v2_paths() {
        let client =
            ClickUpClient::new(ClickUpConfig::new(ClickUpAuth::personal("pk_123")).unwrap())
                .unwrap();

        let url = client
            .build_url(
                &format!("/api/v2/task/{TEST_CUSTOM_TASK_ID}"),
                &[
                    ("custom_task_ids".to_owned(), "true".to_owned()),
                    ("team_id".to_owned(), TEST_WORKSPACE_ID.to_owned()),
                    ("include_subtasks".to_owned(), "true".to_owned()),
                ],
            )
            .unwrap();

        assert_eq!(
            url.as_str(),
            "https://api.clickup.com/api/v2/task/TASK-123?custom_task_ids=true&team_id=42&include_subtasks=true"
        );
    }

    #[test]
    fn builder_with_explicit_auth_builds_successfully() {
        let client = ClickUpClient::builder()
            .auth(ClickUpAuth::personal("pk_test_123"))
            .build();
        assert!(client.is_ok());
    }

    #[test]
    fn builder_base_url_rejects_invalid_url() {
        let result = ClickUpClient::builder().base_url("not a url");
        assert!(result.is_err());
    }
}
