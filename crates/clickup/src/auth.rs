use std::env;

use url::Url;

use crate::error::{ClickUpError, Result};

const DEFAULT_BASE_URL: &str = "https://api.clickup.com/";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClickUpAuth {
    PersonalApiKey(String),
    OAuthAccessToken(String),
}

impl ClickUpAuth {
    pub fn personal(token: impl Into<String>) -> Self {
        Self::PersonalApiKey(normalize_secret(token))
    }

    pub fn oauth(token: impl Into<String>) -> Self {
        Self::OAuthAccessToken(normalize_secret(token))
    }

    pub fn from_authorization_value(value: impl Into<String>) -> Self {
        let value = normalize_secret(value);
        if let Some(token) = value.strip_prefix("Bearer ") {
            return Self::oauth(token);
        }

        Self::personal(value)
    }

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

    pub fn header_value(&self) -> String {
        match self {
            Self::PersonalApiKey(token) => token.clone(),
            Self::OAuthAccessToken(token) => format!("Bearer {token}"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClickUpConfig {
    auth: ClickUpAuth,
    base_url: Url,
}

impl ClickUpConfig {
    pub fn new(auth: ClickUpAuth) -> Result<Self> {
        Self::with_base_url(auth, DEFAULT_BASE_URL)
    }

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

    pub fn from_env() -> Result<Self> {
        let auth = ClickUpAuth::from_env()?;
        match env::var("CLICKUP_API_BASE_URL") {
            Ok(base_url) => Self::with_base_url(auth, base_url),
            Err(_) => Self::new(auth),
        }
    }

    pub fn auth(&self) -> &ClickUpAuth {
        &self.auth
    }

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
