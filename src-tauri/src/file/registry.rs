use crate::peaks::get_cached_peaks;
use crate::project::{
    default_track_level, default_track_pan, read_project, with_project_mut, FileInfo,
    FileInfoSkeleton, TrackSkeleton, WindowProjectState,
};
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

    if let Some(existing_file) = project_clone.files.iter().find(|file| file.path == file_path) {
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
) -> Result<Option<FileInfoSkeleton>, String> {
    let project_state: State<WindowProjectState> = window.state();
    let project = read_project(&window, &project_state);
    let Some(existing) = project.files.iter().find(|file| file.id == file_id) else {
        return Ok(None);
    };
    let existing_name = existing.name.clone();

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

    Ok(Some(FileInfoSkeleton {
        id: file_id,
        path,
        name,
        extension,
    }))
}
