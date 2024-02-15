// ADERYN-PILOT: 0x01 (Please feel free to fix above imports if they mess up)

use std::{fs::OpenOptions, io::BufWriter, path::PathBuf};

/**
 *
 * Why this exists ?
 *  - To refresh the metadata when changes are made to the detectors
 *  - When you generate a new detector it will be added below
 *
 * IMPORTANT
 *  - Do not EVER remove any comments that start with ADERYN-PILOT: 0x
 *  - Do not add any comments of your own, change function definitions, etc
 *  - However, YOU ARE ALLOWED to modify the custom_detectors array so long as you maintain the original structure.
 */
use aderyn_driver::detector::IssueDetector;
use serde::{Deserialize, Serialize};

fn custom_detectors() -> Vec<Box<dyn IssueDetector>> {
    vec![
        // ADERYN-PILOT: 0x02 CUSTOM DETECTORS - Do not remove this comment even if the array is empty
    ]
}

pub fn refresh_metadata() {
    let metadata: Metadata = custom_detectors().into();
    let path = PathBuf::from("metadata/custom_detectors.json");
    _ = std::fs::remove_file(&path); // OK to fail

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)
        .unwrap();

    let bw = BufWriter::new(file);

    let value = serde_json::to_value(metadata).unwrap();
    _ = serde_json::to_writer_pretty(bw, &value);
}

impl From<Vec<Box<dyn IssueDetector>>> for Metadata {
    fn from(detectors: Vec<Box<dyn IssueDetector>>) -> Self {
        let mut custom_detectors = vec![];
        for detector in detectors {
            let custom_bot = CustomDetector {
                name: detector.name(),
                title: detector.title(),
                severity: detector.severity().to_string(),
                description: detector.description(),
            };
            custom_detectors.push(custom_bot);
        }
        Metadata { custom_detectors }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub custom_detectors: Vec<CustomDetector>,
}

#[derive(Serialize, Deserialize)]
pub struct CustomDetector {
    pub severity: String,
    pub title: String,
    pub description: String,
    pub name: String,
}
