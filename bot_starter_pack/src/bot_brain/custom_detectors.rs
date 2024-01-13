// ADERYN-PILOT: 0X01 (Please feel free to fix above imports if they mess up)

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
// ADERYN-PILOT: 0x02 BASIC IMPORTS
use aderyn_driver::detector::Detector;

// ADERYN-PILOT: 0x03 fn custom_detectors
fn custom_detectors() -> Vec<Box<dyn Detector>> {
    vec![
        // ADERYN-PILOT: 0x04 CUSTOM DETECTORS - Do not remove this comment even if the array is empty
    ]
}

pub fn refresh_metadata() {
    println!("[*] Refreshing metadata");
    _ = custom_detectors();
    // TODO: serialize this to disk
}
