// use std::sync::{Mutex, Arc};

// use proteus_audio::player::Player;

// #[derive(Clone)]
// pub struct AuthorPlayer {
//     player: Arc<Mutex<Player>>,
// }

// impl AuthorPlayer {
//     pub fn new(files_)

use std::sync::{Arc, Mutex};
use std::time::Duration;

use proteus_lib::container::play_settings::EffectSettings;
use proteus_lib::diagnostics::reporter::Report;
use proteus_lib::playback::player::Player;
use serde::Deserialize;
use serde::Serialize;
// }
use tauri::Emitter;
use tauri::Manager;
use tauri::State;
use tauri::Window;

use crate::project::ProjectSkeleton;

#[tauri::command]
pub async fn init_player(window: Window) {
    let start_of_process = std::time::Instant::now();
    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let project_state: State<Arc<Mutex<ProjectSkeleton>>> = window.state();
    let project = project_state.lock().unwrap();

    let file_list: Vec<Vec<String>> = project
        .tracks
        .iter()
        .map(|t| {
            t.file_ids
                .iter()
                .map(|id| {
                    project
                        .files
                        .iter()
                        .find(|f| f.id == *id)
                        .unwrap()
                        .path
                        .clone()
                })
                .collect()
        })
        .collect();

    // Remove any empty tracks
    let file_list: Vec<Vec<String>> = file_list
        .iter()
        .filter(|t| t.len() > 0)
        .map(|t| t.clone())
        .collect();

    if file_list.len() == 0 {
        player_state.lock().unwrap().take();
        window.emit("PLAYER_CHANGED", ()).unwrap_or(());
        return;
    }

    let mut new_player = Player::new_from_file_paths_legacy(file_list);
    new_player.set_effects(project.effects.clone());
    let handle = window.app_handle().clone();
    let label = String::from(window.label());
    let reporter = move |Report { time, .. }| {
        let window_clone = handle.get_webview_window(&label).unwrap();
        window_clone.emit("UPDATE_PLAYHEAD", time).unwrap_or(());
    };
    new_player.set_reporting(Arc::new(Mutex::new(reporter)), Duration::from_millis(100));
    // new_player.set_reporting(reporting, reporting_interval)
    new_player.pause();
    new_player.set_start_sink_chunks(1);
    new_player.set_start_buffer_ms(10.0);
    new_player.set_startup_fade_ms(5.0);
    new_player.set_max_sink_chunks(15);

    player_state.lock().unwrap().replace(new_player);
    window.emit("PLAYER_CHANGED", ()).unwrap_or(());

    println!(
        "init_player took {}ms",
        start_of_process.elapsed().as_millis()
    );
}

#[tauri::command]
pub async fn play(window: Window) {
    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let mut player = player_state.lock().unwrap();
    println!(
        "playing ({})",
        if player.is_none() { "nope" } else { "yep" }
    );

    if player.is_none() {
        return;
    }

    let project_state: State<Arc<Mutex<ProjectSkeleton>>> = window.state();
    let effects = project_state.lock().unwrap().effects.clone();
    println!("Setting Effects: {:?}", effects);
    player.as_mut().unwrap().set_effects(effects);
    player.as_mut().unwrap().play();
}

#[tauri::command]
pub async fn pause(window: Window) {
    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let player = player_state.lock().unwrap();

    if player.is_none() {
        return;
    }

    player.as_ref().unwrap().pause();
}

#[tauri::command]
pub async fn stop(window: Window) {
    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let player = player_state.lock().unwrap();

    if player.is_none() {
        return;
    }

    player.as_ref().unwrap().stop();
}

#[tauri::command]
pub async fn seek(position: f64, window: Window) {
    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let mut player = player_state.lock().unwrap();

    if player.is_none() {
        return;
    }

    player.as_mut().unwrap().seek(position);
}

#[tauri::command]
pub async fn shuffle(window: Window) {
    println!("shuffling");
    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let mut player = player_state.lock().unwrap();

    if player.is_none() {
        return;
    }

    player.as_mut().unwrap().shuffle();

    drop(player);

    println!("Calling set_selections");

    set_selections(window);
}

#[tauri::command]
pub async fn get_position(window: Window) -> f64 {
    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let player = player_state.lock().unwrap();

    if player.is_none() {
        return 0.0;
    }

    player.as_ref().unwrap().get_time()
}

#[tauri::command]
pub async fn get_duration(window: Window) -> f64 {
    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let player = player_state.lock().unwrap();

    if player.is_none() {
        return 0.0;
    }

    player.as_ref().unwrap().get_duration()
}

#[tauri::command]
pub fn set_selections(window: Window) -> Vec<String> {
    println!("setting selections");
    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let player = player_state.lock().unwrap();

    println!("player: {:?}", player.as_ref().unwrap().info);

    if player.is_none() {
        return Vec::new();
    }

    let urls = player.as_ref().unwrap().get_ids();

    let project_state: State<Arc<Mutex<ProjectSkeleton>>> = window.state();
    let mut project = project_state.lock().unwrap();

    let mut ids = Vec::new();

    for url in urls {
        let id = project
            .files
            .iter()
            .find(|f| f.path == url)
            .unwrap()
            .id
            .clone();

        ids.push(id);
    }

    println!("ids: {:?}", ids);

    // Apply the selections to the project, keeping selection per track.
    for track in project.tracks.iter_mut() {
        track.selection = None;
    }

    for (track, id) in project.tracks.iter_mut().zip(ids.iter()) {
        if track.file_ids.contains(id) {
            track.selection = Some(id.clone());
        }
    }

    ids
}

#[tauri::command]
pub fn set_volume(volume: f32, window: Window) {
    println!("setting volume: {}", volume);
    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let mut player = player_state.lock().unwrap();

    if player.is_none() {
        return;
    }

    player.as_mut().unwrap().set_volume(volume);
}

#[tauri::command]
pub fn get_volume(window: Window) -> f32 {
    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let player = player_state.lock().unwrap();

    if player.is_none() {
        return 1.0;
    }

    player.as_ref().unwrap().get_volume()
}

#[tauri::command]
pub fn set_effects_chain(
    effects: Vec<EffectSettings>,
    window: Window,
    project_state: State<Arc<Mutex<ProjectSkeleton>>>,
) {
    {
        let mut project = project_state.lock().unwrap();
        project.effects = effects.clone();
    }

    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let mut player = player_state.lock().unwrap();
    if let Some(player) = player.as_mut() {
        println!("Setting Effects: {:?}", effects);
        player.set_effects(effects);
    } else {
        println!("No player found");
    }
}

#[derive(Serialize, Deserialize)]
pub enum PlayerState {
    Playing,
    Paused,
    Stopped,
}

#[tauri::command]
pub fn get_play_state(window: Window) -> PlayerState {
    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let player = player_state.lock().unwrap();

    if player.is_none() {
        return PlayerState::Stopped;
    }

    let player = player.as_ref().unwrap();

    if player.is_playing() {
        return PlayerState::Playing;
    }

    if player.is_paused() {
        return PlayerState::Paused;
    }

    PlayerState::Stopped
}

#[tauri::command]
pub fn get_levels(window: Window) -> Vec<f32> {
    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let player = player_state.lock().unwrap();

    if player.is_none() {
        return vec![0.0, 0.0];
    }

    player.as_ref().unwrap().get_levels()
}

#[tauri::command]
pub fn get_levels_db(window: Window) -> Vec<f32> {
    let player_state: State<Arc<Mutex<Option<Player>>>> = window.state();
    let player = player_state.lock().unwrap();

    if player.is_none() {
        return vec![f32::NEG_INFINITY, f32::NEG_INFINITY];
    }

    player.as_ref().unwrap().get_levels_db()
}
