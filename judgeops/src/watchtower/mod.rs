#![allow(clippy::borrowed_box)]

pub mod lightchaser;
pub mod utils;

use std::{collections::HashMap, process::ExitCode};

use serde::{Deserialize, Serialize};

use crate::IssueSeverity;

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

    /// Total trigger count - Number of times this detector has found issues (regardless of whether they are right or wrong)
    /// Ideally the invariant should be TP + FP == Trigger Count
    /// But we can't always count on that because some watch towers refuse to record TP, FP post some cap
    pub trigger_count: u64,

    /// Total number of reports this detector was "exposed" to (regardless of whether
    /// or not it has found issues)
    pub experience: u64,

    /// Currently asjusted severity
    pub current_severity: IssueSeverity,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feedback {
    // TODO: Assert that there are never common elements among positive_feedbacks and negative_feedbacks in the same `Feedback` object
    pub positive_feedbacks: Vec<String>, // An array of {detector_names} that have performed well
    pub negative_feedbacks: Vec<String>, // An array of {detector_names} that has performed poorly
    pub all_exposed_detectors: Vec<String>, // An array of {detector_name} that was exposed (regardless of whether or not they were triggered) to the codebase
}

pub trait RegistersNewDetector {
    fn register(&self, detector_name: String, assigned_severity: IssueSeverity);
}

pub trait UnregistersDetector {
    fn unregister_detector(&self, detector_name: String);
}

pub trait GetsRegisteredDetectors {
    fn get_registered_detectors_names(&self) -> Vec<String>;
}

pub trait GetsCurrentMetricsForDetector {
    fn metrics(&self, detector_name: String) -> Metrics;
    fn all_metrics(&self) -> HashMap<String, Metrics>;
}

pub trait SuggestsChanges {
    fn print_suggested_changes_before_init(&self) -> ExitCode;
}

pub trait TakesFeedbackFromJudge {
    /// NOTE - Make sure to be ready to take feedback before you take feedback
    fn take_feedback(&self, feedback: Feedback);
}

pub trait InfersMetrics {
    fn is_acceptable(&self) -> bool;
}

pub trait CalculatesValueOfDetector {
    /// All implementations MUST return a value from [0, 1] only
    fn value(&self, detector_name: String) -> f64;
    fn value_from_metrics(&self, metrics: &Metrics) -> f64;
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Tag {
    pub messages: Vec<String>,
}

pub trait WatchTower:
    RegistersNewDetector
    + GetsCurrentMetricsForDetector
    + SuggestsChanges
    + TakesFeedbackFromJudge
    + CalculatesValueOfDetector
    + GetsRegisteredDetectors
{
}
