use tauri::State;
use crate::models::Effect;
use crate::services::{ModeService, Storage};

#[tauri::command]
pub fn get_effects(storage: State<Storage>, mode_id: String) -> Result<Vec<Effect>, String> {
    let conn = storage.conn.lock().map_err(|e| e.to_string())?;
    let mode_service = ModeService::new(&conn);
    mode_service.get_mode_by_id(&mode_id)?
        .map(|m| m.effects)
        .ok_or_else(|| "Mode not found".to_string())
}
