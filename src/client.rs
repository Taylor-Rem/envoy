use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatRequest {
    pub device_id: i64,
    pub conversation_id: Option<u64>,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub conversation_id: u64,
    pub content: String,
}

pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    pub async fn chat(
        &self,
        device_id: i64,
        conversation_id: Option<u64>,
        message: String,
    ) -> Result<ChatResponse> {
        let url = format!("{}/chat", self.base_url);

        let request = ChatRequest {
            device_id,
            conversation_id,
            message,
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .json::<ChatResponse>()
            .await?;

        Ok(response)
    }

    pub async fn register_device(&self, device_name: String) -> Result<i64> {
        let url = format!("{}/devices/register", self.base_url);

        let response = self.client
            .post(&url)
            .json(&serde_json::json!({ "device_name": device_name }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let device_id = response["device_id"]
            .as_i64()
            .ok_or_else(|| anyhow::anyhow!("Invalid device_id in response"))?;

        Ok(device_id)
    }
}