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
        TestsTarget::new("../TEST-CONTRACTS/out/xyz.sol/xyz.json")
            .with(Box::<Example>::default())
            .with(Box::<AnotherExample>::default())
    ]
    .into()
}
*/
