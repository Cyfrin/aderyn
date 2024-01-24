use std::collections::HashMap;

use aderyn_driver::detector::Detector;

pub struct Grounded; // Contract has been given but no detectors attached
pub struct Launchable; // Contract has been given + detectors are attached

pub struct TestsTarget<Stage = Grounded> {
    stage: std::marker::PhantomData<Stage>,
    pub filepath: String,
    pub detectors: Vec<Box<dyn Detector>>,
}

impl TestsTarget<Grounded> {
    pub fn new(filepath: &str) -> Self {
        Self {
            stage: std::marker::PhantomData::<Grounded>,
            filepath: filepath.to_string(),
            detectors: vec![],
        }
    }

    pub fn with(self, detector: Box<dyn Detector>) -> TestsTarget<Launchable> {
        let mut so_far = self.detectors;
        so_far.push(detector);
        TestsTarget::<Launchable> {
            stage: std::marker::PhantomData::<Launchable>,
            filepath: self.filepath,
            detectors: so_far,
        }
    }
}

impl TestsTarget<Launchable> {
    pub fn with(self, detector: Box<dyn Detector>) -> TestsTarget<Launchable> {
        let mut so_far = self.detectors;
        so_far.push(detector);
        TestsTarget::<Launchable> {
            stage: std::marker::PhantomData::<Launchable>,
            filepath: self.filepath,
            detectors: so_far,
        }
    }
}

// For now, let's assume title() is unique for each detector -
// TODO later: implement an id method on detectors
pub type DetectorId = String;
pub type ContractJSONFilepath = String;
pub struct TestsConfig {
    store: HashMap<DetectorId, Vec<ContractJSONFilepath>>,
}

impl From<Vec<TestsTarget<Launchable>>> for TestsConfig {
    fn from(tests_targets: Vec<TestsTarget<Launchable>>) -> Self {
        let mut store: HashMap<DetectorId, Vec<ContractJSONFilepath>> = HashMap::default();

        tests_targets.iter().for_each(|t| {
            let filepath = t.filepath.to_string();
            t.detectors.iter().for_each(|d| {
                if let Some(ptr) = store.get_mut(&d.title()) {
                    ptr.push(filepath.clone());
                } else {
                    store.insert(d.title(), vec![filepath.clone()]);
                }
            });
        });

        Self { store }
    }
}

impl TestsConfig {
    pub fn get_contracts_for(&self, detector_id: DetectorId) -> Vec<ContractJSONFilepath> {
        self.store.get(&detector_id).unwrap_or(&vec![]).clone()
    }
}
