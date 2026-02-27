use crate::project::{
    read_project_by_label, ProjectSkeleton, WindowProjectState, WindowSavedSnapshotState,
};
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use tauri::State;
use tauri::Window;

pub(super) fn canonical_project_json(project: &ProjectSkeleton) -> String {
    let mut canonical = project.clone();
    canonical.location = None;
    serde_json::to_string(&canonical).unwrap_or_else(|_| "{}".to_string())
}

pub(super) fn get_saved_snapshot_by_label(
    label: &str,
    saved_snapshot_state: &State<WindowSavedSnapshotState>,
) -> Option<String> {
    let map = saved_snapshot_state.0.lock().unwrap();
    map.get(label).cloned()
}

pub(super) fn set_saved_snapshot_by_label(
    label: &str,
    saved_snapshot_state: &State<WindowSavedSnapshotState>,
    snapshot: String,
) {
    let mut map = saved_snapshot_state.0.lock().unwrap();
    map.insert(label.to_string(), snapshot);
}

pub(super) fn ensure_saved_snapshot_baseline(
    label: &str,
    project_state: &State<WindowProjectState>,
    saved_snapshot_state: &State<WindowSavedSnapshotState>,
) {
    if get_saved_snapshot_by_label(label, saved_snapshot_state).is_some() {
        return;
    }

    let project = read_project_by_label(label, project_state);
    set_saved_snapshot_by_label(
        label,
        saved_snapshot_state,
        canonical_project_json(&project),
    );
}

pub(super) fn update_window_title(window: &Window, project: &ProjectSkeleton, is_unsaved: bool) {
    let file_name = project.name.clone().unwrap_or("Untitled".to_string());
    let title = if is_unsaved {
        format!("{}*", file_name)
    } else {
        file_name
    };

    if let Err(err) = window.set_title(&title) {
        println!("Failed to set title: {:?}", err);
    }
}

pub(super) fn split_arguments(string: &str) -> Vec<&str> {
    let re = Regex::new(r#"[^"\s]*("[^"]*")|([^"\s]+)"#).unwrap();
    re.find_iter(string)
        .map(|m| {
            let string = m.as_str();
            let first = string.chars().next().unwrap();
            let last = string.chars().last().unwrap();
            let quote = '"';
            if first == quote && last == quote {
                &string[1..string.len() - 1]
            } else {
                string
            }
        })
        .collect()
}

pub(super) fn attachment_mime_for_path(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase()
        .as_str()
    {
        "wav" => "audio/wav",
        "aif" | "aiff" => "audio/aiff",
        "flac" => "audio/flac",
        "ogg" => "audio/ogg",
        _ => "application/octet-stream",
    }
}

pub(super) fn unique_attachment_name(name: &str, used: &mut HashSet<String>) -> String {
    if !used.contains(name) {
        used.insert(name.to_string());
        return name.to_string();
    }

    let (stem, ext) = match name.rsplit_once('.') {
        Some((stem, ext)) => (stem.to_string(), Some(ext.to_string())),
        None => (name.to_string(), None),
    };

    let mut counter = 1;
    loop {
        let candidate = match &ext {
            Some(ext) => format!("{}-{}.{}", stem, counter, ext),
            None => format!("{}-{}", stem, counter),
        };
        if !used.contains(&candidate) {
            used.insert(candidate.clone());
            return candidate;
        }
        counter += 1;
    }
}
