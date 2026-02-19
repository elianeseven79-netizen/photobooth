use std::path::PathBuf;

pub struct Storage {
    pub data_dir: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self, String> {
        let data_dir = dirs::data_local_dir()
            .ok_or("Failed to get data directory")?
            .join("ai-photobooth");

        std::fs::create_dir_all(&data_dir).map_err(|e| format!("Failed to create data dir: {}", e))?;

        let db_path = data_dir.join("photobooth.db");

        // Initialize tables with a temporary connection
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        crate::db::create_tables(&conn)
            .map_err(|e| format!("Failed to create tables: {}", e))?;

        tracing::info!("Database initialized at {:?}", db_path);

        Ok(Self { data_dir })
    }

    pub fn get_connection(&self) -> Result<rusqlite::Connection, String> {
        let db_path = self.data_dir.join("photobooth.db");
        rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))
    }
}

impl Clone for Storage {
    fn clone(&self) -> Self {
        Self { data_dir: self.data_dir.clone() }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new().expect("Failed to initialize storage")
    }
}
