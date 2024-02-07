#[allow(unused_imports)]
use crate::bot_utils::{TestsConfig, TestsTarget};

// Look at the example below

pub fn tests_configuration() -> TestsConfig {
    vec![
        // Define your targets here
    ]
    .into()
}

/*  Example

pub fn tests_configuration() -> TestsConfig {
    vec![
        TestsTarget::new("./foundry_workspace/out/Counter.sol/Counter.json")
            .with_detector(Box::<ExampleDetector>::default())
            .with_reusable_detector(Box::<ExampleReusableDetector>::default()),
    ]
    .into()
}
*/
