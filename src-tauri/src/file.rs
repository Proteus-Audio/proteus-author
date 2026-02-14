use proteus_lib::container::play_settings::PlaySettingsContainer;
use proteus_lib::container::play_settings::{
    PlaySettingsFile, PlaySettingsV2, PlaySettingsV2File, SettingsTrack,
};
use proteus_lib::dsp::effects::AudioEffect;
use regex::Regex;
use serde::Serialize;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::Manager;
use tauri::State;
use tauri::Window;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_shell;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

use crate::peaks::*;
use crate::project::*;

#[derive(Debug, Clone, Serialize)]
pub struct WaveformSegment {
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub file_name: String,
    pub file_end_seconds: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrackWaveformView {
    pub channels: Vec<Vec<f32>>,
    pub segments: Vec<WaveformSegment>,
}

fn split_arguments(string: &str) -> Vec<&str> {
    let re = Regex::new(r#"[^"\s]*("[^"]*)"|([^"\s]+)"#).unwrap();
    re.find_iter(string)
        .map(|m| {
            let string = m.as_str();
            let first = string.chars().next().unwrap();
            let last = string.chars().last().unwrap();
            let quote = "\"".chars().next().unwrap();
            if first == quote && last == quote {
                &string[1..string.len() - 1]
            } else {
                string
            }
        })
        .collect()
}

fn attachment_mime_for_path(path: &Path) -> &'static str {
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

fn unique_attachment_name(name: &str, used: &mut HashSet<String>) -> String {
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

#[tauri::command]
pub fn push_file_id(
    track_id: u32,
    file_id: String,
    project_state: State<Arc<Mutex<ProjectSkeleton>>>,
) {
    let mut project = project_state.lock().unwrap();

    let track = project.tracks.iter_mut().find(|t| t.id == track_id);

    match track {
        Some(track) => {
            if !track.file_ids.iter().any(|id| id == &file_id) {
                track.file_ids.push(file_id);
            }
        }
        None => {
            // Create new track
            let track = TrackSkeleton {
                id: track_id,
                name: "".to_string(),
                selection: Some(file_id.clone()),
                file_ids: vec![file_id],
                shuffle_points: Vec::new(),
            };

            project.tracks.push(track);
        }
    }
}

#[tauri::command]
pub async fn register_file(
    file_path: &str,
    track_id: u32,
    window: Window,
) -> Result<FileInfoSkeleton, String> {
    let project_state: State<Arc<Mutex<ProjectSkeleton>>> = window.state();

    let project = project_state.lock().unwrap();
    let project_clone = project.clone();
    drop(project);

    // See if file is already registered
    if let Some(existing_file) = project_clone.files.iter().find(|file| file.path == file_path) {
        push_file_id(track_id, existing_file.id.clone(), project_state.clone());
        return Ok(FileInfoSkeleton {
            id: existing_file.id.clone(),
            path: existing_file.path.clone(),
            name: existing_file.name.clone(),
            extension: existing_file.extension.clone(),
        });
    }

    // If file is not registered, register it
    {
        // let peaks = proteus_audio::peaks::get_peaks(file_path, true);

        let path = std::path::Path::new(file_path);

        let extention = match path.extension().unwrap().to_str() {
            Some(ext) => Some(ext.to_string()),
            None => None,
        };

        let file = FileInfo {
            id: uuid::Uuid::new_v4().to_string(),
            path: file_path.to_string(),
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            extension: extention,
            // peaks: Some(peaks),
        };

        let mut project = project_state.lock().unwrap();
        project.files.push(file.clone());
        drop(project);

        push_file_id(track_id, file.id.clone(), project_state.clone());

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
}

#[tauri::command]
pub async fn get_simplified_peaks(
    file_id: String,
    zoom: usize,
    window: Window,
) -> Vec<SimplifiedPeaks> {
    let timer = std::time::Instant::now();
    let peaks = get_cached_peaks_for_full_duration(&window, &file_id);
    let simplified_peaks = simplify_peaks(peaks, zoom);
    println!("Simplifying peaks took {}ms", timer.elapsed().as_millis());

    simplified_peaks
}

#[tauri::command]
pub async fn get_waveform_peaks(
    file_id: String,
    start_seconds: f64,
    end_seconds: f64,
    target_peaks: usize,
    window: Window,
) -> Vec<Vec<f32>> {
    get_cached_peak_amplitudes_in_range(
        &window,
        &file_id,
        start_seconds,
        end_seconds,
        target_peaks,
    )
}

#[tauri::command]
pub async fn get_track_waveform_peaks(
    track_id: u32,
    start_seconds: f64,
    end_seconds: f64,
    target_peaks: usize,
    window: Window,
) -> TrackWaveformView {
    let project_state: State<Arc<Mutex<ProjectSkeleton>>> = window.state();
    let project = project_state.lock().unwrap();
    let Some(track) = project.tracks.iter().find(|track| track.id == track_id) else {
        return TrackWaveformView {
            channels: vec![Vec::new()],
            segments: Vec::new(),
        };
    };
    let file_ids = track.file_ids.clone();
    let shuffle_points = track.shuffle_points.clone();
    let selection = track.selection.clone();
    let files = project.files.clone();
    drop(project);

    if file_ids.is_empty() {
        return TrackWaveformView {
            channels: vec![Vec::new()],
            segments: Vec::new(),
        };
    }

    let clamped_start = start_seconds.max(0.0);
    let clamped_end = end_seconds.max(clamped_start + 0.001);
    let view_duration = (clamped_end - clamped_start).max(0.001);
    let points_target = target_peaks.max(1);

    let mut all_points: Vec<f64> = shuffle_points
        .iter()
        .filter_map(|point| parse_timestamp_seconds(point))
        .collect();
    all_points.sort_by(|a, b| a.total_cmp(b));
    all_points.dedup_by(|a, b| (*a - *b).abs() < f64::EPSILON);

    let points_in_range: Vec<f64> = all_points
        .iter()
        .copied()
        .filter(|point| *point > clamped_start && *point < clamped_end)
        .collect();

    let mut segment_bounds = Vec::with_capacity(points_in_range.len() + 2);
    segment_bounds.push(clamped_start);
    segment_bounds.extend(points_in_range.iter().copied());
    segment_bounds.push(clamped_end);

    let selected_index = selection
        .as_ref()
        .and_then(|selection_id| file_ids.iter().position(|id| id == selection_id))
        .unwrap_or(0);
    let base_segment_index = all_points
        .iter()
        .filter(|point| **point <= clamped_start)
        .count();

    let mut channels_out: Vec<Vec<f32>> = Vec::new();
    let mut segments_out: Vec<WaveformSegment> = Vec::new();
    let mut allocated = 0usize;

    for segment_index in 0..(segment_bounds.len().saturating_sub(1)) {
        let seg_start = segment_bounds[segment_index];
        let seg_end = segment_bounds[segment_index + 1];
        let seg_duration = (seg_end - seg_start).max(0.0001);
        let seg_ratio = seg_duration / view_duration;

        let remaining_segments = (segment_bounds.len() - 1) - segment_index;
        let remaining_budget = points_target.saturating_sub(allocated);
        let proportional = ((points_target as f64) * seg_ratio).round() as usize;
        let mut segment_target = proportional.max(8);
        if remaining_segments == 1 {
            segment_target = remaining_budget.max(1);
        } else {
            let max_for_segment = remaining_budget.saturating_sub(remaining_segments - 1).max(1);
            segment_target = segment_target.min(max_for_segment);
        }
        allocated += segment_target;

        // Keep segment choice stable and deterministic relative to selected file.
        let file_index = (selected_index + base_segment_index + segment_index) % file_ids.len();
        let file_id = &file_ids[file_index];
        let file_name = files
            .iter()
            .find(|file| file.id == *file_id)
            .map(|file| file.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());
        let file_end_seconds = get_cached_peak_duration_seconds(&window, file_id);
        segments_out.push(WaveformSegment {
            start_seconds: seg_start,
            end_seconds: seg_end,
            file_name,
            file_end_seconds,
        });

        let segment_channels = get_cached_peak_amplitudes_in_range(
            &window,
            file_id,
            seg_start,
            seg_end,
            segment_target,
        );

        if channels_out.is_empty() {
            channels_out = segment_channels;
            continue;
        }

        if channels_out.len() < segment_channels.len() {
            channels_out.resize_with(segment_channels.len(), Vec::new);
        }

        for channel_index in 0..channels_out.len() {
            if let Some(channel) = segment_channels.get(channel_index) {
                channels_out[channel_index].extend_from_slice(channel);
            } else {
                let pad = vec![0.0_f32; segment_target];
                channels_out[channel_index].extend_from_slice(&pad);
            }
        }
    }

    if channels_out.is_empty() {
        return TrackWaveformView {
            channels: vec![Vec::new()],
            segments: segments_out,
        };
    }

    let channels = channels_out
        .into_iter()
        .map(|channel| resample_to_target_peaks(channel, points_target))
        .collect();

    TrackWaveformView {
        channels,
        segments: segments_out,
    }
}

fn parse_timestamp_seconds(value: &str) -> Option<f64> {
    let parts: Vec<&str> = value.trim().split(':').collect();
    if parts.is_empty() || parts.len() > 3 {
        return None;
    }

    let seconds_component = parts.last()?.parse::<f64>().ok()?;
    if !seconds_component.is_finite() || seconds_component.is_sign_negative() {
        return None;
    }

    let minutes = if parts.len() >= 2 {
        parts[parts.len() - 2].parse::<f64>().ok()?
    } else {
        0.0
    };
    let hours = if parts.len() == 3 {
        parts[0].parse::<f64>().ok()?
    } else {
        0.0
    };

    let seconds = (hours * 3600.0) + (minutes * 60.0) + seconds_component;
    if seconds.is_finite() && seconds >= 0.0 {
        Some(seconds)
    } else {
        None
    }
}

fn resample_to_target_peaks(values: Vec<f32>, target: usize) -> Vec<f32> {
    if target <= 1 {
        return vec![values.first().copied().unwrap_or(0.0)];
    }
    if values.is_empty() {
        return vec![0.0; target];
    }
    if values.len() == target {
        return values;
    }

    let mut out = Vec::with_capacity(target);
    let scale = (values.len() - 1) as f64 / (target - 1) as f64;
    for index in 0..target {
        let src = (index as f64) * scale;
        let left = src.floor() as usize;
        let right = src.ceil() as usize;
        if left == right {
            out.push(values[left]);
        } else {
            let t = (src - left as f64) as f32;
            let value = values[left] * (1.0 - t) + values[right] * t;
            out.push(value);
        }
    }

    out
}

#[tauri::command]
pub fn get_peaks(file_path: &str) -> Vec<Vec<(f32, f32)>> {
    let peaks_file_path = format!("{}.peaks", file_path);
    if !Path::new(&peaks_file_path).exists() {
        proteus_lib::peaks::write_peaks(file_path, &peaks_file_path)
            .expect("failed to write .peaks file");
    }

    let peaks_data = proteus_lib::peaks::get_peaks(&peaks_file_path, Default::default())
        .expect("failed to read .peaks file");

    peaks_data
        .channels
        .into_iter()
        .map(|channel| channel.into_iter().map(|peak| (peak.max, peak.min)).collect())
        .collect()
}

#[tauri::command]
pub fn project_changes(
    new_project: ProjectSkeleton,
    window: Window,
    project_state: State<Arc<Mutex<ProjectSkeleton>>>,
) -> String {
    println!("new_project: {:?}", new_project);
    let project = project_state.lock().unwrap();
    let project_json = serde_json::to_string(&*project).unwrap();
    let new_project_json = serde_json::to_string(&new_project).unwrap();

    let file_name = project.name.clone().unwrap_or("Untitled".to_string());

    if project_json != new_project_json {
        UNSAVED_CHANGES.store(true, std::sync::atomic::Ordering::Relaxed);
        window
            .set_title(&format!("{}*", file_name).as_str())
            .unwrap();
        "Unsaved Changes".to_string()
    } else {
        UNSAVED_CHANGES.store(false, std::sync::atomic::Ordering::Relaxed);
        window.set_title(&format!("{}", file_name)).unwrap();
        "Saved".to_string()
    }
}

#[tauri::command]
pub fn auto_save(new_project: ProjectSkeleton, project_state: State<Arc<Mutex<ProjectSkeleton>>>) {
    println!("Auto Saving");
    let mut project = project_state.lock().unwrap();
    println!("Project: {:?}", project);
    project.tracks = new_project.tracks.clone();
    println!("Project: {:?}", project);
    project.effects = new_project.effects.clone();
    println!("Project: {:?}", project);
    drop(project);
}

#[tauri::command]
pub async fn save_file(window: Window) -> Option<ProjectSkeleton> {
    // auto_save(new_project.clone());
    let project_state: State<Arc<Mutex<ProjectSkeleton>>> = window.state();

    let project_state_clone = project_state.clone();
    if UNSAVED_CHANGES.load(std::sync::atomic::Ordering::Relaxed) == false {
        let project = project_state_clone.lock().unwrap();
        println!("No changes to save");
        if !project.location.is_none() {
            return None;
        }
        drop(project);
    }

    let project_already_saved = project_state.lock().unwrap().location.is_some();
    // let project_already_saved = new_project.location.is_some();
    // drop(project);

    if !project_already_saved {
        return save_file_as(window).await;
    }

    let project = project_state.lock().unwrap();
    let project_json = serde_json::to_string(&*project).unwrap();

    let mut file = File::create(project.location.clone().unwrap()).unwrap();
    file.write_all(project_json.as_bytes()).unwrap();

    let file_name = project.name.clone().unwrap_or("Untitled".to_string());
    UNSAVED_CHANGES.store(false, std::sync::atomic::Ordering::Relaxed);
    window.set_title(&format!("{}", file_name)).unwrap();

    Some(project.clone())
}

#[tauri::command]
pub async fn save_file_as(window: Window) -> Option<ProjectSkeleton> {
    let project_state: State<Arc<Mutex<ProjectSkeleton>>> = window.state();
    let project = project_state.lock().unwrap();
    let file_name = project.name.clone().unwrap_or("untitled".to_string());
    drop(project);

    // let file_name = new_project.name.clone().unwrap_or("untitled".to_string()) + ".protproject";
    // auto_save(new_project)

    let file_path = window
        .dialog()
        .file()
        .add_filter("Proteus Author Project", &["protproject"])
        .set_title("Save Project")
        .set_file_name(file_name.as_str())
        .blocking_save_file();

    if file_path.is_none() {
        println!("No file selected");
        return None;
    }

    let path_buff = match file_path.unwrap().into_path() {
        Ok(path) => path,
        Err(err) => {
            println!("Invalid file path: {:?}", err);
            return None;
        }
    };
    let file_name =
        String::from(path_buff.file_name().unwrap().to_str().unwrap()).replace(".protproject", "");

    let mut project = project_state.lock().unwrap();

    project.name = Some(file_name.clone());
    project.location = Some(path_buff.to_str().unwrap().to_string());

    println!("Project: {:?}", serde_json::to_string(&project.clone()));

    let project_json = serde_json::to_string(&*project).unwrap();

    let mut file = File::create(&path_buff).unwrap();
    file.write_all(project_json.as_bytes()).unwrap();

    UNSAVED_CHANGES.store(false, std::sync::atomic::Ordering::Relaxed);

    window.set_title(&file_name).unwrap();

    Some(project.clone())
}

#[tauri::command]
pub async fn open_file(window: Window) {
    println!("Window: {:?}", window);
    window
        .dialog()
        .file()
        .add_filter("Proteus Project", &["protproject"])
        .pick_file(move |file| {
            if file.is_none() {
                println!("No file selected");
                return;
            }

            let file_path = file.unwrap();
            let path_buff = match file_path.into_path() {
                Ok(path) => path,
                Err(err) => {
                    println!("Invalid file path: {:?}", err);
                    return;
                }
            };
            let project_state: State<Arc<Mutex<ProjectSkeleton>>> = window.state();

            if path_buff.extension().unwrap() != "protproject" {
                println!("File extension is not .protproject");
                ()
            }

            let file_name = path_buff.file_name().unwrap().to_str().unwrap();
            let project_location = path_buff.to_str().unwrap().to_string();

            let file_contents = std::fs::read_to_string(path_buff.clone()).unwrap();
            let project_result: Result<ProjectSkeleton, serde_json::Error> =
                serde_json::from_str(&file_contents);

            let mut project = project_state.lock().unwrap();

            match project_result {
                Ok(new_project) => {
                    project.name = Some(file_name.to_string());
                    project.location = Some(project_location.to_string());
                    project.tracks = new_project.tracks.clone();
                    project.effects = new_project.effects.clone();
                    project.files = new_project.files.clone();
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }

            let file_name = project.name.clone().unwrap();
            window.set_title(&file_name.as_str()).unwrap();
            window
                .emit("FILE_LOADED", project.clone())
                .expect("failed to emit event");

            drop(project);
        });
}

#[tauri::command]
pub async fn load_empty_project(handle: AppHandle) {
    let empty_project = Arc::new(Mutex::new(empty_project()));
    let project = empty_project.lock().unwrap();

    let window = handle
        .get_webview_window(&"main-window-1".to_string())
        .unwrap();
    window
        .set_title(
            &project
                .name
                .clone()
                .unwrap_or("untitled".to_string())
                .as_str(),
        )
        .unwrap();
    window
        .emit("FILE_LOADED", project.clone())
        .expect("failed to emit event");
    drop(project);
}

#[tauri::command]
pub fn export_prot(project_state: State<Arc<Mutex<ProjectSkeleton>>>, window: Window) {
    let project = project_state.lock().unwrap().clone();
    let file_name = project.name.clone().unwrap_or("export".to_string()) + ".prot";
    let save_dialog = window
        .dialog()
        .file()
        .add_filter("Proteus Audio", &["prot"])
        .set_title("Save Project")
        .set_file_name(file_name.as_str());

    let handle = window.app_handle().clone();

    save_dialog.save_file(move |file_path| {
        if file_path.is_none() {
            println!("No file selected");
            handle.emit("EXPORTING", "Cancelled").unwrap();
            ()
        }

        let file_path = match file_path.clone().unwrap().into_path() {
            Ok(path) => path,
            Err(err) => {
                println!("Invalid file path: {:?}", err);
                handle.emit("EXPORTING", "Cancelled").unwrap();
                return;
            }
        };
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        handle
            .emit("EXPORTING", format!("Exporting {}", file_name))
            .unwrap();
        // `new_sidecar()` expects just the filename, NOT the whole path like in JavaScript
        let mut reduced_file_list = Vec::new();

        let mut effects = project.effects.clone();
        let mut ir_attachments: Vec<(String, String, String)> = Vec::new();
        let mut used_attachment_names = HashSet::new();

        for effect in effects.iter_mut() {
            if let AudioEffect::ConvolutionReverb(convolution) = effect {
                let mut path = convolution.settings.impulse_response_path.clone();
                if path.is_none() {
                    if let Some(raw) = convolution.settings.impulse_response.clone() {
                        if !raw.starts_with("attachment:") {
                            let candidate = raw.strip_prefix("file:").unwrap_or(&raw).to_string();
                            if Path::new(&candidate).exists() {
                                path = Some(candidate);
                            }
                        }
                    }
                }

                if let Some(path) = path {
                    let ir_path = Path::new(&path);
                    if !ir_path.exists() {
                        println!("Impulse response not found: {}", path);
                        continue;
                    }

                    let file_name = ir_path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("impulse_response.wav");
                    let attachment_name =
                        unique_attachment_name(file_name, &mut used_attachment_names);
                    let attachment_ref = format!("attachment:{}", attachment_name);

                    convolution.settings.impulse_response = Some(attachment_ref.clone());
                    convolution.settings.impulse_response_attachment = Some(attachment_ref);
                    convolution.settings.impulse_response_path = None;

                    let mime = attachment_mime_for_path(ir_path).to_string();
                    ir_attachments.push((path, attachment_name, mime));
                }
            }
        }

        let mut play_settings = PlaySettingsV2 {
            effects,
            tracks: Vec::new(),
        };

        for track in project.tracks.iter() {
            let mut settings_track = SettingsTrack {
                level: 1.0,
                pan: 0.0,
                ids: Vec::new(),
                name: track.name.clone(),
                safe_name: track.name.clone(),
                selections_count: 1,
                shuffle_points: track.shuffle_points.clone(),
            };

            for file_id in &track.file_ids {
                let file = project
                    .files
                    .iter()
                    .find(|f| f.id == *file_id)
                    .unwrap()
                    .clone();

                // If filepath is already in reduced_file_list, skip
                if !reduced_file_list.contains(&file.path) {
                    reduced_file_list.push(file.path.clone());
                }

                // Get index of filepath in reduced_file_list
                let index = reduced_file_list
                    .iter()
                    .position(|r| r == &file.path)
                    .unwrap();
                settings_track.ids.push((index + 1) as u32);
            }

            play_settings.tracks.push(settings_track);
        }

        let mut input_list = String::new();
        let mut map_list = String::new();
        let mut metadata_list = String::new();

        for (index, file) in reduced_file_list.iter().enumerate() {
            input_list.push_str(&format!("-i \"{}\" ", file));
            map_list.push_str(&format!("-map {} ", index));
            metadata_list.push_str(&format!("-metadata:s:a:{} title=\"{}\" ", index, file));
        }

        let settings_file = PlaySettingsV2File {
            settings: PlaySettingsContainer::Nested {
                play_settings: play_settings,
            },
        };

        let settings_encoder = PlaySettingsFile::V2(settings_file);

        let json_settings = serde_json::to_string(&settings_encoder).unwrap();

        let output_dir = file_path.parent().unwrap().to_str().unwrap().to_string();
        let settings_file_path = format!("{}/play_settings.json", output_dir);
        let mut settings_file = File::create(settings_file_path.clone()).unwrap();
        settings_file.write_all(json_settings.as_bytes()).unwrap();

        print!("Settings Written: {:?}", json_settings);

        // Replace extension .prot with .mka
        // TODO: Replace with regex
        let output_file = file_path
            .to_str()
            .unwrap()
            .to_string()
            .replace(".prot", ".mka");

        let mut attachment_args = String::new();
        let mut attachment_index = 0;

        attachment_args.push_str(&format!(
            "-attach \"{}\" -metadata:s:t:{} mimetype=application/json -metadata:s:t:{} filename=play_settings.json ",
            settings_file_path, attachment_index, attachment_index
        ));
        attachment_index += 1;

        for (path, name, mime) in ir_attachments.iter() {
            attachment_args.push_str(&format!(
                "-attach \"{}\" -metadata:s:t:{} mimetype={} -metadata:s:t:{} filename=\"{}\" ",
                path, attachment_index, mime, attachment_index, name
            ));
            attachment_index += 1;
        }

        let out_command = format!(
            "-y {}{}{}{} {} \"{}\"",
            input_list,
            map_list,
            attachment_args,
            metadata_list,
            format!("-f matroska"),
            output_file
        );

        println!("{}", out_command);
        println!("{:?}", split_arguments(out_command.as_str()));

        let shell = handle.shell();
        let (mut rx, ..) = shell
            .sidecar("ffmpeg")
            .expect("failed to create `ffmpeg` binary command")
            .args(split_arguments(out_command.as_str()))
            .spawn()
            .expect("Failed to spawn sidecar");

        tauri::async_runtime::spawn(async move {
            // read events such as stdout
            while let Some(event) = rx.recv().await {
                match event {
                    CommandEvent::Stdout(line) => {
                        let text = String::from_utf8_lossy(&line);
                        println!("Line: {}", text.trim_end());
                    }
                    CommandEvent::Stderr(line) => {
                        let text = String::from_utf8_lossy(&line);
                        eprintln!("Error: {}", text.trim_end());
                    }
                    CommandEvent::Terminated(exit_status) => {
                        println!("Exit: {:#?}", exit_status);
                    }
                    _ => {}
                }
            }

            // Remove settings file
            std::fs::remove_file(settings_file_path).unwrap();

            // Rename output file if it exists from .mka to .prot
            if std::path::Path::new(&output_file).exists() {
                std::fs::rename(output_file.clone(), output_file.replace(".mka", ".prot")).unwrap();
            }

            handle.emit("EXPORTING", "Export Finished").unwrap();
        });
    });
}
