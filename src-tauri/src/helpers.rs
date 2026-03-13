use tauri::Manager;
use tauri::Window;

use std::fs;

pub fn get_cache_dir(window: &Window) -> Result<String, String> {
    let app_cache = match window.path().app_cache_dir() {
        Ok(path) => path,
        Err(err) => return Err(format!("Failed to get app cache dir: {}", err)),
    };

    let app_cache_dir = match app_cache.to_str() {
        Some(dir) => dir,
        None => return Err("Failed to convert app cache dir to string".to_string()),
    };

    if !(fs::exists(app_cache_dir).unwrap()) {
        fs::create_dir_all(app_cache_dir)
            .map_err(|err| format!("Failed to create cache dir: {}", err))?;
    }

    Ok(app_cache_dir.to_string())
}
