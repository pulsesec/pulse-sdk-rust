use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClassifyPayload {
    pub token: String,
    pub site_key: String,
    pub secret_key: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum APIClassifyResponse {
    IsBot { is_bot: bool },
    Errors { errors: Vec<APIErrorData> },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct APIErrorData {
    pub error: String,
    pub code: String,
}
