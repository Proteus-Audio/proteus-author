use proteus_lib::container::play_settings::EffectSettings;
use proteus_lib::playback::player::Player;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri::State;
use tauri::Window;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackSkeleton {
    pub id: u32,
    pub name: String,
    pub selection: Option<String>,
    pub file_ids: Vec<String>,
    #[serde(default)]
    pub shuffle_points: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileSkeleton {
    pub id: u32,
    pub path: String,
    pub name: String,
    pub extension: Option<String>,
    pub peaks: Option<Vec<Vec<(f32, f32)>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub id: String,
    pub path: String,
    pub name: String,
    pub extension: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfoSkeleton {
    pub id: String,
    pub path: String,
    pub name: String,
    pub extension: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectSkeleton {
    pub location: Option<String>,
    pub name: Option<String>,
    pub tracks: Vec<TrackSkeleton>,
    pub effects: Vec<EffectSettings>,
    pub files: Vec<FileInfo>,
}

pub fn empty_project() -> ProjectSkeleton {
    ProjectSkeleton {
        name: Some("untitled".to_string()),
        location: None,
        tracks: vec![TrackSkeleton {
            id: 1,
            name: "".to_string(),
            selection: None,
            file_ids: Vec::new(),
            shuffle_points: Vec::new(),
        }],
        effects: Vec::new(),
        files: Vec::new(),
    }
}

pub struct WindowProjectState(pub Arc<Mutex<HashMap<String, ProjectSkeleton>>>);
pub struct WindowPlayerState(pub Arc<Mutex<HashMap<String, Option<Player>>>>);
pub struct WindowUnsavedState(pub Arc<Mutex<HashMap<String, bool>>>);
pub struct WindowSavedSnapshotState(pub Arc<Mutex<HashMap<String, String>>>);

pub fn create_project_state() -> WindowProjectState {
    WindowProjectState(Arc::new(Mutex::new(HashMap::new())))
}

pub fn create_player_state() -> WindowPlayerState {
    WindowPlayerState(Arc::new(Mutex::new(HashMap::new())))
}

pub fn create_unsaved_state() -> WindowUnsavedState {
    WindowUnsavedState(Arc::new(Mutex::new(HashMap::new())))
}

pub fn create_saved_snapshot_state() -> WindowSavedSnapshotState {
    WindowSavedSnapshotState(Arc::new(Mutex::new(HashMap::new())))
}

fn window_key(window: &Window) -> String {
    window.label().to_string()
}

pub fn read_project_by_label(
    label: &str,
    project_state: &State<WindowProjectState>,
) -> ProjectSkeleton {
    let mut map = project_state.0.lock().unwrap();
    map.entry(label.to_string())
        .or_insert_with(empty_project)
        .clone()
}

pub fn read_project(window: &Window, project_state: &State<WindowProjectState>) -> ProjectSkeleton {
    read_project_by_label(&window_key(window), project_state)
}

pub fn with_project_mut<R, F>(window: &Window, project_state: &State<WindowProjectState>, f: F) -> R
where
    F: FnOnce(&mut ProjectSkeleton) -> R,
{
    with_project_mut_by_label(&window_key(window), project_state, f)
}

pub fn with_project_mut_by_label<R, F>(
    label: &str,
    project_state: &State<WindowProjectState>,
    f: F,
) -> R
where
    F: FnOnce(&mut ProjectSkeleton) -> R,
{
    let mut map = project_state.0.lock().unwrap();
    let project = map.entry(label.to_string()).or_insert_with(empty_project);
    f(project)
}

pub fn set_project(
    window: &Window,
    project_state: &State<WindowProjectState>,
    project: ProjectSkeleton,
) {
    set_project_by_label(&window_key(window), project_state, project);
}

pub fn set_project_by_label(
    label: &str,
    project_state: &State<WindowProjectState>,
    project: ProjectSkeleton,
) {
    let mut map = project_state.0.lock().unwrap();
    map.insert(label.to_string(), project);
}

pub fn with_player_mut<R, F>(window: &Window, player_state: &State<WindowPlayerState>, f: F) -> R
where
    F: FnOnce(&mut Option<Player>) -> R,
{
    with_player_mut_by_label(&window_key(window), player_state, f)
}

pub fn with_player_mut_by_label<R, F>(
    label: &str,
    player_state: &State<WindowPlayerState>,
    f: F,
) -> R
where
    F: FnOnce(&mut Option<Player>) -> R,
{
    let mut map = player_state.0.lock().unwrap();
    let player = map.entry(label.to_string()).or_insert(None);
    f(player)
}

pub fn with_player<R, F>(window: &Window, player_state: &State<WindowPlayerState>, f: F) -> R
where
    F: FnOnce(&Option<Player>) -> R,
{
    with_player_by_label(&window_key(window), player_state, f)
}

pub fn with_player_by_label<R, F>(label: &str, player_state: &State<WindowPlayerState>, f: F) -> R
where
    F: FnOnce(&Option<Player>) -> R,
{
    let mut map = player_state.0.lock().unwrap();
    let player = map.entry(label.to_string()).or_insert(None);
    f(player)
}

pub fn set_unsaved(window: &Window, unsaved_state: &State<WindowUnsavedState>, unsaved: bool) {
    set_unsaved_by_label(&window_key(window), unsaved_state, unsaved);
}

pub fn set_unsaved_by_label(label: &str, unsaved_state: &State<WindowUnsavedState>, unsaved: bool) {
    let mut map = unsaved_state.0.lock().unwrap();
    map.insert(label.to_string(), unsaved);
}

pub fn get_unsaved(window: &Window, unsaved_state: &State<WindowUnsavedState>) -> bool {
    get_unsaved_by_label(&window_key(window), unsaved_state)
}

pub fn get_unsaved_by_label(label: &str, unsaved_state: &State<WindowUnsavedState>) -> bool {
    let map = unsaved_state.0.lock().unwrap();
    *map.get(label).unwrap_or(&false)
}

pub fn clear_window_state_by_label(
    label: &str,
    project_state: &State<WindowProjectState>,
    player_state: &State<WindowPlayerState>,
    unsaved_state: &State<WindowUnsavedState>,
    saved_snapshot_state: &State<WindowSavedSnapshotState>,
) {
    {
        let mut map = project_state.0.lock().unwrap();
        map.remove(label);
    }

    {
        let mut map = player_state.0.lock().unwrap();
        if let Some(Some(player)) = map.remove(label) {
            player.stop();
        }
    }

    {
        let mut map = unsaved_state.0.lock().unwrap();
        map.remove(label);
    }

    {
        let mut map = saved_snapshot_state.0.lock().unwrap();
        map.remove(label);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectStatus {
    pub project: ProjectSkeleton,
    pub saved: bool,
}

#[tauri::command]
pub fn check_status(window: Window, project_state: State<WindowProjectState>) -> ProjectStatus {
    let project = read_project(&window, &project_state);
    ProjectStatus {
        saved: project.location.is_some(),
        project,
    }
}

#[tauri::command]
pub async fn get_project_state(window: Window) -> ProjectSkeleton {
    let project_state: State<WindowProjectState> = window.state();
    read_project(&window, &project_state)
}
