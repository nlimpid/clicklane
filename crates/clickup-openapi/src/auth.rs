use std::env;

use url::Url;

use crate::error::{ClickUpError, Result};

const DEFAULT_BASE_URL: &str = "https://api.clickup.com/";

/// The authentication method used when talking to the ClickUp API.
///
/// ClickUp supports two token types:
/// - **Personal API key** (`pk_...`) — sent as the bare `Authorization` header value.
/// - **OAuth access token** — sent as `Bearer <token>`.
///
/// # Examples
///
/// ```
/// use clickup_openapi::ClickUpAuth;
///
/// let personal = ClickUpAuth::personal("pk_live_abc123");
/// let oauth    = ClickUpAuth::oauth("my_oauth_token");
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClickUpAuth {
    /// A ClickUp personal API key (starts with `pk_`).
    PersonalApiKey(String),
    /// An OAuth 2.0 access token, sent with a `Bearer` prefix.
    OAuthAccessToken(String),
}

impl ClickUpAuth {
    /// Creates a [`ClickUpAuth::PersonalApiKey`] from a token string.
    ///
    /// Leading and trailing whitespace is trimmed.
    pub fn personal(token: impl Into<String>) -> Self {
        Self::PersonalApiKey(normalize_secret(token))
    }

    /// Creates a [`ClickUpAuth::OAuthAccessToken`] from a token string.
    ///
    /// Leading and trailing whitespace is trimmed.
    pub fn oauth(token: impl Into<String>) -> Self {
        Self::OAuthAccessToken(normalize_secret(token))
    }

    /// Parses a raw `Authorization` header value into the correct variant.
    ///
    /// Values starting with `"Bearer "` are treated as OAuth tokens; everything
    /// else is treated as a personal API key.
    pub fn from_authorization_value(value: impl Into<String>) -> Self {
        let value = normalize_secret(value);
        if let Some(token) = value.strip_prefix("Bearer ") {
            return Self::oauth(token);
        }

        Self::personal(value)
    }

    /// Reads credentials from standard ClickUp environment variables.
    ///
    /// Variables checked in order:
    /// 1. `CLICKUP_API_TOKEN` — treated via [`ClickUpAuth::from_authorization_value`]
    /// 2. `CLICKUP_ACCESS_TOKEN` — treated as an OAuth token
    /// 3. `CLICKUP_OAUTH_ACCESS_TOKEN` — treated as an OAuth token
    ///
    /// # Errors
    ///
    /// Returns [`ClickUpError::MissingAuthToken`] if none of the variables are set.
    pub fn from_env() -> Result<Self> {
        if let Ok(token) = env::var("CLICKUP_API_TOKEN") {
            return Ok(Self::from_authorization_value(token));
        }

        if let Ok(token) = env::var("CLICKUP_ACCESS_TOKEN") {
            return Ok(Self::oauth(token));
        }

        if let Ok(token) = env::var("CLICKUP_OAUTH_ACCESS_TOKEN") {
            return Ok(Self::oauth(token));
        }

        Err(ClickUpError::MissingAuthToken)
    }

    /// Returns the value to use for the `Authorization` HTTP header.
    ///
    /// Personal keys are returned as-is; OAuth tokens are prefixed with `"Bearer "`.
    pub fn header_value(&self) -> String {
        match self {
            Self::PersonalApiKey(token) => token.clone(),
            Self::OAuthAccessToken(token) => format!("Bearer {token}"),
        }
    }
}

/// Configuration for [`crate::ClickUpClient`]: auth credentials and API base URL.
#[derive(Clone, Debug)]
pub struct ClickUpConfig {
    auth: ClickUpAuth,
    base_url: Url,
}

impl ClickUpConfig {
    /// Creates a config targeting the default ClickUp API base URL.
    ///
    /// # Errors
    ///
    /// Returns an error if the default URL cannot be parsed (should not happen).
    pub fn new(auth: ClickUpAuth) -> Result<Self> {
        Self::with_base_url(auth, DEFAULT_BASE_URL)
    }

    /// Creates a config with a custom base URL.
    ///
    /// A trailing `/` is added if the URL does not already end with one.
    ///
    /// # Errors
    ///
    /// Returns [`ClickUpError::InvalidBaseUrl`] if `base_url` is not a valid URL.
    pub fn with_base_url(auth: ClickUpAuth, base_url: impl AsRef<str>) -> Result<Self> {
        let raw = base_url.as_ref().trim();
        let normalized = if raw.ends_with('/') {
            raw.to_owned()
        } else {
            format!("{raw}/")
        };

        Ok(Self {
            auth,
            base_url: Url::parse(&normalized)?,
        })
    }

    /// Creates a config by reading from environment variables.
    ///
    /// Delegates to [`ClickUpAuth::from_env`] and optionally reads
    /// `CLICKUP_API_BASE_URL` to override the default base URL.
    ///
    /// # Errors
    ///
    /// Returns an error if no auth env var is set or if `CLICKUP_API_BASE_URL`
    /// is not a valid URL.
    pub fn from_env() -> Result<Self> {
        let auth = ClickUpAuth::from_env()?;
        match env::var("CLICKUP_API_BASE_URL") {
            Ok(base_url) => Self::with_base_url(auth, base_url),
            Err(_) => Self::new(auth),
        }
    }

    /// Returns the authentication credentials.
    pub fn auth(&self) -> &ClickUpAuth {
        &self.auth
    }

    /// Returns the API base URL.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }
}

fn normalize_secret(value: impl Into<String>) -> String {
    value.into().trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::{ClickUpAuth, ClickUpConfig};

    #[test]
    fn personal_tokens_are_sent_as_is() {
        let auth = ClickUpAuth::personal("pk_123");
        assert_eq!(auth.header_value(), "pk_123");
    }

    #[test]
    fn oauth_tokens_are_sent_with_bearer_prefix() {
        let auth = ClickUpAuth::oauth("abc");
        assert_eq!(auth.header_value(), "Bearer abc");
    }

    #[test]
    fn base_url_is_normalized_with_trailing_slash() {
        let config =
            ClickUpConfig::with_base_url(ClickUpAuth::personal("pk_123"), "https://example.com")
                .expect("valid config");
        assert_eq!(config.base_url().as_str(), "https://example.com/");
    }
}
