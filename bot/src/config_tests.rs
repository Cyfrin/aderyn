#[allow(unused_imports)]
use crate::bot_utils::{TestsConfig, TestsTarget};

// Look at the example below

pub fn tests_configuration() -> TestsConfig {
    vec![
        // Define your targets here. Example:
        // TestsTarget::new("./foundry_workspace/out/Counter.sol/Counter.json")
        //     .with_issue_detector(Box::<ExampleIssueDetector>::default())
        //     .with_reusable_detector(Box::<ExampleReusableDetector>::default()),
    ]
    .into()
}
