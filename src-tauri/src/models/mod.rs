use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoMode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub id: String,
    pub mode_id: String,
    pub name: String,
    pub prompt: String,
    pub thumbnail: String,
    pub price_download: i32,
    pub price_print: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoSession {
    pub id: String,
    pub mode_id: String,
    pub effect_id: String,
    pub original_photo: Option<String>,  // Base64 encoded
    pub generated_photo: Option<String>, // Base64 encoded
    pub status: SessionStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    SelectingMode,
    SelectingEffect,
    Capturing,
    Processing,
    Previewing,
    Completed,
}

impl std::fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionStatus::SelectingMode => write!(f, "selecting_mode"),
            SessionStatus::SelectingEffect => write!(f, "selecting_effect"),
            SessionStatus::Capturing => write!(f, "capturing"),
            SessionStatus::Processing => write!(f, "processing"),
            SessionStatus::Previewing => write!(f, "previewing"),
            SessionStatus::Completed => write!(f, "completed"),
        }
    }
}

impl std::str::FromStr for SessionStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "selecting_mode" => Ok(SessionStatus::SelectingMode),
            "selecting_effect" => Ok(SessionStatus::SelectingEffect),
            "capturing" => Ok(SessionStatus::Capturing),
            "processing" => Ok(SessionStatus::Processing),
            "previewing" => Ok(SessionStatus::Previewing),
            "completed" => Ok(SessionStatus::Completed),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub session_id: String,
    pub order_type: OrderType,
    pub amount: i32,
    pub status: OrderStatus,
    pub wechat_order_id: Option<String>,
    pub payment_time: Option<i64>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Download,
    Print,
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::Download => write!(f, "download"),
            OrderType::Print => write!(f, "print"),
        }
    }
}

impl std::str::FromStr for OrderType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "download" => Ok(OrderType::Download),
            "print" => Ok(OrderType::Print),
            _ => Err(format!("Unknown order type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Paid,
    Cancelled,
    Refunded,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "pending"),
            OrderStatus::Paid => write!(f, "paid"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
            OrderStatus::Refunded => write!(f, "refunded"),
        }
    }
}

impl std::str::FromStr for OrderStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(OrderStatus::Pending),
            "paid" => Ok(OrderStatus::Paid),
            "cancelled" => Ok(OrderStatus::Cancelled),
            "refunded" => Ok(OrderStatus::Refunded),
            _ => Err(format!("Unknown order status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub session_id: String,
    pub current_step: Step,
    pub mode_id: Option<String>,
    pub effect_id: Option<String>,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Step {
    Home,
    SelectMode,
    SelectEffect,
    Capture,
    Preview,
    Payment,
    Download,
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::Home => write!(f, "home"),
            Step::SelectMode => write!(f, "select_mode"),
            Step::SelectEffect => write!(f, "select_effect"),
            Step::Capture => write!(f, "capture"),
            Step::Preview => write!(f, "preview"),
            Step::Payment => write!(f, "payment"),
            Step::Download => write!(f, "download"),
        }
    }
}

impl std::str::FromStr for Step {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "home" => Ok(Step::Home),
            "select_mode" => Ok(Step::SelectMode),
            "select_effect" => Ok(Step::SelectEffect),
            "capture" => Ok(Step::Capture),
            "preview" => Ok(Step::Preview),
            "payment" => Ok(Step::Payment),
            "download" => Ok(Step::Download),
            _ => Err(format!("Unknown step: {}", s)),
        }
    }
}
