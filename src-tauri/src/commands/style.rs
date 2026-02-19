use tauri::State;
use crate::models::Style;
use crate::services::{StyleService, Storage};

#[tauri::command]
pub fn get_styles(storage: State<Storage>) -> Result<Vec<Style>, String> {
    let conn = storage.get_connection()?;
    let style_service = StyleService::new(&conn);
    style_service.get_all_styles()
}

#[tauri::command]
pub fn get_style(storage: State<Storage>, style_id: String) -> Result<Option<Style>, String> {
    let conn = storage.get_connection()?;
    let style_service = StyleService::new(&conn);
    style_service.get_style_by_id(&style_id)
}
