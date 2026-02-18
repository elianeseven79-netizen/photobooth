use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatPayRequest {
    appid: String,
    mch_id: String,
    nonce_str: String,
    sign: String,
    body: String,
    out_trade_no: String,
    total_fee: i32,
    spbill_create_ip: String,
    notify_url: String,
    trade_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatPayResponse {
    return_code: String,
    return_msg: String,
    result_code: String,
    prepay_id: String,
    qr_code: String,
}

pub struct WeChatService {
    app_id: String,
    mch_id: String,
    api_key: String,
    notify_url: String,
}

impl WeChatService {
    pub fn new() -> Result<Self, String> {
        let app_id = env::var("WECHAT_APP_ID").map_err(|_| "WECHAT_APP_ID not set")?;
        let mch_id = env::var("WECHAT_MCH_ID").map_err(|_| "WECHAT_MCH_ID not set")?;
        let api_key = env::var("WECHAT_API_KEY").map_err(|_| "WECHAT_API_KEY not set")?;
        let notify_url = env::var("WECHAT_NOTIFY_URL").unwrap_or_default();

        Ok(Self {
            app_id,
            mch_id,
            api_key,
            notify_url,
        })
    }

    pub fn create_qr_code_url(&self, prepay_id: &str) -> String {
        format!(
            "weixin://wxpay/bizpayurl?pr={}",
            prepay_id
        )
    }

    // Stub: In production, this would call the actual WeChat Pay API
    pub async fn create_order(&self, order_id: &str, amount: i32, description: &str) -> Result<(String, String), String> {
        // Return mock data for development
        // In production, this would call WeChat's unifiedorder API
        let prepay_id = format!("prepay_{}_{}", order_id, chrono::Utc::now().timestamp_millis());
        let qr_code = self.create_qr_code_url(&prepay_id);

        tracing::info!("WeChat order created: {} for {} cents", order_id, amount);

        Ok((prepay_id, qr_code))
    }

    // Stub: In production, this would query the actual WeChat Pay API
    pub async fn query_order(&self, wechat_order_id: &str) -> Result<String, String> {
        // Return mock data for development
        // In production, this would call WeChat's orderquery API
        tracing::info!("WeChat order queried: {}", wechat_order_id);

        // Simulate: return "SUCCESS" meaning paid
        Ok("SUCCESS".to_string())
    }
}

impl Default for WeChatService {
    fn default() -> Self {
        Self::new().expect("Failed to create WeChatService")
    }
}
