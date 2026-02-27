use crate::file::types::{TrackWaveformView, WaveformSegment};
use crate::peaks::{
    get_cached_peak_amplitudes_in_range, get_cached_peak_duration_seconds,
    get_cached_peaks_for_full_duration, simplify_peaks, SimplifiedPeaks,
};
use crate::player_runtime::{player_shuffle_schedule, PlayerActorState};
use crate::project::{read_project, WindowProjectState};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use tauri::Manager;
use tauri::State;
use tauri::Window;

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
    get_cached_peak_amplitudes_in_range(&window, &file_id, start_seconds, end_seconds, target_peaks)
}

#[tauri::command]
pub async fn get_track_waveform_peaks(
    track_id: u32,
    start_seconds: f64,
    end_seconds: f64,
    target_peaks: usize,
    window: Window,
) -> TrackWaveformView {
    let project_state: State<WindowProjectState> = window.state();
    let project = read_project(&window, &project_state);
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
    let playable_track_index = project
        .tracks
        .iter()
        .filter(|candidate| {
            candidate.file_ids.iter().any(|file_id| {
                files
                    .iter()
                    .any(|file| file.id == *file_id && !file.path.is_empty())
            })
        })
        .position(|candidate| candidate.id == track_id);

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
    let total_segments = segment_bounds.len().saturating_sub(1);
    let allowed_file_ids: HashSet<&str> = file_ids.iter().map(|id| id.as_str()).collect();
    let path_to_file_id: HashMap<&str, &str> = files
        .iter()
        .map(|file| (file.path.as_str(), file.id.as_str()))
        .collect();
    let shuffle_schedule = {
        let player_state: State<PlayerActorState> = window.state();
        player_shuffle_schedule(&window, &player_state)
    };

    let resolve_file_from_schedule = |segment_time: f64| -> Option<String> {
        let track_index = playable_track_index?;
        let schedule = shuffle_schedule.as_ref()?;
        if schedule.is_empty() {
            return None;
        }

        let mut current_sources = &schedule[0].1;
        for (at_seconds, sources) in schedule.iter() {
            if *at_seconds <= segment_time {
                current_sources = sources;
            } else {
                break;
            }
        }

        let track = current_sources.get(track_index)?;
        let path = track.first()?;
        let file_id = path_to_file_id.get(path.as_str())?;
        if allowed_file_ids.contains(*file_id) {
            Some((*file_id).to_string())
        } else {
            None
        }
    };

    for segment_index in 0..total_segments {
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
            let max_for_segment = remaining_budget
                .saturating_sub(remaining_segments - 1)
                .max(1);
            segment_target = segment_target.min(max_for_segment);
        }
        allocated += segment_target;

        let segment_lookup_time = (seg_start + 0.000_001).min(seg_end);
        let scheduled_file_id = resolve_file_from_schedule(segment_lookup_time);
        let file_id = if let Some(file_id) = scheduled_file_id {
            file_id
        } else {
            let file_index = (selected_index + base_segment_index + segment_index) % file_ids.len();
            file_ids[file_index].clone()
        };
        let file_name = files
            .iter()
            .find(|file| file.id == file_id)
            .map(|file| file.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());
        let file_end_seconds = get_cached_peak_duration_seconds(&window, &file_id);
        segments_out.push(WaveformSegment {
            start_seconds: seg_start,
            end_seconds: seg_end,
            file_name,
            file_end_seconds,
            left_boundary_is_shuffle_point: segment_index > 0,
            right_boundary_is_shuffle_point: segment_index < total_segments.saturating_sub(1),
        });

        let segment_channels = get_cached_peak_amplitudes_in_range(
            &window,
            &file_id,
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
        .map(|channel| {
            channel
                .into_iter()
                .map(|peak| (peak.max, peak.min))
                .collect()
        })
        .collect()
}
