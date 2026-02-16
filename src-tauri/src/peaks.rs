// pub fn simplify_peaks(peaks: Vec<Vec<(f32, f32)>>, zoom: f32) -> Vec<Vec<(f32, f32)>> {
//     let mut simplified_peaks: Vec<Vec<(f32, f32)>> = Vec::new();
//     let group_size: usize = (100.0 / zoom) as usize;

//     for channel_peaks in peaks {
//         let mut group: Vec<(f32, f32)> = Vec::new();
//         let mut channel_simplified: Vec<(f32, f32)> = Vec::new();

//         for (index, peak) in channel_peaks.iter().enumerate() {
//             if group.len() < group_size {
//                 group.push(*peak);
//             }

//             let is_last = index == channel_peaks.len() - 1;

//             if group.len() == group_size || is_last {
//                 let min = group.iter().map(|p| p.0).fold(f32::INFINITY, f32::min);
//                 let max = group.iter().map(|p| p.1).fold(f32::NEG_INFINITY, f32::max);
//                 // println!("min: {}, max: {}", min, max);
//                 channel_simplified.push((min, max));
//                 group.clear();
//             }
//         }

//         simplified_peaks.push(channel_simplified);
//     }

//     simplified_peaks
// }

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use tauri::Manager;
use tauri::State;
use tauri::Window;

use crate::helpers::get_cache_dir;

use crate::project::{read_project, WindowProjectState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedPeaks {
    peaks: Vec<f32>,
    zoom: usize,
    original_length: usize,
}

fn to_legacy_peaks(peaks_data: proteus_lib::peaks::PeaksData) -> Vec<Vec<(f32, f32)>> {
    peaks_data
        .channels
        .into_iter()
        .map(|channel| {
            channel
                .into_iter()
                .map(|window| (window.max, window.min))
                .collect()
        })
        .collect()
}

fn peaks_duration_seconds(peaks_data: &proteus_lib::peaks::PeaksData) -> f64 {
    if peaks_data.channels.is_empty() || peaks_data.sample_rate == 0 || peaks_data.window_size == 0
    {
        return 0.0;
    }

    let peak_count = peaks_data.channels[0].len() as f64;
    let samples = peak_count * peaks_data.window_size as f64;
    samples / peaks_data.sample_rate as f64
}

pub fn get_cached_peak_duration_seconds(window: &Window, file_id: &str) -> f64 {
    let peaks_file_path = ensure_peaks_file(window, file_id);
    let peaks_data = proteus_lib::peaks::get_peaks(&peaks_file_path, Default::default())
        .expect("failed to read .peaks");
    peaks_duration_seconds(&peaks_data).max(0.0)
}

fn get_peaks_file_path(window: &Window, file_id: &str) -> String {
    let app_cache = get_cache_dir(window).unwrap();
    format!("{}/{}.peaks", app_cache, file_id)
}

fn ensure_peaks_file(window: &Window, file_id: &str) -> String {
    let peaks_file_path = get_peaks_file_path(window, file_id);

    if Path::new(&peaks_file_path).exists() {
        return peaks_file_path;
    }

    let project_state: State<WindowProjectState> = window.state();
    let project = read_project(window, &project_state);
    let file_path = project
        .files
        .iter()
        .find(|f| f.id == file_id)
        .unwrap()
        .path
        .clone();

    proteus_lib::peaks::write_peaks(&file_path, &peaks_file_path)
        .expect("failed to write .peaks file");

    let peaks_data = proteus_lib::peaks::get_peaks(&peaks_file_path, Default::default())
        .expect("failed to read .peaks file after writing");
    let peaks = to_legacy_peaks(peaks_data);

    let app_cache = get_cache_dir(window).unwrap();
    save_svgs_in_new_thread_for_each_zoom_level(peaks, format!("{}/{}.svg", app_cache, file_id));

    peaks_file_path
}

pub fn simplify_peaks(peaks: Vec<Vec<(f32, f32)>>, zoom: usize) -> Vec<SimplifiedPeaks> {
    let mut simplified_peaks: Vec<SimplifiedPeaks> = Vec::new();

    if zoom > 20 || zoom < 1 {
        panic!("Zoom level must be between 1 and 20");
    }

    // Twenty different zoom levels from 1 to 20
    // which coorespond to 5% to 100% of the original peaks
    // with no repeating values
    let zoom_levels: Arc<[u32]> = Arc::new([
        100, 70, 50, 33, 25, 20, 16, 14, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
    ]);
    let group_size: usize = zoom_levels[zoom + 1] as usize;

    for channel_peaks in peaks {
        let mut group: Vec<(f32, f32)> = Vec::new();
        // let mut channel_simplified: Vec<(f32, f32)> = Vec::new();
        let mut channel_simplified: Vec<f32> = Vec::new();

        for (index, peak) in channel_peaks.iter().enumerate() {
            if group.len() < group_size {
                group.push(*peak);
            }

            let is_last = index == channel_peaks.len() - 1;

            if group.len() == group_size || is_last {
                let min = group.iter().map(|p| p.0).fold(f32::INFINITY, f32::min);
                let max = group.iter().map(|p| p.1).fold(f32::NEG_INFINITY, f32::max);
                // println!("min: {}, max: {}", min, max);
                // channel_simplified.push((min, max));

                let avg = (min.abs() + max.abs()) / 2.0;
                channel_simplified.push(avg);
                group.clear();
            }
        }

        simplified_peaks.push(SimplifiedPeaks {
            peaks: channel_simplified,
            zoom,
            original_length: channel_peaks.len(),
        });
    }

    simplified_peaks
}

pub fn get_cached_peaks(window: &Window, file_id: &str) -> Vec<Vec<(f32, f32)>> {
    let peaks_file_path = ensure_peaks_file(window, file_id);
    let peaks_data = proteus_lib::peaks::get_peaks(&peaks_file_path, Default::default())
        .expect("failed to read .peaks");
    to_legacy_peaks(peaks_data)
}

pub fn get_cached_peaks_in_range(
    window: &Window,
    file_id: &str,
    start_seconds: f64,
    end_seconds: f64,
) -> Vec<Vec<(f32, f32)>> {
    let peaks_file_path = ensure_peaks_file(window, file_id);
    let peaks_data =
        proteus_lib::peaks::get_peaks_in_range(&peaks_file_path, start_seconds, end_seconds)
            .expect("failed to read .peaks range");
    to_legacy_peaks(peaks_data)
}

pub fn get_cached_peaks_for_full_duration(window: &Window, file_id: &str) -> Vec<Vec<(f32, f32)>> {
    let peaks_file_path = ensure_peaks_file(window, file_id);
    let full_peaks = proteus_lib::peaks::get_peaks(&peaks_file_path, Default::default())
        .expect("failed to read .peaks");
    let end_seconds = peaks_duration_seconds(&full_peaks);

    if end_seconds <= 0.0 {
        return to_legacy_peaks(full_peaks);
    }

    get_cached_peaks_in_range(window, file_id, 0.0, end_seconds)
}

pub fn get_cached_peak_amplitudes_in_range(
    window: &Window,
    file_id: &str,
    start_seconds: f64,
    end_seconds: f64,
    target_peaks: usize,
) -> Vec<Vec<f32>> {
    let peaks_file_path = ensure_peaks_file(window, file_id);
    let clamped_start = start_seconds.max(0.0);
    let clamped_end = end_seconds.max(clamped_start + 0.001);
    let options = proteus_lib::peaks::GetPeaksOptions {
        start_seconds: Some(clamped_start),
        end_seconds: Some(clamped_end),
        target_peaks: Some(target_peaks.max(1)),
        channels: None,
    };

    let peaks_data =
        proteus_lib::peaks::get_peaks(&peaks_file_path, options).expect("failed to read .peaks");

    peaks_data
        .channels
        .into_iter()
        .map(|channel| {
            channel
                .into_iter()
                .map(|window| ((window.min.abs() + window.max.abs()) / 2.0).clamp(0.0, 1.0))
                .collect()
        })
        .collect()
}

pub fn make_svg_from_peaks(peaks: Vec<Vec<(f32, f32)>>, height: u32) -> String {
    let mut svg = String::new();

    let width = peaks[0].len() as u32;

    svg.push_str(&format!(
        "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
        width, height
    ));

    let mut x = 0;
    let y = (height / 2) as i32;

    for channel in peaks {
        for peak in channel {
            let max_peak = peak.0;
            let min_peak = peak.1;

            let max_peak_y = (max_peak * height as f32 / 2.0) as i32;
            let min_peak_y = (min_peak * height as f32 / 2.0) as i32;

            svg.push_str(&format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" stroke-width=\"1\" />", x, y + max_peak_y, x, y + min_peak_y));

            x += 1;
        }
    }

    svg.push_str("</svg>");

    svg
}

pub fn make_svg_from_simplified_peaks(peaks: Vec<SimplifiedPeaks>, height: u32) -> String {
    let mut svg = String::new();

    let width = peaks[0].peaks.len() as u32;

    svg.push_str(&format!(
        "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
        width, height
    ));

    let mut x = 0;
    let y = (height / 2) as i32;

    for channel in peaks {
        for peak in channel.peaks {
            let max_peak = peak;
            let min_peak = -peak;

            let max_peak_y = (max_peak * height as f32 / 2.0) as i32;
            let min_peak_y = (min_peak * height as f32 / 2.0) as i32;

            svg.push_str(&format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" stroke-width=\"1\" />", x, y + max_peak_y, x, y + min_peak_y));

            x += 1;
        }
    }

    svg.push_str("</svg>");

    svg
}

pub fn save_svg_in_new_thread(peaks: Vec<Vec<(f32, f32)>>, file_path: String) {
    thread::spawn(move || {
        let svg = make_svg_from_peaks(peaks, 1000);
        let mut svg_file = File::create(file_path).unwrap();
        svg_file.write(svg.as_bytes()).unwrap();
    });
}

pub fn save_svgs_in_new_thread_for_each_zoom_level(peaks: Vec<Vec<(f32, f32)>>, file_path: String) {
    thread::spawn(move || {
        // Replace the above with for loop 1 - 20
        for zoom in 1..=20 {
            let svg = make_svg_from_simplified_peaks(simplify_peaks(peaks.clone(), zoom), 1000);
            let mut svg_file = File::create(format!("{}-{}.svg", file_path, zoom)).unwrap();
            svg_file.write(svg.as_bytes()).unwrap();
        }
    });
}
