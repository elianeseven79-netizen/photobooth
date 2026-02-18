use tauri::State;
use crate::models::{Order, OrderStatus, OrderType};
use crate::services::{SessionService, Storage};

#[tauri::command]
pub fn create_order(
    storage: State<Storage>,
    session_id: String,
    order_type: String,
    amount: i32,
) -> Result<Order, String> {
    let order_type = match order_type.as_str() {
        "download" => OrderType::Download,
        "print" => OrderType::Print,
        _ => return Err("Invalid order type".to_string()),
    };

    let conn = storage.conn.lock().map_err(|e| e.to_string())?;
    let session_service = SessionService::new(&conn);
    session_service.create_order(&session_id, order_type, amount)
}

#[tauri::command]
pub fn get_order(storage: State<Storage>, order_id: String) -> Result<Option<Order>, String> {
    let conn = storage.conn.lock().map_err(|e| e.to_string())?;
    let session_service = SessionService::new(&conn);
    session_service.get_order(&order_id)
}

#[tauri::command]
pub fn update_order_status(
    storage: State<Storage>,
    order_id: String,
    status: String,
    wechat_order_id: Option<String>,
) -> Result<(), String> {
    let status = match status.as_str() {
        "pending" => OrderStatus::Pending,
        "paid" => OrderStatus::Paid,
        "cancelled" => OrderStatus::Cancelled,
        "refunded" => OrderStatus::Refunded,
        _ => return Err("Invalid order status".to_string()),
    };

    let conn = storage.conn.lock().map_err(|e| e.to_string())?;
    let session_service = SessionService::new(&conn);
    session_service.update_order_status(&order_id, status, wechat_order_id)
}
