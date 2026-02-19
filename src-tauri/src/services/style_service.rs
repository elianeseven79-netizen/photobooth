use crate::models::Style;
use rusqlite::Connection;

pub struct StyleService<'a> {
    conn: &'a Connection,
}

impl<'a> StyleService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_styles(&self) -> Result<Vec<Style>, String> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, icon, prompt_template FROM styles"
        ).map_err(|e| e.to_string())?;

        let styles = stmt.query_map([], |row| {
            Ok(Style {
                id: row.get(0).unwrap_or_default(),
                name: row.get(1).unwrap_or_default(),
                description: row.get(2).unwrap_or_default(),
                icon: row.get(3).unwrap_or_default(),
                prompt_template: row.get(4).unwrap_or_default(),
            })
        }).map_err(|e| e.to_string())?.filter_map(|s| s.ok()).collect();

        Ok(styles)
    }

    pub fn get_style_by_id(&self, id: &str) -> Result<Option<Style>, String> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, icon, prompt_template FROM styles WHERE id = ?1"
        ).map_err(|e| e.to_string())?;

        let mut rows = stmt.query([id]).map_err(|e| e.to_string())?;
        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            Ok(Some(Style {
                id: row.get(0).unwrap_or_default(),
                name: row.get(1).unwrap_or_default(),
                description: row.get(2).unwrap_or_default(),
                icon: row.get(3).unwrap_or_default(),
                prompt_template: row.get(4).unwrap_or_default(),
            }))
        } else {
            Ok(None)
        }
    }

    /// Generate final prompt from template by replacing placeholder
    pub fn generate_prompt(&self, style_id: &str, original_description: &str) -> Result<String, String> {
        let style = self.get_style_by_id(style_id)?
            .ok_or_else(|| "Style not found".to_string())?;

        let prompt = style.prompt_template.replace("{original_description}", original_description);
        Ok(prompt)
    }
}
