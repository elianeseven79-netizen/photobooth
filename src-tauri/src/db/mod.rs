// Database schema for AI Photobooth
use rusqlite::{Connection, Result};

pub fn create_tables(conn: &Connection) -> Result<()> {
    // Photo modes table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS photo_modes (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            icon TEXT
        )",
        [],
    )?;

    // Effects table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS effects (
            id TEXT PRIMARY KEY,
            mode_id TEXT NOT NULL,
            name TEXT NOT NULL,
            prompt TEXT NOT NULL,
            thumbnail TEXT,
            price_download INTEGER NOT NULL DEFAULT 300,
            price_print INTEGER NOT NULL DEFAULT 1000,
            FOREIGN KEY (mode_id) REFERENCES photo_modes(id)
        )",
        [],
    )?;

    // Photo sessions table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS photo_sessions (
            id TEXT PRIMARY KEY,
            mode_id TEXT NOT NULL,
            effect_id TEXT NOT NULL,
            style_id TEXT,
            original_photo BLOB,
            generated_photo BLOB,
            status TEXT NOT NULL DEFAULT 'SelectingMode',
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (mode_id) REFERENCES photo_modes(id),
            FOREIGN KEY (effect_id) REFERENCES effects(id),
            FOREIGN KEY (style_id) REFERENCES styles(id)
        )",
        [],
    )?;

    // Orders table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS orders (
            id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL,
            order_type TEXT NOT NULL,
            amount INTEGER NOT NULL,
            status TEXT NOT NULL DEFAULT 'Pending',
            wechat_order_id TEXT,
            payment_time INTEGER,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (session_id) REFERENCES photo_sessions(id)
        )",
        [],
    )?;

    // User sessions table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_sessions (
            session_id TEXT PRIMARY KEY,
            current_step TEXT NOT NULL DEFAULT 'Home',
            mode_id TEXT,
            effect_id TEXT,
            expires_at INTEGER NOT NULL
        )",
        [],
    )?;

    // Styles table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS styles (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            icon TEXT,
            prompt_template TEXT NOT NULL
        )",
        [],
    )?;

    // Insert default modes
    insert_default_modes(conn)?;

    // Insert default styles
    insert_default_styles(conn)?;

    // Migration: Add style_id column if it doesn't exist
    conn.execute(
        "ALTER TABLE photo_sessions ADD COLUMN style_id TEXT",
        [],
    ).ok(); // ok() to ignore error if column already exists

    Ok(())
}

fn insert_default_modes(conn: &Connection) -> Result<()> {
    let modes = vec![
        ("cartoon", "卡通模式", "可爱的卡通风格照片", "cartoon.png"),
        ("movie", "电影海报", "经典电影海报风格", "movie.png"),
        ("anime", "二次元", "日式动漫风格", "anime.png"),
        ("cyberpunk", "赛博朋克", "未来科技风格", "cyberpunk.png"),
        ("traditional", "古装风格", "中国传统服饰风格", "traditional.png"),
        ("age", "年龄变化", "年龄老化或年轻化效果", "age.png"),
    ];

    for (id, name, description, icon) in modes {
        conn.execute(
            "INSERT OR IGNORE INTO photo_modes (id, name, description, icon) VALUES (?1, ?2, ?3, ?4)",
            [id, name, description, icon],
        )?;
    }

    // Insert default effects for each mode
    let effects = vec![
        // Cartoon effects
        ("cartoon-01", "cartoon", "卡通可爱", "cute cartoon style, bright colors, Disney animation", "cartoon-01.jpg", 300, 1000),
        ("cartoon-02", "cartoon", "卡通动漫", "anime cartoon style, Japanese manga", "cartoon-02.jpg", 300, 1000),
        // Movie effects
        ("movie-01", "movie", "动作海报", "action movie poster, dramatic lighting", "movie-01.jpg", 300, 1000),
        ("movie-02", "movie", "爱情海报", "romantic movie poster, soft lighting", "movie-02.jpg", 300, 1000),
        // Anime effects
        ("anime-01", "anime", "少女漫", "shoujo anime style, pink themes", "anime-01.jpg", 300, 1000),
        ("anime-02", "anime", "少年漫", "shounen anime style, action poses", "anime-02.jpg", 300, 1000),
        // Cyberpunk effects
        ("cyberpunk-01", "cyberpunk", "未来都市", "futuristic city, neon lights", "cyberpunk-01.jpg", 300, 1000),
        ("cyberpunk-02", "cyberpunk", "机械战士", "cyborg warrior, metal details", "cyberpunk-02.jpg", 300, 1000),
        // Traditional effects
        ("traditional-01", "traditional", "汉服", "hanfu traditional Chinese clothing", "traditional-01.jpg", 300, 1000),
        ("traditional-02", "traditional", "古风", "ancient Chinese style, traditional hairstyle", "traditional-02.jpg", 300, 1000),
        // Age effects
        ("age-01", "age", "童年", "childhood version, young and cute", "age-01.jpg", 300, 1000),
        ("age-02", "age", "老年", "elderly version, wise and mature", "age-02.jpg", 300, 1000),
    ];

    for (id, mode_id, name, prompt, thumbnail, price_download, price_print) in effects {
        conn.execute(
            "INSERT OR IGNORE INTO effects (id, mode_id, name, prompt, thumbnail, price_download, price_print) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![id, mode_id, name, prompt, thumbnail, price_download, price_print],
        )?;
    }

    Ok(())
}

fn insert_default_styles(conn: &Connection) -> Result<()> {
    let styles = vec![
        ("cartoon", "卡通", "将照片转换为可爱的卡通风格，使用鲜明的色彩和简洁的线条", "🎨", "将照片转换为卡通风格，{original_description}，使用鲜明的色彩和简洁的线条，呈现Disney风格的动画效果"),
        ("movie", "电影海报", "经典电影海报风格，具有戏剧性的光线和构图", "🎬", "将照片转换为电影海报风格，{original_description}，具有戏剧性的光线和电影级构图，展现经典好莱坞电影海报的视觉效果"),
        ("anime", "二次元", "日式动漫风格，使用动漫风格的眼睛和面部特征", "🌸", "将照片转换为动漫/二次元风格，{original_description}，使用动漫风格的眼睛和面部特征，呈现日式漫画的精致画风"),
        ("cyberpunk", "赛博朋克", "未来科技风格，霓虹灯光和数字化效果", "🌃", "将照片转换为赛博朋克风格，{original_description}，带有霓虹灯光、数字化效果和未来科技元素，展现高科技都市氛围"),
        ("watercolor", "水彩画", "艺术水彩画风格，柔和的色彩和流畅的笔触", "🎭", "将照片转换为水彩画风格，{original_description}，使用柔和的色彩和流畅的笔触，呈现艺术水彩画的优雅效果"),
        ("oil-painting", "油画", "古典油画风格，丰富的色彩和纹理", "🖼️", "将照片转换为古典油画风格，{original_description}，使用丰富的色彩和油画纹理，展现欧洲古典绘画的艺术魅力"),
    ];

    for (id, name, description, icon, prompt_template) in styles {
        conn.execute(
            "INSERT OR IGNORE INTO styles (id, name, description, icon, prompt_template) VALUES (?1, ?2, ?3, ?4, ?5)",
            [id, name, description, icon, prompt_template],
        )?;
    }

    Ok(())
}
