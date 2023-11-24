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

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedPeaks {
    peaks: Vec<f32>,
    zoom: u32,
    original_length: usize,
}

pub fn simplify_peaks(peaks: Vec<Vec<(f32, f32)>>, zoom: u32) -> Vec<SimplifiedPeaks> {
    let mut simplified_peaks: Vec<SimplifiedPeaks> = Vec::new();

    // Twenty different zoom levels from 1 to 20
    // which coorespond to 5% to 100% of the original peaks
    // with no repeating values
    let zoom_levels: HashMap<u32, u32> = [
        (1, 100),
        (2, 70),
        (3, 50),
        (4, 33),
        (5, 25),
        (6, 20),
        (7, 16),
        (8, 14),
        (9, 12),
        (10, 11),
        (11, 10),
        (12, 9),
        (13, 8),
        (14, 7),
        (15, 6),
        (16, 5),
        (17, 4),
        (18, 3),
        (19, 2),
        (20, 1),
    ]
    .iter()
    .cloned()
    .collect();

    println!("zoom: {}", zoom);

    let group_size: usize = zoom_levels[&zoom] as usize;

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
