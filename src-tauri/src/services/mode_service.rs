use crate::models::{Effect, PhotoMode};
use rusqlite::Connection;

pub struct ModeService<'a> {
    conn: &'a Connection,
}

impl<'a> ModeService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_modes(&self) -> Result<Vec<PhotoMode>, String> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, icon FROM photo_modes"
        ).map_err(|e| e.to_string())?;

        let modes: Vec<PhotoMode> = stmt.query_map([], |row| {
            Ok(PhotoMode {
                id: row.get(0).unwrap_or_default(),
                name: row.get(1).unwrap_or_default(),
                description: row.get(2).unwrap_or_default(),
                icon: row.get(3).unwrap_or_default(),
                effects: vec![],
            })
        }).map_err(|e| e.to_string())?.filter_map(|m| m.ok()).collect();

        let mut result = Vec::new();
        for mut mode in modes {
            mode.effects = self.get_effects_by_mode(&mode.id)?;
            result.push(mode);
        }

        Ok(result)
    }

    pub fn get_mode_by_id(&self, id: &str) -> Result<Option<PhotoMode>, String> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, icon FROM photo_modes WHERE id = ?1"
        ).map_err(|e| e.to_string())?;

        let mut rows = stmt.query([id]).map_err(|e| e.to_string())?;
        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let mut mode = PhotoMode {
                id: row.get(0).unwrap_or_default(),
                name: row.get(1).unwrap_or_default(),
                description: row.get(2).unwrap_or_default(),
                icon: row.get(3).unwrap_or_default(),
                effects: vec![],
            };
            mode.effects = self.get_effects_by_mode(&mode.id)?;
            Ok(Some(mode))
        } else {
            Ok(None)
        }
    }

    fn get_effects_by_mode(&self, mode_id: &str) -> Result<Vec<Effect>, String> {
        let mut stmt = self.conn.prepare(
            "SELECT id, mode_id, name, prompt, thumbnail, price_download, price_print
             FROM effects WHERE mode_id = ?1"
        ).map_err(|e| e.to_string())?;

        let effects = stmt.query_map([mode_id], |row| {
            Ok(Effect {
                id: row.get(0).unwrap_or_default(),
                mode_id: row.get(1).unwrap_or_default(),
                name: row.get(2).unwrap_or_default(),
                prompt: row.get(3).unwrap_or_default(),
                thumbnail: row.get(4).unwrap_or_default(),
                price_download: row.get::<_, i32>(5).unwrap_or(300),
                price_print: row.get::<_, i32>(6).unwrap_or(1000),
            })
        }).map_err(|e| e.to_string())?.filter_map(|e| e.ok()).collect();

        Ok(effects)
    }
}
