pub mod commands;
pub mod db;
pub mod models;
pub mod services;

use services::Storage;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting AI Photobooth application");

    // Initialize storage
    let storage = Storage::new().expect("Failed to initialize storage");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(storage)
        .invoke_handler(tauri::generate_handler![
            commands::get_modes,
            commands::get_mode,
            commands::get_effects,
            commands::create_session,
            commands::get_session,
            commands::save_original_photo,
            commands::save_generated_photo,
            commands::create_order,
            commands::get_order,
            commands::update_order_status,
            commands::generate_photo,
            commands::create_payment,
            commands::query_payment,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
