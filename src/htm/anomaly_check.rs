use crate::htm::htm_model::HTMModel;
use crate::htm::temporal_keys::TemporalKey;
use std::collections::VecDeque;

// Configurable parameters for anomaly checking
pub struct AnomalyCheckerConfig {
    pub window_size: usize,
    pub dynamic_threshold: bool,
    pub static_threshold: f32,
    pub complexity_metric: bool,
    pub temporal_correlation: bool,
}

// Anomaly checker structure
pub struct AnomalyChecker {
    recent_keys: VecDeque<TemporalKey>,
    config: AnomalyCheckerConfig,
}

impl AnomalyChecker {
    // Initialize with configuration
    pub fn new(config: AnomalyCheckerConfig) -> Self {
        Self {
            recent_keys: VecDeque::with_capacity(config.window_size),
            config,
        }
    }

    // Store the recent keys for analysis
    pub fn add_key(&mut self, key: TemporalKey) {
        if self.recent_keys.len() >= self.config.window_size {
            self.recent_keys.pop_front();
        }
        self.recent_keys.push_back(key);
    }

  // Check for anomalies based on recent keys and HTM model
  pub fn check_anomaly(&self, htm_model: &HTMModel) -> bool {
    let anomaly_score = self.calculate_anomaly_score(htm_model);
    let threshold = if self.config.dynamic_threshold {
        self.calculate_dynamic_threshold()
    } else {
        self.config.static_threshold
    };

    anomaly_score > threshold
}

// Calculate the anomaly score based on the recent keys and HTM model
fn calculate_anomaly_score(&self, htm_model: &HTMModel) -> f32 {
    let mut score = 0.0;

    // Example logic: Sum of differences between expected and actual keys
    for key in &self.recent_keys {
        score += htm_model.compare_with_expected(key);
    }

    if self.config.complexity_metric {
        score += self.calculate_complexity_metric();
    }

    if self.config.temporal_correlation {
        score += self.calculate_temporal_correlation();
    }

    score
}

// Example function to calculate a complexity metric
fn calculate_complexity_metric(&self) -> f32 {
    // Example: Variance of the TemporalKeys
    let mean = self.recent_keys.iter().map(|k| k.value()).sum::<f32>() / self.recent_keys.len() as f32;
    self.recent_keys.iter().map(|k| (k.value() - mean).powi(2)).sum::<f32>() / self.recent_keys.len() as f32
}

// Example function to calculate temporal correlation
fn calculate_temporal_correlation(&self) -> f32 {
    // Implement the logic for temporal correlation analysis
    0.0
}

// Calculate dynamic threshold based on recent anomaly scores
fn calculate_dynamic_threshold(&self) -> f32 {
    // Example: Average of recent scores
    let sum: f32 = self.recent_keys.iter().map(|k| k.anomaly_score()).sum();
    let count = self.recent_keys.len() as f32;
    sum / count
}

    // Additional helper methods can be added as needed
}

// Implement unit tests for AnomalyChecker
#[cfg(test)]
mod tests {
    use super::*;

    // Write tests here to validate the functionality of AnomalyChecker
}
