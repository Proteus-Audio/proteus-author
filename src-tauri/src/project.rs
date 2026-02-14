use once_cell::sync::Lazy;
use proteus_lib::container::play_settings::EffectSettings;
use std::sync::{atomic::AtomicBool, Arc, Mutex};

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

pub fn create_project() -> Arc<Mutex<ProjectSkeleton>> {
    Arc::new(Mutex::new(empty_project()))
}

pub static PROJECT: Lazy<Arc<Mutex<ProjectSkeleton>>> = Lazy::new(|| create_project());

pub static UNSAVED_CHANGES: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectStatus {
    pub project: ProjectSkeleton,
    pub saved: bool,
}

#[tauri::command]
pub fn check_status() -> ProjectStatus {
    let project = PROJECT.lock().unwrap();
    ProjectStatus {
        project: project.clone(),
        saved: project.location.is_some(),
    }
}

#[tauri::command]
pub async fn get_project_state(window: Window) -> ProjectSkeleton {
    let project_state: State<Arc<Mutex<ProjectSkeleton>>> = window.state();
    let project = project_state.lock().unwrap();
    project.clone()
}
