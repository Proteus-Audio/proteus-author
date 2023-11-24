
use std::sync::{Arc, Mutex, atomic::AtomicBool};
use once_cell::sync::Lazy;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReverbSettings {
    pub decay: f32,
    pub pre_delay: f32,
    pub mix: f32,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompressorSettings {
    pub attack: f32,
    pub knee: f32,
    pub ratio: f32,
    pub release: f32,
    pub threshold: f32,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EffectSettings {
    ReverbSettings(ReverbSettings),
    CompressorSettings(CompressorSettings),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Effect {
    Compressor(CompressorSettings),
    Reverb(ReverbSettings),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EffectSkeleton {
    pub id: u32,
    pub effect: Effect,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackSkeleton {
    pub id: u32,
    pub name: String,
    pub file_ids: Vec<String>,
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
    pub peaks: Option<Vec<Vec<(f32, f32)>>>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettingsTrack {
    pub level: f32,
    pub pan: f32,
    pub ids: Vec<u32>,
    pub name: String,
    pub safe_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettingsEncoder {
    pub play_settings: PlaySettings,
    pub encoder_version: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlaySettings {
    pub effects: Vec<EffectSettings>,
    pub tracks: Vec<SettingsTrack>,
}

pub fn empty_project() -> ProjectSkeleton {
    ProjectSkeleton {
        name: Some("untitled".to_string()),
        location: None,
        tracks: vec![TrackSkeleton {
            id: 1,
            name: "".to_string(),
            file_ids: Vec::new(),
        }],
        effects: Vec::new(),
        files: Vec::new(),
    }
}

pub fn create_project() -> Arc<Mutex<ProjectSkeleton>> {
    Arc::new(Mutex::new(empty_project()))
}

pub static PROJECT: Lazy<Arc<Mutex<ProjectSkeleton>>> = Lazy::new(|| {
    create_project()
});

pub static UNSAVED_CHANGES: Lazy<AtomicBool> = Lazy::new(|| {
    AtomicBool::new(false)
});


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