use tauri::State;
use crate::models::PhotoSession;
use crate::services::{SessionService, Storage};

#[tauri::command]
pub fn create_session(storage: State<Storage>, mode_id: String, effect_id: String) -> Result<PhotoSession, String> {
    let conn = storage.get_connection()?;
    let session_service = SessionService::new(&conn);
    session_service.create_session(&mode_id, &effect_id)
}

#[tauri::command]
pub fn get_session(storage: State<Storage>, session_id: String) -> Result<Option<PhotoSession>, String> {
    let conn = storage.get_connection()?;
    let session_service = SessionService::new(&conn);
    session_service.get_session(&session_id)
}

#[tauri::command]
pub fn save_original_photo(storage: State<Storage>, session_id: String, photo_base64: String) -> Result<PhotoSession, String> {
    let conn = storage.get_connection()?;
    let session_service = SessionService::new(&conn);
    session_service.save_original_photo(&session_id, &photo_base64)?;
    session_service.get_session(&session_id)?
        .ok_or_else(|| "Session not found".to_string())
}

#[tauri::command]
pub fn save_generated_photo(storage: State<Storage>, session_id: String, photo_base64: String) -> Result<(), String> {
    let conn = storage.get_connection()?;
    let session_service = SessionService::new(&conn);
    session_service.save_generated_photo(&session_id, &photo_base64)
}
