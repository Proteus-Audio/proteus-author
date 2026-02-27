use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct WaveformSegment {
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub file_name: String,
    pub file_end_seconds: f64,
    pub left_boundary_is_shuffle_point: bool,
    pub right_boundary_is_shuffle_point: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrackWaveformView {
    pub channels: Vec<Vec<f32>>,
    pub segments: Vec<WaveformSegment>,
}
