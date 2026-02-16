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

use proteus_lib::container::prot::PathsTrack;
use proteus_lib::container::prot::Prot;
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

use crate::project::*;

fn build_paths_tracks(project: &ProjectSkeleton) -> Vec<PathsTrack> {
    project
        .tracks
        .iter()
        .filter_map(|track| {
            let mut file_paths: Vec<String> = track
                .file_ids
                .iter()
                .filter_map(|id| {
                    project
                        .files
                        .iter()
                        .find(|f| f.id == *id)
                        .map(|f| f.path.clone())
                })
                .collect();

            if file_paths.is_empty() {
                return None;
            }

            // Keep the selected file first so the active take remains stable at init time.
            if let Some(selection_id) = &track.selection {
                if let Some(selected_path) = project
                    .files
                    .iter()
                    .find(|f| f.id == *selection_id)
                    .map(|f| f.path.clone())
                {
                    if let Some(index) = file_paths.iter().position(|path| path == &selected_path) {
                        if index > 0 {
                            file_paths.swap(0, index);
                        }
                    }
                }
            }

            Some(PathsTrack {
                file_paths,
                level: 1.0,
                pan: 0.0,
                selections_count: 1,
                shuffle_points: track.shuffle_points.clone(),
            })
        })
        .collect()
}

#[tauri::command]
pub async fn init_player(window: Window) {
    let start_of_process = std::time::Instant::now();
    let player_state: State<WindowPlayerState> = window.state();
    let project_state: State<WindowProjectState> = window.state();
    let project = read_project(&window, &project_state);

    let tracks_for_player = build_paths_tracks(&project);

    if tracks_for_player.is_empty() {
        with_player_mut(&window, &player_state, |player| {
            player.take();
        });
        window.emit("PLAYER_CHANGED", ()).unwrap_or(());
        return;
    }

    let mut new_player = Player::new_from_file_paths(tracks_for_player);
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
    // new_player.set_seek_fade_in_ms(200.0);
    // new_player.set_seek_fade_out_ms(200.0);

    with_player_mut(&window, &player_state, |player| {
        player.replace(new_player);
    });
    window.emit("PLAYER_CHANGED", ()).unwrap_or(());

    println!(
        "init_player took {}ms",
        start_of_process.elapsed().as_millis()
    );
}

#[tauri::command]
pub fn get_possible_combinations(window: Window) -> Option<String> {
    let project_state: State<WindowProjectState> = window.state();
    let project = read_project(&window, &project_state);
    let tracks_for_player = build_paths_tracks(&project);

    if tracks_for_player.is_empty() {
        return Some(String::from("0"));
    }

    let prot = Prot::new_from_file_paths(tracks_for_player);
    prot.count_possible_combinations().map(|count| count.to_string())
}

#[tauri::command]
pub async fn add_shuffle_point(track_id: u32, seconds: f64, window: Window) -> Vec<String> {
    let timestamp = format_shuffle_point_timestamp(seconds);

    {
        let project_state: State<WindowProjectState> = window.state();
        let mut updated = false;
        with_project_mut(&window, &project_state, |project| {
            let Some(track) = project.tracks.iter_mut().find(|track| track.id == track_id) else {
                return;
            };

            if !track.shuffle_points.contains(&timestamp) {
                track.shuffle_points.push(timestamp.clone());
                track.shuffle_points.sort_by(|a, b| {
                    parse_shuffle_point_seconds(a).total_cmp(&parse_shuffle_point_seconds(b))
                });
                track.shuffle_points.dedup();
            }
            updated = true;
        });
        if !updated {
            return Vec::new();
        }
    }

    let (resume_playback, current_time) = {
        let player_state: State<WindowPlayerState> = window.state();
        with_player(&window, &player_state, |player| {
            if let Some(player) = player.as_ref() {
                (player.is_playing(), player.get_time())
            } else {
                (false, 0.0)
            }
        })
    };

    init_player(window.clone()).await;

    if current_time > 0.0 {
        seek(current_time, window.clone()).await;
    }
    if resume_playback {
        play(window.clone()).await;
    }

    let project_state: State<WindowProjectState> = window.state();
    read_project(&window, &project_state)
        .tracks
        .iter()
        .find(|track| track.id == track_id)
        .map(|track| track.shuffle_points.clone())
        .unwrap_or_default()
}

#[tauri::command]
pub async fn remove_shuffle_point(
    track_id: u32,
    seconds: f64,
    tolerance_seconds: f64,
    window: Window,
) -> Vec<String> {
    let target = seconds.max(0.0);
    let tolerance = tolerance_seconds.max(0.0);

    {
        let project_state: State<WindowProjectState> = window.state();
        let mut updated = false;
        with_project_mut(&window, &project_state, |project| {
            let Some(track) = project.tracks.iter_mut().find(|track| track.id == track_id) else {
                return;
            };

            let mut best_index: Option<usize> = None;
            let mut best_distance = f64::INFINITY;

            for (index, point) in track.shuffle_points.iter().enumerate() {
                let distance = (parse_shuffle_point_seconds(point) - target).abs();
                if distance <= tolerance && distance < best_distance {
                    best_distance = distance;
                    best_index = Some(index);
                }
            }

            if let Some(index) = best_index {
                track.shuffle_points.remove(index);
            }
            updated = true;
        });
        if !updated {
            return Vec::new();
        }
    }

    let (resume_playback, current_time) = {
        let player_state: State<WindowPlayerState> = window.state();
        with_player(&window, &player_state, |player| {
            if let Some(player) = player.as_ref() {
                (player.is_playing(), player.get_time())
            } else {
                (false, 0.0)
            }
        })
    };

    init_player(window.clone()).await;

    if current_time > 0.0 {
        seek(current_time, window.clone()).await;
    }
    if resume_playback {
        play(window.clone()).await;
    }

    let project_state: State<WindowProjectState> = window.state();
    read_project(&window, &project_state)
        .tracks
        .iter()
        .find(|track| track.id == track_id)
        .map(|track| track.shuffle_points.clone())
        .unwrap_or_default()
}

fn format_shuffle_point_timestamp(seconds: f64) -> String {
    let normalized = if seconds.is_finite() {
        seconds.max(0.0)
    } else {
        0.0
    };
    format!("{normalized:.3}")
}

fn parse_shuffle_point_seconds(value: &str) -> f64 {
    let parts: Vec<&str> = value.trim().split(':').collect();
    if parts.is_empty() || parts.len() > 3 {
        return f64::INFINITY;
    }

    let seconds_component = parts
        .last()
        .and_then(|part| part.parse::<f64>().ok())
        .unwrap_or(f64::INFINITY);
    let minutes = if parts.len() >= 2 {
        parts[parts.len() - 2].parse::<f64>().unwrap_or(0.0)
    } else {
        0.0
    };
    let hours = if parts.len() == 3 {
        parts[0].parse::<f64>().unwrap_or(0.0)
    } else {
        0.0
    };

    (hours * 3600.0) + (minutes * 60.0) + seconds_component
}

#[tauri::command]
pub async fn play(window: Window) {
    let player_state: State<WindowPlayerState> = window.state();
    let project_state: State<WindowProjectState> = window.state();
    let effects = read_project(&window, &project_state).effects;
    with_player_mut(&window, &player_state, |player| {
        println!("playing ({})", if player.is_none() { "nope" } else { "yep" });
        if let Some(player) = player.as_mut() {
            println!("Setting Effects: {:?}", effects);
            player.set_effects(effects);
            player.play();
        }
    });
}

#[tauri::command]
pub async fn pause(window: Window) {
    let player_state: State<WindowPlayerState> = window.state();
    with_player(&window, &player_state, |player| {
        if let Some(player) = player.as_ref() {
            player.pause();
        }
    });
}

#[tauri::command]
pub async fn stop(window: Window) {
    let player_state: State<WindowPlayerState> = window.state();
    with_player(&window, &player_state, |player| {
        if let Some(player) = player.as_ref() {
            player.stop();
        }
    });
}

#[tauri::command]
pub async fn seek(position: f64, window: Window) {
    let player_state: State<WindowPlayerState> = window.state();
    with_player_mut(&window, &player_state, |player| {
        if let Some(player) = player.as_mut() {
            player.seek(position);
        }
    });
}

#[tauri::command]
pub async fn shuffle(window: Window) {
    println!("shuffling");
    let player_state: State<WindowPlayerState> = window.state();
    let mut shuffled = false;
    with_player_mut(&window, &player_state, |player| {
        if let Some(player) = player.as_mut() {
            player.shuffle();
            shuffled = true;
        }
    });
    if !shuffled {
        return;
    }

    println!("Calling set_selections");

    set_selections(window);
}

#[tauri::command]
pub async fn get_position(window: Window) -> f64 {
    let player_state: State<WindowPlayerState> = window.state();
    with_player(&window, &player_state, |player| {
        player.as_ref().map(|player| player.get_time()).unwrap_or(0.0)
    })
}

#[tauri::command]
pub async fn get_duration(window: Window) -> f64 {
    let player_state: State<WindowPlayerState> = window.state();
    with_player(&window, &player_state, |player| {
        player
            .as_ref()
            .map(|player| player.get_duration())
            .unwrap_or(0.0)
    })
}

#[tauri::command]
pub fn set_selections(window: Window) -> Vec<String> {
    println!("setting selections");
    let player_state: State<WindowPlayerState> = window.state();
    let urls = with_player(&window, &player_state, |player| {
        let Some(player) = player.as_ref() else {
            return None;
        };
        println!("player: {:?}", player.info);
        Some(player.get_ids())
    });
    let Some(urls) = urls else {
        return Vec::new();
    };

    let project_state: State<WindowProjectState> = window.state();
    let project = read_project(&window, &project_state);

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

    with_project_mut(&window, &project_state, |project| {
        for track in project.tracks.iter_mut() {
            track.selection = None;
        }
        for (track, id) in project.tracks.iter_mut().zip(ids.iter()) {
            if track.file_ids.contains(id) {
                track.selection = Some(id.clone());
            }
        }
    });

    ids
}

#[tauri::command]
pub fn set_volume(volume: f32, window: Window) {
    println!("setting volume: {}", volume);
    let player_state: State<WindowPlayerState> = window.state();
    with_player_mut(&window, &player_state, |player| {
        if let Some(player) = player.as_mut() {
            player.set_volume(volume);
        }
    });
}

#[tauri::command]
pub fn get_volume(window: Window) -> f32 {
    let player_state: State<WindowPlayerState> = window.state();
    with_player(&window, &player_state, |player| {
        player.as_ref().map(|player| player.get_volume()).unwrap_or(1.0)
    })
}

#[tauri::command]
pub fn set_effects_chain(
    effects: Vec<EffectSettings>,
    window: Window,
    project_state: State<WindowProjectState>,
) {
    with_project_mut(&window, &project_state, |project| {
        project.effects = effects.clone();
    });

    let player_state: State<WindowPlayerState> = window.state();
    with_player_mut(&window, &player_state, |player| {
        if let Some(player) = player.as_mut() {
            println!("Setting Effects: {:?}", effects);
            player.set_effects(effects);
        } else {
            println!("No player found");
        }
    });
}

#[derive(Serialize, Deserialize)]
pub enum PlayerState {
    Playing,
    Paused,
    Stopped,
}

#[tauri::command]
pub fn get_play_state(window: Window) -> PlayerState {
    let player_state: State<WindowPlayerState> = window.state();
    with_player(&window, &player_state, |player| {
        let Some(player) = player.as_ref() else {
            return PlayerState::Stopped;
        };
        if player.is_playing() {
            return PlayerState::Playing;
        }
        if player.is_paused() {
            return PlayerState::Paused;
        }
        PlayerState::Stopped
    })
}

#[tauri::command]
pub fn get_levels(window: Window) -> Vec<f32> {
    let player_state: State<WindowPlayerState> = window.state();
    with_player(&window, &player_state, |player| {
        player
            .as_ref()
            .map(|player| player.get_levels())
            .unwrap_or_else(|| vec![0.0, 0.0])
    })
}

#[tauri::command]
pub fn get_levels_db(window: Window) -> Vec<f32> {
    let player_state: State<WindowPlayerState> = window.state();
    with_player(&window, &player_state, |player| {
        player
            .as_ref()
            .map(|player| player.get_levels_db())
            .unwrap_or_else(|| vec![f32::NEG_INFINITY, f32::NEG_INFINITY])
    })
}
