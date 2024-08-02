use reqwest::{Client, Response};

use crate::errors::{PulseError, TokenExpiredError, TokenNotFoundError, TokenUsedError};
use crate::types::{APIClassifyPayload, APIClassifyResponse};

pub struct Pulse {
    client: Client,
    site_key: String,
    secret_key: String,
    api_url: String,
}

impl Pulse {
    pub fn new(site_key: String, secret_key: String) -> Self {
        Pulse::with_url(
            site_key,
            secret_key,
            "https://api.pulsesecurity.org".to_string(),
        )
    }

    pub fn with_url(site_key: String, secret_key: String, api_url: String) -> Self {
        Pulse {
            client: Client::new(),
            site_key,
            secret_key,
            api_url,
        }
    }

    pub async fn classify(&self, token: String) -> Result<bool, PulseError> {
        let payload = APIClassifyPayload {
            token,
            site_key: self.site_key.clone(),
            secret_key: self.secret_key.clone(),
        };

        let response: Response = self
            .client
            .post(format!("{}/api/classify", self.api_url))
            .json(&payload)
            .send()
            .await
            .map_err(|e| PulseError::UnknownError(format!("Request failed: {}", e)))?;

        let data: APIClassifyResponse = response
            .json()
            .await
            .map_err(|e| PulseError::UnknownError(format!("Failed to get response data: {}", e)))?;

        match data {
            APIClassifyResponse::IsBot { is_bot } => Ok(is_bot),
            APIClassifyResponse::Errors { errors } => {
                if errors.is_empty() {
                    return Err(PulseError::UnknownError(
                        "Unknown error (no error returned)".to_string(),
                    ));
                }

                let error = &errors[0].clone();
                match error.code.as_str() {
                    "TOKEN_NOT_FOUND" => Err(TokenNotFoundError(error.clone()).into()),
                    "TOKEN_USED" => Err(TokenUsedError(error.clone()).into()),
                    "TOKEN_EXPIRED" => Err(TokenExpiredError(error.clone()).into()),
                    code => Err(PulseError::UnknownError(format!(
                        "Unknown error code: {}",
                        code
                    ))),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{TokenExpiredError, TokenNotFoundError, TokenUsedError};
    use crate::types::APIErrorData;
    use mockito::Server;

    #[tokio::test]
    async fn test_classify_bot() {
        let mut m = Server::new_async().await;

        let classify_mock = m
            .mock("POST", "/api/classify")
            .with_status(200)
            .with_body(r#"{"is_bot": true}"#)
            .create_async()
            .await;

        let api = Pulse::with_url("siteKey".to_string(), "siteSecret".to_string(), m.url());
        let result = api.classify("token".to_string()).await.unwrap();

        classify_mock.assert_async().await;
        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn test_handle_errors() {
        let error_tests: Vec<(&str, PulseError)> = vec![
            (
                "TOKEN_NOT_FOUND",
                PulseError::TokenNotFoundError(TokenNotFoundError(APIErrorData {
                    code: "TOKEN_NOT_FOUND".to_string(),
                    error: "Test error message".to_string(),
                })),
            ),
            (
                "TOKEN_USED",
                PulseError::TokenUsedError(TokenUsedError(APIErrorData {
                    code: "TOKEN_USED".to_string(),
                    error: "Test error message".to_string(),
                })),
            ),
            (
                "TOKEN_EXPIRED",
                PulseError::TokenExpiredError(TokenExpiredError(APIErrorData {
                    code: "TOKEN_EXPIRED".to_string(),
                    error: "Test error message".to_string(),
                })),
            ),
        ];

        for (code, expected_error) in error_tests {
            let mut m = Server::new_async().await;

            let classify_mock = m
                .mock("POST", "/api/classify")
                .with_status(400)
                .with_body(format!(
                    r#"{{"errors": [{{"code": "{}", "error": "Test error message"}}]}}"#,
                    code
                ))
                .create();

            let api = Pulse::with_url("siteKey".to_string(), "siteSecret".to_string(), m.url());
            let result = api.classify("token".to_string()).await;

            classify_mock.assert_async().await;

            // Match on the result to check for the expected error type
            match result {
                Ok(_) => panic!("Expected an error but got an Ok result."),
                Err(err) => {
                    // Use PartialEq to compare the errors directly
                    assert_eq!(err, expected_error, "Error does not match");
                }
            }
        }
    }
}
