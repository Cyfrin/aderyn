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
use aderyn_driver::{
    detector::IssueDetector,
    driver::{drive_with, Args},
};
use serde::Serialize;

fn custom_detectors() -> Vec<Box<dyn IssueDetector>> {
    vec![
        // ADERYN-PILOT: 0x02 CUSTOM DETECTORS - Do not remove this comment even if the array is empty
    ]
}

pub fn refresh_metadata() {
    let metadata: Metadata = custom_detectors().into();
    let path = PathBuf::from("metadata/custom_bots.json");
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

pub fn generate_report_for_judge(root: &str, output: &str) {
    drive_with(
        Args {
            root: root.to_string(),
            output: output.to_string(),
            exclude: None,
            scope: None,
            no_snippets: false,
        },
        custom_detectors(),
    )
}

impl From<Vec<Box<dyn IssueDetector>>> for Metadata {
    fn from(detectors: Vec<Box<dyn IssueDetector>>) -> Self {
        let mut custom_bots = vec![];
        for detector in detectors {
            let custom_bot = CustomBot {
                title: detector.title(),
                severity: detector.severity().to_string(),
                description: detector.description(),
            };
            custom_bots.push(custom_bot);
        }
        Metadata { custom_bots }
    }
}

#[derive(Serialize)]
struct Metadata {
    custom_bots: Vec<CustomBot>,
}

#[derive(Serialize)]
struct CustomBot {
    severity: String,
    title: String,
    description: String,
}
