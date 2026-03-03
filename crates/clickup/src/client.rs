use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION},
    Method,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use url::Url;

use crate::{
    auth::ClickUpConfig,
    error::{ClickUpError, Result},
};

#[derive(Clone, Debug)]
pub struct ClickUpClient {
    http: reqwest::Client,
    config: ClickUpConfig,
}

impl ClickUpClient {
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

    pub fn from_env() -> Result<Self> {
        Self::new(ClickUpConfig::from_env()?)
    }

    pub fn config(&self) -> &ClickUpConfig {
        &self.config
    }

    pub async fn get_json<T>(&self, path: &str, query: &[(String, String)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request_json::<T, Value>(Method::GET, path, query, None)
            .await
    }

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
}
