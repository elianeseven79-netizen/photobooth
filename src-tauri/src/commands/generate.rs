use tauri::State;
use crate::models::{PhotoSession, SessionStatus};
use crate::services::{ModeService, MiniMaxService, SessionService, Storage, WeChatService, StyleService};

#[tauri::command]
pub async fn generate_photo(
    storage: State<'_, Storage>,
    session_id: String,
    photo_base64: String,
    style_id: Option<String>,
) -> Result<PhotoSession, String> {
    eprintln!("[Generate] START session_id={}", session_id);

    // Get session info, effect prompt
    let (effect_prompt, final_prompt): (String, String) = {
        eprintln!("[Generate] Getting connection...");
        let conn = storage.get_connection()?;
        eprintln!("[Generate] Got connection, getting session...");
        let session_service = SessionService::new(&conn);
        eprintln!("[Generate] Calling get_session...");
        let session_result = session_service.get_session(&session_id);
        eprintln!("[Generate] get_session returned: {:?}", session_result.as_ref().map(|s| s.is_some()));
        let session = session_result
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Session not found".to_string())?;
        eprintln!("[Generate] Got session, status={:?}", session.status);

        // Check if already processing
        if session.status == SessionStatus::Processing {
            eprintln!("[Generate] Already processing, returning");
            return Ok(session);
        }

        eprintln!("[Generate] Getting mode...");
        let mode_service = ModeService::new(&conn);
        let mode = mode_service.get_mode_by_id(&session.mode_id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Mode not found".to_string())?;

        eprintln!("[Generate] Got mode, getting effect...");
        let effect = mode.effects
            .into_iter()
            .find(|e| e.id == session.effect_id)
            .ok_or_else(|| "Effect not found".to_string())?;

        let effect_prompt = effect.prompt.clone();

        let final_prompt = if let Some(ref sid) = style_id {
            let style_service = StyleService::new(&conn);
            style_service.generate_prompt(sid, &effect_prompt)?
        } else {
            effect_prompt.clone()
        };

        eprintln!("[Generate] Updating status to Processing...");
        session_service.update_session_status(&session_id, SessionStatus::Processing)
            .map_err(|e| e.to_string())?;

        eprintln!("[Generate] Done getting prompt, releasing connection");
        (effect_prompt, final_prompt)
    };

    eprintln!("[Generate] Calling MiniMax API...");
    let minimax = MiniMaxService::new().map_err(|e| e.to_string())?;

    let generated_photo = match tokio::time::timeout(
        std::time::Duration::from_secs(30),
        minimax.generate_image(&photo_base64, &final_prompt)
    ).await {
        Ok(result) => result.map_err(|e| format!("AI generation failed: {}", e))?,
        Err(_) => {
            eprintln!("[Generate] MiniMax timed out, using mock");
            minimax.generate_placeholder_image()
        }
    };

    eprintln!("[Generate] Saving results...");
    let final_session = {
        let conn = storage.get_connection()?;
        let session_service = SessionService::new(&conn);
        session_service.save_original_photo(&session_id, &photo_base64)
            .map_err(|e| e.to_string())?;
        session_service.save_generated_photo(&session_id, &generated_photo)
            .map_err(|e| e.to_string())?;

        if let Some(ref sid) = style_id {
            session_service.update_session_style(&session_id, sid)
                .map_err(|e| e.to_string())?;
        }

        session_service.update_session_status(&session_id, SessionStatus::Previewing)
            .map_err(|e| e.to_string())?;

        session_service.get_session(&session_id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Session not found".to_string())?
    };

    eprintln!("[Generate] DONE");
    Ok(final_session)
}

#[tauri::command]
pub async fn create_payment(
    storage: State<'_, Storage>,
    session_id: String,
    order_type: String,
    amount: i32,
) -> Result<(String, String), String> {
    let order_type_model = match order_type.as_str() {
        "download" => crate::models::OrderType::Download,
        "print" => crate::models::OrderType::Print,
        _ => return Err("Invalid order type".to_string()),
    };

    let order = {
        let conn = storage.get_connection()?;
        let session_service = SessionService::new(&conn);
        session_service.create_order(&session_id, order_type_model, amount)
            .map_err(|e| e.to_string())?
    };

    let wechat = WeChatService::new().map_err(|e| e.to_string())?;

    let (_prepay_id, qr_code) = wechat.create_order(&order.id, amount, "AI Photo Download")
        .await
        .map_err(|e| e.to_string())?;

    Ok((order.id, qr_code))
}

#[tauri::command]
pub async fn query_payment(
    storage: State<'_, Storage>,
    order_id: String,
) -> Result<String, String> {
    let wechat = WeChatService::new().map_err(|e| e.to_string())?;
    let status = wechat.query_order(&order_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(status)
}
