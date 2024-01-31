use aderyn_core::{
    detect::detector::{get_all_detectors_names, get_detector_by_name},
    watchtower::WatchTower,
};

pub(crate) fn auto_register_new_core_detectors(watch_tower: &Box<dyn WatchTower>) {
    // Step 1 - Get the difference
    let existing_watch_tower_detectors = watch_tower.get_registered_detectors_names();
    let current_core_detectors_names = get_all_detectors_names();
    let mut extras = vec![];
    for core_detector in current_core_detectors_names {
        if !existing_watch_tower_detectors
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
        watch_tower.register(new_detector, detector.severity());
    }
}
