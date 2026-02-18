use tauri::State;
use crate::models::PhotoMode;
use crate::services::{ModeService, Storage};

#[tauri::command]
pub fn get_modes(storage: State<Storage>) -> Result<Vec<PhotoMode>, String> {
    let conn = storage.conn.lock().map_err(|e| e.to_string())?;
    let mode_service = ModeService::new(&conn);
    mode_service.get_all_modes()
}

#[tauri::command]
pub fn get_mode(storage: State<Storage>, mode_id: String) -> Result<Option<PhotoMode>, String> {
    let conn = storage.conn.lock().map_err(|e| e.to_string())?;
    let mode_service = ModeService::new(&conn);
    mode_service.get_mode_by_id(&mode_id)
}
