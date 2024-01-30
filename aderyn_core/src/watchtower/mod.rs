pub mod lightchaser;
pub mod utils;

use serde::{Deserialize, Serialize};

use crate::detect::detector::IssueSeverity;

// TODO: See how to get in true_negatives and false_negatives.
// (We can complete the confusion matrix)

// Live data point for a given {detector_name}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metrics {
    /// Name of the detector (Should come from DetectorNamePool)
    pub detector_name: String,

    /// Total number of reports where no issue was found as a false positive
    /// and at least 1 issue was found by {detector_name}
    pub true_positives: u64,

    /// Total number of reports where at least 1 issue was found as a false positive
    pub false_positives: u64,

    /// Total trigger count. Ideally the invariant should be TP + FP == Trigger Count
    /// But we can't always count on that because some watch towers refuse to record TP, FP post some cap
    pub trigger_count: u64,

    /// Total number of reports this detector was "exposed" to (regardless of whether
    /// or not it has found issues)
    pub experience: u64,

    /// Currently asjusted severity
    pub current_severity: IssueSeverity,
}

pub struct Feedback {
    // TODO: Assert that there are never common elements among positive_feedbacks and negative_feedbacks in the same `Feedback` object
    pub positive_feedbacks: Vec<String>, // An array of {detector_names} that have performed well
    pub negative_feedbacks: Vec<String>, // An array of {detector_names} that has performed poorly
}

// Should be called when a new detector is introduced into the codebase
// TODO: create admin tool (binary crate)
pub trait RegisterNewDetector {
    fn accept(&self, detector_name: String, assigned_severity: IssueSeverity);
}

// Planned to be used by ReportPrinter and Aderyn Registry
pub trait GetCurrentMetricsForDetector {
    fn metrics(&self, detector_name: String) -> Metrics;
}

pub trait TakeFeedbackFromJudge {
    fn take_feedback(&self, feedback: Feedback);
}

pub trait InferMetrics {
    fn is_acceptable(&self) -> bool;
}
