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

use std::sync::{Arc, Mutex};
use std::thread;
use std::{collections::HashMap, fs::File};
use std::io::prelude::*;

use proteus_audio::peaks;
use serde::{Serialize, Deserialize};
use tauri::{Window, Manager};
use tauri::State;

use crate::project::ProjectSkeleton;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedPeaks {
    peaks: Vec<f32>,
    zoom: usize,
    original_length: usize,
}

pub fn simplify_peaks(peaks: Vec<Vec<(f32, f32)>>, zoom: usize) -> Vec<SimplifiedPeaks> {
    let mut simplified_peaks: Vec<SimplifiedPeaks> = Vec::new();

    if zoom > 20 || zoom < 1 {
        panic!("Zoom level must be between 1 and 20");
    }

    // Twenty different zoom levels from 1 to 20
    // which coorespond to 5% to 100% of the original peaks
    // with no repeating values
    let zoom_levels: Arc<[u32]> = Arc::new([100, 70, 50, 33, 25, 20, 16, 14, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
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

#[tauri::command]
pub fn get_json_peaks(window: &Window, file_id: &String, peaks_option: Option<Vec<Vec<(f32, f32)>>>) -> Vec<Vec<(f32, f32)>> {
    // let timer = std::time::Instant::now();
    let app_cache = window.app_handle().path_resolver().app_cache_dir().unwrap();
    let app_cache_dir = app_cache.to_str().unwrap();
    
    if peaks_option.is_some() {
        let timer = std::time::Instant::now();
        let peaks = peaks_option.unwrap();
        let mut peaks_file = File::create(format!("{}/{}.json", app_cache_dir, file_id)).unwrap();

        let timer = std::time::Instant::now();
        let peaks_json = serde_json::to_string(&peaks).unwrap();
        peaks_file.write(peaks_json.as_bytes()).unwrap();
        
        return peaks;
    }
    
    let peaks_file = File::open(format!("{}/{}.json", app_cache_dir, file_id));

    match peaks_file {
        Ok(mut peaks_file) => {
            let timer = std::time::Instant::now();
            let mut peaks_json = String::new();
            let timer = std::time::Instant::now();
            peaks_file.read_to_string(&mut peaks_json).unwrap();
            let timer = std::time::Instant::now();
            let peaks: Vec<Vec<(f32, f32)>> = serde_json::from_str(&peaks_json).unwrap();
            return peaks;
        },
        Err(_) => {
            let project_state: State<Arc<Mutex<ProjectSkeleton>>> = window.state();

            let project = project_state.lock().unwrap();
            let file_path = project.files.iter().find(|f| f.id == *file_id).unwrap().path.clone();

            let peaks = proteus_audio::peaks::get_peaks(&file_path, true);

            let peaks_json = serde_json::to_string(&peaks).unwrap();
            let mut peaks_file = File::create(format!("{}/{}.json", app_cache_dir, file_id)).unwrap();
            peaks_file.write(peaks_json.as_bytes()).unwrap();

            save_svgs_in_new_thread_for_each_zoom_level(peaks.clone(), format!("{}/{}.svg", app_cache_dir, file_id));

            return peaks
        }
    }

}



pub fn make_svg_from_peaks(peaks: Vec<Vec<(f32, f32)>>, height: u32) -> String {
    let mut svg = String::new();

    let width = peaks[0].len() as u32;

    svg.push_str(&format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">", width, height));

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

    svg.push_str(&format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">", width, height));

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