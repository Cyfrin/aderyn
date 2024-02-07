use aderyn_driver::detector::IssueDetector;
use aderyn_driver::driver::{drive_with, Args};

// Core detectors
use aderyn_driver::detection_modules::low::PushZeroOpcodeDetector;

// Custom detectors

// `cargo run` will run this function
pub fn run() {
    let subscriptions: Vec<Box<dyn IssueDetector>> = vec![
        // List of detectors to run in producing report
        Box::<PushZeroOpcodeDetector>::default(),
    ];

    drive_with(
        Args {
            root: "./foundry_workspace".to_string(),
            output: "report.md".to_string(),
            exclude: None,
            scope: None,
            no_snippets: false,
        },
        subscriptions,
    )
}
