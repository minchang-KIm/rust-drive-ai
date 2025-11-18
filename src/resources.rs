use bevy::prelude::*;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct SimStats {
    pub num_cars_alive: usize,
    pub fitness: Vec<f32>,
    pub generation_count: u32,
    pub max_current_score: f32,
}

impl SimStats {
    /// Export statistics to a JSON file
    pub fn export_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Export statistics with automatic timestamped filename
    pub fn export_auto(&self) -> Result<String, std::io::Error> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("saves/stats_{}.json", timestamp);

        // Create saves directory if it doesn't exist
        fs::create_dir_all("saves")?;

        self.export_to_file(&filename)?;
        Ok(filename)
    }

    /// Load statistics from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(path)?;
        let stats: SimStats = serde_json::from_str(&json)?;
        Ok(stats)
    }
}

#[derive(Resource)]
pub struct Settings {
    pub is_show_rays: bool,
    pub is_hide_rays_at_start: bool,
    pub start_next_generation: bool,
    pub restart_sim: bool,
    pub is_camera_follow: bool,
    pub save_best_brain: bool,
    pub load_brain: bool,
    pub export_stats: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            is_show_rays: true,
            is_hide_rays_at_start: true,
            start_next_generation: false,
            restart_sim: false,
            is_camera_follow: true,
            save_best_brain: false,
            load_brain: false,
            export_stats: false,
        }
    }
}

#[derive(Resource, Default)]
pub struct BrainToDisplay(pub Vec<Vec<f64>>);

#[derive(Resource, Default)]
pub struct MaxDistanceTravelled(pub f32);

#[derive(Resource, Default)]
pub struct BestBrain(pub Option<crate::nn::Net>);

impl BestBrain {
    /// Save the best brain with automatic timestamped filename
    pub fn save_auto(&self) -> Result<String, crate::nn::NeuralNetError> {
        if let Some(ref brain) = self.0 {
            let timestamp = Local::now().format("%Y%m%d_%H%M%S");
            let filename = format!("saves/best_brain_{}.json", timestamp);

            // Create saves directory if it doesn't exist
            fs::create_dir_all("saves").map_err(|e| crate::nn::NeuralNetError::Io(e))?;

            brain.save(&filename)?;
            Ok(filename)
        } else {
            Err(crate::nn::NeuralNetError::InvalidStructure(
                "No brain to save".to_string(),
            ))
        }
    }

    /// Load a brain from the latest save file
    pub fn load_latest() -> Result<crate::nn::Net, crate::nn::NeuralNetError> {
        let saves_dir = Path::new("saves");
        if !saves_dir.exists() {
            return Err(crate::nn::NeuralNetError::InvalidStructure(
                "No saves directory found".to_string(),
            ));
        }

        let mut brain_files: Vec<_> = fs::read_dir(saves_dir)
            .map_err(|e| crate::nn::NeuralNetError::Io(e))?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()?.to_str()? == "json"
                    && path.file_name()?.to_str()?.starts_with("best_brain_") {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        if brain_files.is_empty() {
            return Err(crate::nn::NeuralNetError::InvalidStructure(
                "No brain saves found".to_string(),
            ));
        }

        // Sort by filename (which includes timestamp) to get the latest
        brain_files.sort();
        let latest_file = brain_files.last().unwrap();

        crate::nn::Net::load(latest_file)
    }
}
