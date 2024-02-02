use std::collections::HashMap;

use aderyn_core::watchtower::{InfersMetrics, Metrics, WatchTower};

// This struct contains a snapshot of all the metrics for all the detectors
// before and after a change. It has the ability to summarize the changes
pub(crate) struct MetricsChangeSummarizer {
    // Key - detector_name
    pub(crate) before_metrics: HashMap<String, Metrics>,
    pub(crate) after_metrics: HashMap<String, Metrics>,
}

impl MetricsChangeSummarizer {
    pub(crate) fn print_summary_of_changes(&self, watchtower: &Box<dyn WatchTower>) {
        let detectors_names = watchtower.get_registered_detectors_names();
        let mut improved_detectors = vec![];
        let mut deteriorated_detectors = vec![];
        let mut detectors_on_life_support = vec![];

        for name in &detectors_names {
            let before_metrics = self.before_metrics.get(name).unwrap();
            let after_metrics = self.after_metrics.get(name).unwrap();

            let before_value = watchtower.value_from_metrics(before_metrics);
            let after_value = watchtower.value_from_metrics(after_metrics);

            let is_accepted_now = after_metrics.is_acceptable();

            if !is_accepted_now {
                detectors_on_life_support.push(name.clone());
            }

            if before_value < after_value {
                improved_detectors.push((name, before_value, after_value));
            } else if before_value > after_value {
                deteriorated_detectors.push((name, before_value, after_value));
            }
        }

        if !improved_detectors.is_empty() {
            println!("Detectors that have improved in value");
            improved_detectors
                .iter()
                .for_each(|(detector_name, before_value, after_value)| {
                    println!(
                        "{:.2} -> {:.2}\t:{}",
                        before_value, after_value, detector_name
                    );
                });
        }

        if !deteriorated_detectors.is_empty() {
            println!("\nDetectors that have deteriorated in value");
            deteriorated_detectors
                .iter()
                .for_each(|(detector_name, before_value, after_value)| {
                    println!(
                        "{:.2} -> {:.2}\t:{}",
                        before_value, after_value, detector_name
                    );
                });
        }

        if !detectors_on_life_support.is_empty() {
            println!(
                "\nDetectors on life support\n{}",
                detectors_on_life_support.join(", ")
            );
        }
    }
}
