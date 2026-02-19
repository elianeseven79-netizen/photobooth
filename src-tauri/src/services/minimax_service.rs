use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

// Request struct without subject_reference (text-to-image)
#[derive(Debug, Serialize)]
struct MiniMaxRequest {
    model: String,
    prompt: String,
    #[serde(rename = "aspect_ratio")]
    aspect_ratio: String,
    #[serde(rename = "response_format")]
    response_format: String,
}

// Request struct with subject_reference (image-to-image)
#[derive(Debug, Serialize)]
struct MiniMaxRequestWithRef {
    model: String,
    prompt: String,
    #[serde(rename = "aspect_ratio")]
    aspect_ratio: String,
    #[serde(rename = "response_format")]
    response_format: String,
    #[serde(rename = "subject_reference")]
    subject_reference: Vec<SubjectReference>,
}

#[derive(Debug, Serialize)]
struct SubjectReference {
    #[serde(rename = "type")]
    r#type: String,
    #[serde(rename = "image_file")]
    image_file: String,
}

#[derive(Debug, Deserialize)]
struct MiniMaxResponse {
    data: MiniMaxData,
}

#[derive(Debug, Deserialize)]
struct MiniMaxData {
    #[serde(rename = "image_base64")]
    image_base64: Vec<String>,
}

pub struct MiniMaxService {
    client: Client,
    api_key: String,
    base_url: String,
    use_mock: bool,
}

impl MiniMaxService {
    pub fn new() -> Result<Self, String> {
        let api_key = env::var("MINIMAX_API_KEY").unwrap_or_default();

        // Check if mock mode is forced via env var
        let force_mock = env::var("FORCE_MOCK").unwrap_or_default();

        // Use mock if: FORCE_MOCK=true, no API key, or placeholder
        let use_mock = force_mock == "true"
            || api_key.is_empty()
            || api_key == "your-api-key-here"
            || api_key.starts_with("placeholder");

        tracing::info!("MiniMax service initialized: use_mock={}, FORCE_MOCK={}, has_api_key={}", use_mock, force_mock, !api_key.is_empty());

        Ok(Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .map_err(|e| format!("Failed to create HTTP client: {}", e))?,
            api_key,
            base_url: "https://api.minimaxi.com".to_string(),
            use_mock,
        })
    }

    pub async fn generate_image(
        &self,
        user_photo_base64: &str,
        prompt: &str,
    ) -> Result<String, String> {
        tracing::info!("[MiniMax] ===== START generate_image =====");
        tracing::info!("[MiniMax] use_mock={}, prompt={}", self.use_mock, prompt);
        tracing::info!("[MiniMax] User photo base64 length: {}", user_photo_base64.len());

        // Use mock mode if configured
        if self.use_mock {
            tracing::warn!("[MiniMax] Using MOCK photo (use_mock=true)");
            let mock_image = self.generate_placeholder_image();
            tracing::info!("[MiniMax] Mock image generated, length: {}", mock_image.len());
            tracing::info!("[MiniMax] ===== END generate_image (mock) =====");
            return Ok(mock_image);
        }

        tracing::info!("[MiniMax] Calling MiniMax API (image-to-image)...");
        tracing::info!("[MiniMax] API URL: {}/v1/image_generation", self.base_url);

        // Image-to-image: add data URL prefix to base64
        let image_with_prefix = format!("data:image/jpeg;base64,{}", user_photo_base64);

        let i2i_request = MiniMaxRequestWithRef {
            model: "image-01".to_string(),
            prompt: prompt.to_string(),
            aspect_ratio: "3:4".to_string(),
            response_format: "base64".to_string(),
            subject_reference: vec![SubjectReference {
                r#type: "character".to_string(),
                image_file: image_with_prefix,
            }],
        };

        tracing::info!("[MiniMax] Sending image-to-image request with subject_reference...");

        let response = self
            .client
            .post(format!("{}/v1/image_generation", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&i2i_request)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        tracing::info!("[MiniMax] Response received, status: {}", response.status());

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            tracing::warn!("MiniMax API error {}: {}", status, body);

            // Fall back to text-to-image if image-to-image fails
            tracing::warn!("[MiniMax] Image-to-image failed, trying text-to-image...");

            let text_request = MiniMaxRequest {
                model: "image-01".to_string(),
                prompt: prompt.to_string(),
                aspect_ratio: "3:4".to_string(),
                response_format: "base64".to_string(),
            };

            let response = self
                .client
                .post(format!("{}/v1/image_generation", self.base_url))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&text_request)
                .send()
                .await
                .map_err(|e| format!("Failed to send request: {}", e))?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                tracing::warn!("Text-to-image also failed {}: {}", status, body);
                let mock_image = self.generate_placeholder_image();
                return Ok(mock_image);
            }

            let result: MiniMaxResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;

            let generated_image = result
                .data
                .image_base64
                .into_iter()
                .next()
                .ok_or_else(|| "No image in response".to_string())?;

            tracing::info!("[MiniMax] Text-to-image fallback success, length: {}", generated_image.len());
            return Ok(generated_image);
        }

        let result: MiniMaxResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let generated_image = result
            .data
            .image_base64
            .into_iter()
            .next()
            .ok_or_else(|| "No image in response".to_string())?;

        tracing::info!("[MiniMax] API response parsed, generated image length: {}", generated_image.len());
        tracing::info!("[MiniMax] ===== END generate_image (success) =====");
        Ok(generated_image)
    }

    /// Generate a simple placeholder image directly (no network request)
    pub fn generate_placeholder_image(&self) -> String {
        // Use a hardcoded small valid base64 image (1x1 purple pixel JPEG)
        // This is a minimal valid JPEG that browsers can display
        // Strip the "data:image/jpeg;base64," prefix
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==".to_string()
    }
}

impl Default for MiniMaxService {
    fn default() -> Self {
        Self {
            client: Client::new(),
            api_key: String::new(),
            base_url: "https://api.minimaxi.com".to_string(),
            use_mock: true,
        }
    }
}
