use std::sync::Mutex;

pub struct Storage {
    pub conn: Mutex<rusqlite::Connection>,
    pub data_dir: std::path::PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self, String> {
        let data_dir = dirs::data_local_dir()
            .ok_or("Failed to get data directory")?
            .join("ai-photobooth");

        std::fs::create_dir_all(&data_dir).map_err(|e| format!("Failed to create data dir: {}", e))?;

        let db_path = data_dir.join("photobooth.db");
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        // Initialize tables
        crate::db::create_tables(&conn)
            .map_err(|e| format!("Failed to create tables: {}", e))?;

        tracing::info!("Database initialized at {:?}", db_path);

        Ok(Self {
            conn: Mutex::new(conn),
            data_dir,
        })
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new().expect("Failed to initialize storage")
    }
}
