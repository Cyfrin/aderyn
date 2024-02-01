use std::process::ExitCode;

use aderyn_core::{
    detect::detector::{get_all_detectors_names, get_detector_by_name},
    watchtower::WatchTower,
};

pub(crate) fn auto_register_new_core_detectors(watchtower: &Box<dyn WatchTower>) {
    // Step 1 - Get the difference
    let existing_watchtower_detectors = watchtower.get_registered_detectors_names();
    let current_core_detectors_names = get_all_detectors_names();
    let mut extras = vec![];
    for core_detector in current_core_detectors_names {
        if !existing_watchtower_detectors
            .iter()
            .any(|t| t.clone() == core_detector)
        {
            extras.push(core_detector);
        }
    }

    if extras.len() == 0 {
        println!("There are no new detectors to register!");
        return;
    }

    // Step 2 - Register each new detector
    for new_detector in extras {
        let detector = get_detector_by_name(&new_detector);
        println!(
            "Registering {} with severity {} ",
            new_detector,
            detector.severity()
        );
        watchtower.register(new_detector, detector.severity());
    }
}

pub(crate) fn tag_detector(
    watchtower: &Box<dyn WatchTower>,
    detector_name: &str,
    message: &str,
) -> ExitCode {
    let existing_watchtower_detectors = watchtower.get_registered_detectors_names();
    if !existing_watchtower_detectors
        .iter()
        .any(|d| d == detector_name)
    {
        ExitCode::FAILURE
    }
}
