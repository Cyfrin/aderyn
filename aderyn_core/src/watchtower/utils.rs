use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::EnumCount;

use crate::detect::detector::IssueSeverity;

use super::Metrics;

#[derive(Serialize, Deserialize, Debug)]
pub struct MetricsDatabase {
    pub metrics: HashMap<String, Metrics>,
    pub db_path: String,
}

impl MetricsDatabase {
    fn get_current_db(&self) -> MetricsDatabase {
        let db_content = std::fs::read_to_string(&self.db_path).unwrap();
        serde_json::from_str(&db_content).unwrap()
    }

    fn save_db(&self, metrics_db: MetricsDatabase) {
        let db_content = serde_json::to_string_pretty(&metrics_db).unwrap();
        std::fs::write(&self.db_path, db_content).unwrap();
    }

    pub fn register_new_detector(&self, name: String, current_severity: IssueSeverity) {
        let mut state = self.get_current_db();

        if state.metrics.contains_key(&name) {
            panic!("Database already contains that key !")
        }

        let payload = Metrics {
            detector_name: name.clone(),
            true_positives: IssueSeverity::COUNT as u64,
            false_positives: 0,
            trigger_count: IssueSeverity::COUNT as u64,
            experience: IssueSeverity::COUNT as u64,
            current_severity,
        };

        state.metrics.insert(name, payload);
        self.save_db(state);
    }

    pub fn increase_true_positive_with_trigger_count(&self, name: String) {
        let mut state = self.get_current_db();
        let current_tp = state.metrics.get(&name).unwrap().true_positives;
        let current_trigger_count = state.metrics.get(&name).unwrap().trigger_count;
        state.metrics.get_mut(&name).unwrap().true_positives = current_tp + 1;
        state.metrics.get_mut(&name).unwrap().trigger_count = current_trigger_count + 1;
        self.save_db(state);
    }

    pub fn increase_false_positive_with_trigger_count(&self, name: String) {
        let mut state = self.get_current_db();
        let current_tp = state.metrics.get(&name).unwrap().false_positives;
        let current_trigger_count = state.metrics.get(&name).unwrap().trigger_count;
        state.metrics.get_mut(&name).unwrap().false_positives = current_tp + 1;
        state.metrics.get_mut(&name).unwrap().trigger_count = current_trigger_count + 1;
        self.save_db(state);
    }

    pub fn increase_trigger_count(&self, name: String) {
        let mut state = self.get_current_db();
        let current_trigger_count = state.metrics.get(&name).unwrap().trigger_count;
        state.metrics.get_mut(&name).unwrap().trigger_count = current_trigger_count + 1;
        self.save_db(state);
    }

    pub fn get_metrics(&self, detector_name: String) -> Metrics {
        let state = self.get_current_db();
        state.metrics.get(&detector_name).unwrap().clone()
    }

    pub fn increase_experience(&self, name: String) {
        let mut state = self.get_current_db();
        let current_exp = state.metrics.get(&name).unwrap().experience;
        state.metrics.get_mut(&name).unwrap().experience = current_exp + 1;
        self.save_db(state);
    }

    pub fn get_all_detectors_names(&self) -> Vec<String> {
        let state = self.get_current_db();
        state.metrics.into_keys().collect()
    }
}
