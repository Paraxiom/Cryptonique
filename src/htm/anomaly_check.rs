//anomaly_check.rs
#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::htm::htm_model::HTMModel;
use crate::htm::temporal_keys::TemporalKey;
use std::collections::VecDeque;

const ANOMALY_THRESHOLD: f32 = 0.8; // Set an arbitrary threshold for anomaly detection

pub struct AnomalyChecker {
    recent_keys: VecDeque<TemporalKey>,
}

impl AnomalyChecker {
    pub fn new() -> Self {
        Self {
            recent_keys: VecDeque::new(),
        }
    }

    // Store the recent keys for analysis
    pub fn add_key(&mut self, key: TemporalKey) {
        if self.recent_keys.len() >= 10 {
            // Keep last 10 keys for checking
            self.recent_keys.pop_front();
        }
        self.recent_keys.push_back(key);
    }

    // Check for anomalies based on recent keys
    pub fn check_anomaly(&self, htm_model: &HTMModel) -> bool {
        let mut anomaly_score = 0.0;

        // Calculate an 'anomaly score' here based on your specific metrics
        // For example, you could compare the recent keys against the current state of the HTM model
        // Anomaly score is a float between 0 and 1 where higher values mean more anomalous

        if anomaly_score > ANOMALY_THRESHOLD {
            return true;
        }
        false
    }
}
