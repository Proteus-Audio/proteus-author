use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use tauri::{AppHandle, Manager, Runtime};

use crate::file::is_project_file_path;

const MAX_RECENT_FILES: usize = 10;
const RECENT_FILES_NAME: &str = "recent-files.json";

pub fn load<R: Runtime>(app: &AppHandle<R>) -> Vec<PathBuf> {
    match load_from_disk(app) {
        Ok(files) => files
            .into_iter()
            .filter(|path| path.is_file() && is_project_file_path(path))
            .collect(),
        Err(err) => {
            println!("Failed to load recent files: {err}");
            Vec::new()
        }
    }
}

pub fn path_at<R: Runtime>(app: &AppHandle<R>, index: usize) -> Option<PathBuf> {
    load(app).into_iter().nth(index)
}

pub fn record<R: Runtime>(app: &AppHandle<R>, path: &Path) {
    if !path.is_file() || !is_project_file_path(path) {
        return;
    }

    let path = path.to_path_buf();
    let mut files = load(app);
    files.retain(|recent| recent != &path);
    files.insert(0, path);
    files.truncate(MAX_RECENT_FILES);

    if let Err(err) = save(app, &files) {
        println!("Failed to save recent files: {err}");
        return;
    }

    if let Err(err) = crate::menu::refresh_recent_files_menu(app) {
        println!("Failed to refresh recent files menu: {err}");
    }
}

pub fn refresh_menu<R: Runtime>(app: &AppHandle<R>) {
    if let Err(err) = crate::menu::refresh_recent_files_menu(app) {
        println!("Failed to refresh recent files menu: {err}");
    }
}

fn load_from_disk<R: Runtime>(app: &AppHandle<R>) -> Result<Vec<PathBuf>, String> {
    let path = storage_path(app)?;

    match fs::read(&path) {
        Ok(contents) => serde_json::from_slice(&contents)
            .map_err(|err| format!("could not parse {}: {err}", path.display())),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(Vec::new()),
        Err(err) => Err(format!("could not read {}: {err}", path.display())),
    }
}

fn save<R: Runtime>(app: &AppHandle<R>, files: &[PathBuf]) -> Result<(), String> {
    let path = storage_path(app)?;
    let directory = path
        .parent()
        .ok_or_else(|| "recent-files storage path has no parent directory".to_string())?;

    fs::create_dir_all(directory)
        .map_err(|err| format!("could not create {}: {err}", directory.display()))?;

    let contents = serde_json::to_vec(files)
        .map_err(|err| format!("could not serialize recent files: {err}"))?;
    fs::write(&path, contents).map_err(|err| format!("could not write {}: {err}", path.display()))
}

fn storage_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|directory| directory.join(RECENT_FILES_NAME))
        .map_err(|err| format!("could not determine the app data directory: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recent_files_are_stored_as_json_paths() {
        let files = vec![
            PathBuf::from("/projects/first.protproject"),
            PathBuf::from("/projects/second.protproject"),
        ];
        let serialized = serde_json::to_vec(&files).expect("paths should serialize");

        let restored: Vec<PathBuf> =
            serde_json::from_slice(&serialized).expect("paths should deserialize");

        assert_eq!(restored, files);
    }
}
