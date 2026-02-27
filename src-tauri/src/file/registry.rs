use crate::peaks::get_cached_peaks;
use crate::project::{
    default_track_level, default_track_pan, read_project, with_project_mut, FileInfo,
    FileInfoSkeleton, TrackSkeleton, WindowProjectState,
};
use serde::Serialize;
use std::path::Path;
use tauri::Manager;
use tauri::State;
use tauri::Window;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub fn push_file_id(
    track_id: u32,
    file_id: String,
    window: Window,
    project_state: State<WindowProjectState>,
) {
    with_project_mut(&window, &project_state, |project| {
        let track = project.tracks.iter_mut().find(|t| t.id == track_id);

        match track {
            Some(track) => {
                if !track.file_ids.iter().any(|id| id == &file_id) {
                    track.file_ids.push(file_id);
                }
            }
            None => {
                let track = TrackSkeleton {
                    id: track_id,
                    name: "".to_string(),
                    selection: Some(file_id.clone()),
                    file_ids: vec![file_id],
                    shuffle_points: Vec::new(),
                    level: default_track_level(),
                    pan: default_track_pan(),
                };

                project.tracks.push(track);
            }
        }
    });
}

#[tauri::command]
pub async fn register_file(
    file_path: &str,
    track_id: u32,
    window: Window,
) -> Result<FileInfoSkeleton, String> {
    let project_state: State<WindowProjectState> = window.state();
    let project_clone = read_project(&window, &project_state);

    if let Some(existing_file) = project_clone
        .files
        .iter()
        .find(|file| file.path == file_path)
    {
        push_file_id(
            track_id,
            existing_file.id.clone(),
            window.clone(),
            project_state.clone(),
        );
        return Ok(FileInfoSkeleton {
            id: existing_file.id.clone(),
            path: existing_file.path.clone(),
            name: existing_file.name.clone(),
            extension: existing_file.extension.clone(),
        });
    }

    let path = Path::new(file_path);

    let extention = match path.extension().unwrap().to_str() {
        Some(ext) => Some(ext.to_string()),
        None => None,
    };

    let file = FileInfo {
        id: uuid::Uuid::new_v4().to_string(),
        path: file_path.to_string(),
        name: path.file_name().unwrap().to_str().unwrap().to_string(),
        extension: extention,
    };

    with_project_mut(&window, &project_state, |project| {
        project.files.push(file.clone());
    });

    push_file_id(
        track_id,
        file.id.clone(),
        window.clone(),
        project_state.clone(),
    );

    let peaks_start = std::time::Instant::now();
    let _peaks = get_cached_peaks(&window, &file.id);
    println!("Peaks took {}ms", peaks_start.elapsed().as_millis());

    Ok(FileInfoSkeleton {
        id: file.id.clone(),
        path: file.path.clone(),
        name: file.name.clone(),
        extension: file.extension.clone(),
    })
}

#[tauri::command]
pub fn get_missing_project_files(
    window: Window,
    project_state: State<WindowProjectState>,
) -> Vec<FileInfoSkeleton> {
    let project = read_project(&window, &project_state);

    project
        .files
        .iter()
        .filter(|file| file.path.is_empty() || !Path::new(&file.path).exists())
        .map(|file| FileInfoSkeleton {
            id: file.id.clone(),
            path: file.path.clone(),
            name: file.name.clone(),
            extension: file.extension.clone(),
        })
        .collect()
}

#[tauri::command]
pub async fn locate_project_file(
    file_id: String,
    window: Window,
) -> Result<Option<LocateProjectFileResult>, String> {
    let project_state: State<WindowProjectState> = window.state();
    let project = read_project(&window, &project_state);
    let Some(existing) = project.files.iter().find(|file| file.id == file_id) else {
        return Ok(None);
    };
    let existing_name = existing.name.clone();
    let missing_path = existing.path.clone();

    let (tx, rx) = std::sync::mpsc::channel();
    window
        .dialog()
        .file()
        .add_filter("Audio Files", &["wav", "mp3", "aif", "aiff", "flac", "ogg"])
        .set_title(format!("Locate {}", existing_name))
        .pick_file(move |file_path| {
            let _ = tx.send(file_path);
        });

    let Some(picked) = rx.recv().ok().flatten() else {
        return Ok(None);
    };

    let path_buff = match picked.into_path() {
        Ok(path) => path,
        Err(err) => {
            return Err(format!("Invalid file path: {:?}", err));
        }
    };
    let path = path_buff.to_string_lossy().to_string();
    let name = path_buff
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or(existing_name.as_str())
        .to_string();
    let extension = path_buff
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_string());

    with_project_mut(&window, &project_state, |project| {
        if let Some(file) = project.files.iter_mut().find(|file| file.id == file_id) {
            file.path = path.clone();
            file.name = name.clone();
            file.extension = extension.clone();
        }
    });

    let other_files = locate_missing_files_from_example(missing_path, path.to_string(), window)
        .await
        .unwrap_or_default();

    Ok(Some(LocateProjectFileResult {
        linked_file: FileInfoSkeleton {
            id: file_id,
            path,
            name,
            extension,
        },
        found_files: other_files,
    }))
}

#[derive(Debug, Clone, Serialize)]
pub struct LocateProjectFileResult {
    pub linked_file: FileInfoSkeleton,
    pub found_files: Vec<FileInfoSkeleton>,
}

pub async fn locate_missing_files_from_example(
    missing_path: String,
    found_path: String,
    window: Window,
) -> Result<Vec<FileInfoSkeleton>, String> {
    let missing = Path::new(&missing_path);
    let found = Path::new(&found_path);
    let Some((old_base, new_base)) = infer_path_mapping(missing, found) else {
        return Ok(Vec::new());
    };

    let project_state: State<WindowProjectState> = window.state();
    let project = read_project(&window, &project_state);

    let resolved = project
        .files
        .iter()
        .filter(|file| {
            file.path != missing_path && !file.path.is_empty() && !Path::new(&file.path).exists()
        })
        .filter_map(|file| {
            let relative = Path::new(&file.path).strip_prefix(&old_base).ok()?;
            let candidate = new_base.join(relative);
            if !candidate.is_file() {
                return None;
            }

            let candidate_path = candidate.to_string_lossy().to_string();
            let name = candidate
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or(file.name.as_str())
                .to_string();
            let extension = candidate
                .extension()
                .and_then(|value| value.to_str())
                .map(|value| value.to_string());

            Some(FileInfoSkeleton {
                id: file.id.clone(),
                path: candidate_path,
                name,
                extension,
            })
        })
        .collect();

    Ok(resolved)
}

#[tauri::command]
pub async fn apply_found_files(
    found_files: Vec<FileInfoSkeleton>,
    window: Window,
) -> Result<Vec<FileInfoSkeleton>, String> {
    let project_state: State<WindowProjectState> = window.state();
    let mut applied_files = Vec::new();

    with_project_mut(&window, &project_state, |project| {
        for found_file in found_files {
            if !Path::new(&found_file.path).is_file() {
                continue;
            }

            if let Some(existing) = project
                .files
                .iter_mut()
                .find(|file| file.id == found_file.id)
            {
                existing.path = found_file.path.clone();
                existing.name = found_file.name.clone();
                existing.extension = found_file.extension.clone();
                applied_files.push(found_file);
            }
        }
    });

    Ok(applied_files)
}

fn infer_path_mapping(
    missing: &Path,
    found: &Path,
) -> Option<(std::path::PathBuf, std::path::PathBuf)> {
    let mut best_match: Option<(usize, std::path::PathBuf, std::path::PathBuf)> = None;

    for old_base in missing.ancestors() {
        let Ok(old_suffix) = missing.strip_prefix(old_base) else {
            continue;
        };
        let suffix_len = old_suffix.components().count();
        if suffix_len == 0 {
            continue;
        }

        for new_base in found.ancestors() {
            let Ok(new_suffix) = found.strip_prefix(new_base) else {
                continue;
            };

            if old_suffix != new_suffix {
                continue;
            }

            let should_replace = best_match
                .as_ref()
                .map(|(best_len, _, _)| suffix_len > *best_len)
                .unwrap_or(true);
            if should_replace {
                best_match = Some((suffix_len, old_base.to_path_buf(), new_base.to_path_buf()));
            }
        }
    }

    best_match.map(|(_, old_base, new_base)| (old_base, new_base))
}
