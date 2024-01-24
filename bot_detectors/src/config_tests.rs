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
        TestsTarget::new("./foundry-workspace/out/Counter.sol/Counter.json")
            .with(Box::<ExampleDetector>::default())
            .with(Box::<AnotherExampleDetector>::default())
    ]
    .into()
}
*/
