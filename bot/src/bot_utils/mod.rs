use std::collections::HashMap;

use aderyn_driver::detector::{IssueDetector, ReusableDetector};

pub struct Grounded; // Contract has been given but no detectors attached
pub struct Launchable; // Contract has been given + detectors are attached

pub struct TestsTarget<Stage = Grounded> {
    stage: std::marker::PhantomData<Stage>,
    pub filepath: String,
    pub issue_detectors: Vec<Box<dyn IssueDetector>>,
    pub reusables_detectors: Vec<Box<dyn ReusableDetector>>,
}

impl TestsTarget<Grounded> {
    pub fn new(filepath: &str) -> Self {
        Self {
            stage: std::marker::PhantomData::<Grounded>,
            filepath: filepath.to_string(),
            issue_detectors: vec![],
            reusables_detectors: vec![],
        }
    }

    pub fn with_issue_detector(self, detector: Box<dyn IssueDetector>) -> TestsTarget<Launchable> {
        let mut detectors_so_far = self.issue_detectors;
        detectors_so_far.push(detector);

        TestsTarget::<Launchable> {
            stage: std::marker::PhantomData::<Launchable>,
            filepath: self.filepath,
            issue_detectors: detectors_so_far,
            reusables_detectors: self.reusables_detectors,
        }
    }

    pub fn with_reusable_detector(
        self,
        detector: Box<dyn ReusableDetector>,
    ) -> TestsTarget<Launchable> {
        let mut detectors_so_far = self.reusables_detectors;
        detectors_so_far.push(detector);

        TestsTarget::<Launchable> {
            stage: std::marker::PhantomData::<Launchable>,
            filepath: self.filepath,
            issue_detectors: self.issue_detectors,
            reusables_detectors: detectors_so_far,
        }
    }
}

impl TestsTarget<Launchable> {
    pub fn with_issue_detector(self, detector: Box<dyn IssueDetector>) -> TestsTarget<Launchable> {
        let mut detectors_so_far = self.issue_detectors;
        detectors_so_far.push(detector);
        TestsTarget::<Launchable> {
            stage: std::marker::PhantomData::<Launchable>,
            filepath: self.filepath,
            issue_detectors: detectors_so_far,
            reusables_detectors: self.reusables_detectors,
        }
    }

    pub fn with_reusable_detector(
        self,
        detector: Box<dyn ReusableDetector>,
    ) -> TestsTarget<Launchable> {
        let mut detectors_so_far = self.reusables_detectors;
        detectors_so_far.push(detector);
        TestsTarget::<Launchable> {
            stage: std::marker::PhantomData::<Launchable>,
            filepath: self.filepath,
            issue_detectors: self.issue_detectors,
            reusables_detectors: detectors_so_far,
        }
    }
}

pub type DetectorName = String;
pub type ContractJSONFilepath = String;
pub struct TestsConfig {
    store: HashMap<DetectorName, Vec<ContractJSONFilepath>>,
}

impl From<Vec<TestsTarget<Launchable>>> for TestsConfig {
    fn from(tests_targets: Vec<TestsTarget<Launchable>>) -> Self {
        let mut store: HashMap<DetectorName, Vec<ContractJSONFilepath>> = HashMap::default();

        tests_targets.iter().for_each(|t| {
            let filepath = t.filepath.to_string();
            t.issue_detectors.iter().for_each(|d| {
                if let Some(ptr) = store.get_mut(&d.name()) {
                    ptr.push(filepath.clone());
                } else {
                    store.insert(d.name(), vec![filepath.clone()]);
                }
            });
            t.reusables_detectors.iter().for_each(|d| {
                if let Some(ptr) = store.get_mut(&d.name()) {
                    ptr.push(filepath.clone());
                } else {
                    store.insert(d.name(), vec![filepath.clone()]);
                }
            });
        });

        Self { store }
    }
}

impl TestsConfig {
    pub fn get_contracts_for(&self, detector_id: DetectorName) -> Vec<ContractJSONFilepath> {
        self.store.get(&detector_id).unwrap_or(&vec![]).clone()
    }
}
