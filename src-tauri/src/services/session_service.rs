use crate::models::{Order, OrderStatus, OrderType, PhotoSession, SessionStatus, Step};
use chrono::Utc;
use rusqlite::Connection;
use uuid::Uuid;

pub struct SessionService<'a> {
    conn: &'a Connection,
}

impl<'a> SessionService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create_session(&self, mode_id: &str, effect_id: &str) -> Result<PhotoSession, String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();

        self.conn.execute(
            "INSERT INTO photo_sessions (id, mode_id, effect_id, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![id, mode_id, effect_id, SessionStatus::SelectingMode.to_string(), now, now],
        ).map_err(|e| e.to_string())?;

        let session_id = Uuid::new_v4().to_string();
        let expires_at = now + 3600;

        self.conn.execute(
            "INSERT INTO user_sessions (session_id, current_step, mode_id, effect_id, expires_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![session_id, Step::SelectMode.to_string(), mode_id, effect_id, expires_at],
        ).map_err(|e| e.to_string())?;

        Ok(PhotoSession {
            id,
            mode_id: mode_id.to_string(),
            effect_id: effect_id.to_string(),
            style_id: None,
            original_photo: None,
            generated_photo: None,
            status: SessionStatus::SelectingMode,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_session(&self, id: &str) -> Result<Option<PhotoSession>, String> {
        let mut stmt = self.conn.prepare(
            "SELECT id, mode_id, effect_id, style_id, original_photo, generated_photo, status, created_at, updated_at
             FROM photo_sessions WHERE id = ?1"
        ).map_err(|e| e.to_string())?;

        let mut rows = stmt.query([id]).map_err(|e| e.to_string())?;
        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let status_str: String = row.get(6).unwrap_or_default();
            Ok(Some(PhotoSession {
                id: row.get(0).unwrap_or_default(),
                mode_id: row.get(1).unwrap_or_default(),
                effect_id: row.get(2).unwrap_or_default(),
                style_id: row.get(3).ok(),
                original_photo: row.get(4).ok(),
                generated_photo: row.get(5).ok(),
                status: status_str.parse().unwrap_or(SessionStatus::SelectingMode),
                created_at: row.get::<_, i64>(7).unwrap_or(0),
                updated_at: row.get::<_, i64>(8).unwrap_or(0),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn update_session_status(&self, id: &str, status: SessionStatus) -> Result<(), String> {
        let now = Utc::now().timestamp();
        self.conn.execute(
            "UPDATE photo_sessions SET status = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![status.to_string(), now, id],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn save_original_photo(&self, id: &str, photo_base64: &str) -> Result<(), String> {
        let now = Utc::now().timestamp();
        self.conn.execute(
            "UPDATE photo_sessions SET original_photo = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![photo_base64, now, id],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn save_generated_photo(&self, id: &str, photo_base64: &str) -> Result<(), String> {
        let now = Utc::now().timestamp();
        self.conn.execute(
            "UPDATE photo_sessions SET generated_photo = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![photo_base64, now, id],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn update_session_style(&self, id: &str, style_id: &str) -> Result<(), String> {
        let now = Utc::now().timestamp();
        self.conn.execute(
            "UPDATE photo_sessions SET style_id = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![style_id, now, id],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_orders(&self, session_id: &str) -> Result<Vec<Order>, String> {
        let mut stmt = self.conn.prepare(
            "SELECT id, session_id, order_type, amount, status, wechat_order_id, payment_time, created_at
             FROM orders WHERE session_id = ?1 ORDER BY created_at DESC"
        ).map_err(|e| e.to_string())?;

        let orders = stmt.query_map([session_id], |row| {
            let order_type_str: String = row.get(2).unwrap_or_default();
            let status_str: String = row.get(4).unwrap_or_default();
            Ok(Order {
                id: row.get(0).unwrap_or_default(),
                session_id: row.get(1).unwrap_or_default(),
                order_type: order_type_str.parse().unwrap_or(OrderType::Download),
                amount: row.get::<_, i32>(3).unwrap_or(0),
                status: status_str.parse().unwrap_or(OrderStatus::Pending),
                wechat_order_id: row.get(5).ok(),
                payment_time: row.get(6).ok(),
                created_at: row.get::<_, i64>(7).unwrap_or(0),
            })
        }).map_err(|e| e.to_string())?.filter_map(|o| o.ok()).collect();

        Ok(orders)
    }

    pub fn create_order(&self, session_id: &str, order_type: OrderType, amount: i32) -> Result<Order, String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();

        self.conn.execute(
            "INSERT INTO orders (id, session_id, order_type, amount, status, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![id, session_id, order_type.to_string(), amount, OrderStatus::Pending.to_string(), now],
        ).map_err(|e| e.to_string())?;

        Ok(Order {
            id,
            session_id: session_id.to_string(),
            order_type,
            amount,
            status: OrderStatus::Pending,
            wechat_order_id: None,
            payment_time: None,
            created_at: now,
        })
    }

    pub fn update_order_status(&self, order_id: &str, status: OrderStatus, wechat_order_id: Option<String>) -> Result<(), String> {
        let payment_time = if status == OrderStatus::Paid {
            Some(Utc::now().timestamp())
        } else {
            None
        };

        self.conn.execute(
            "UPDATE orders SET status = ?1, wechat_order_id = ?2, payment_time = ?3 WHERE id = ?4",
            rusqlite::params![status.to_string(), wechat_order_id, payment_time, order_id],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_order(&self, order_id: &str) -> Result<Option<Order>, String> {
        let mut stmt = self.conn.prepare(
            "SELECT id, session_id, order_type, amount, status, wechat_order_id, payment_time, created_at
             FROM orders WHERE id = ?1"
        ).map_err(|e| e.to_string())?;

        let mut rows = stmt.query([order_id]).map_err(|e| e.to_string())?;
        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let order_type_str: String = row.get(2).unwrap_or_default();
            let status_str: String = row.get(4).unwrap_or_default();
            Ok(Some(Order {
                id: row.get(0).unwrap_or_default(),
                session_id: row.get(1).unwrap_or_default(),
                order_type: order_type_str.parse().unwrap_or(OrderType::Download),
                amount: row.get::<_, i32>(3).unwrap_or(0),
                status: status_str.parse().unwrap_or(OrderStatus::Pending),
                wechat_order_id: row.get(5).ok(),
                payment_time: row.get(6).ok(),
                created_at: row.get::<_, i64>(7).unwrap_or(0),
            }))
        } else {
            Ok(None)
        }
    }
}
