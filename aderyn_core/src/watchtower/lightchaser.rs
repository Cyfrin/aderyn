// LightChaser is one of the flavors that implements WatchTower
// Twitter Handle - https://twitter.com/ChaseTheLight99
//
// Inspired from series on twitter -
//
// Chapter 001 Dynamic Accuracy
// https://twitter.com/ChaseTheLight99/status/1749521023446171722
//
// Chapter 002 Quality Assurance
// https://twitter.com/ChaseTheLight99/status/1749830985884090785
//
// Chapter 003 Dynamic Accuracy
// https://twitter.com/ChaseTheLight99/status/1750508847947522519
//
// And more...

use strum::EnumCount;

use crate::detect::detector::{get_all_detectors_names, IssueSeverity};

use super::utils::MetricsDatabase;
use super::{
    GetCurrentMetricsForDetector, InferMetrics, Metrics, RegisterNewDetector, TakeFeedbackFromJudge,
};

pub struct LightChaser {
    metrics_db: MetricsDatabase,
}

impl RegisterNewDetector for LightChaser {
    fn accept(&self, detector_name: String, assigned_severity: IssueSeverity) {
        self.metrics_db
            .register_new_detector(detector_name, assigned_severity);
    }
}

impl GetCurrentMetricsForDetector for LightChaser {
    fn metrics(&self, detector_name: String) -> Metrics {
        self.metrics_db.get_metrics(detector_name)
    }
}

impl TakeFeedbackFromJudge for LightChaser {
    /// Light Chaser caps the value of TP - FP from 0 to IssueSeverity::COUNT
    /// Refer to Chapter 001 - Link at the top of this page
    fn take_feedback(&self, feedback: super::Feedback) {
        feedback.negative_feedbacks.iter().for_each(|fdbck| {
            let current_metrics = self.metrics_db.get_metrics(fdbck.clone());
            let current_lc_accuracy = current_metrics.lc_accuracy();

            if current_lc_accuracy >= 1 {
                self.metrics_db
                    .increase_false_positive_with_trigger_count(fdbck.clone());
            } else {
                self.metrics_db.increase_trigger_count(fdbck.clone());
                eprintln!("WARNING: detector {}'s lc_accuracy is at 0 !!! Unable to process Negative Feedback", fdbck);
            }
        });

        feedback.positive_feedbacks.iter().for_each(|fdbck| {
            let current_metrics = self.metrics_db.get_metrics(fdbck.clone());
            let current_lc_accuracy = current_metrics.lc_accuracy();

            if current_lc_accuracy < (IssueSeverity::COUNT - 1) as u64 {
                // NOTE: LightChaser follows the "Once imperfect, always imperfect strategy"
                // Therefore, score can never "rise" to a perfect IssueSeverity::COUNT
                self.metrics_db
                    .increase_true_positive_with_trigger_count(fdbck.clone());
            } else {
                self.metrics_db.increase_trigger_count(fdbck.clone());
            }
        });

        let all_detector_names = get_all_detectors_names();
        all_detector_names
            .into_iter()
            .for_each(|detector_name| self.metrics_db.increase_experience(detector_name));
    }
}

// TODO
// When initialized check if `get_all_detectors` reflects what is serialized in lc-metrics.json

impl InferMetrics for Metrics {
    fn is_acceptable(&self) -> bool {
        //"If a detector is triggered, its accuracy score is looked up and the following cap is applied."
        //      - @ChaseTheLight99, 2024
        // Chapter 003 - Dynamic Accuracy https://twitter.com/ChaseTheLight99/status/1750508847947522519
        let lc_accuracy = self.lc_accuracy();
        match self.current_severity {
            IssueSeverity::Critical => lc_accuracy == IssueSeverity::COUNT as u64,
            IssueSeverity::High => lc_accuracy >= IssueSeverity::COUNT as u64 - 1,
            IssueSeverity::Medium => lc_accuracy >= IssueSeverity::COUNT as u64 - 2,
            IssueSeverity::Low => lc_accuracy >= IssueSeverity::COUNT as u64 - 3,
            IssueSeverity::NC => lc_accuracy >= IssueSeverity::COUNT as u64 - 4,
        }
    }
}

impl Metrics {
    pub(crate) fn lc_accuracy(&self) -> u64 {
        // NOTE: TP - FP is scientifically not the same as accuracy, however this value is referred as
        // "accuracy" in the lc (light chaser) tweets. So I'm putting that out here "as is".
        self.true_positives - self.false_positives
    }
}
