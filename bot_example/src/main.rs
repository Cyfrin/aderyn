// This import is required at minimum
use aderyn_driver::driver::{self, Args};

// These imports are for subscribing to desired core detectors
use aderyn_driver::detection_modules::high::arbitrary_transfer_from::ArbitraryTransferFromDetector;
use aderyn_driver::detection_modules::low::push_0_opcode::PushZeroOpcodeDetector;
use aderyn_driver::detector::Detector;

// This import is for the custom detector `unindexed_events.rs`
use bot_example::unindexed_events::UnindexedEventsDetector;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    /// Desired directory path for outputting the final 3 reports (will overwrite existing one)
    #[arg(short, long, default_value = "bot_reports")]
    output: String,
}

fn main() {
    let cmd_args = CommandLineArgs::parse();

    let root_path = PathBuf::from("tests/contract-playground");

    //////////////////////  DEFAULT (EVERYTHING) //////////////////////////////////

    driver::drive(Args {
        root: root_path.to_str().unwrap().to_string(),
        output: format!("{}/default_analysis_report.md", cmd_args.output).to_string(),
        exclude: None,
        no_snippets: false,
        scope: None,
    });

    ////////////////////// SUBSCRIBE TO INTERESTED ONES ///////////////////////////

    let subscribe_to: Vec<Box<dyn Detector>> = vec![
        Box::<ArbitraryTransferFromDetector>::default(),
        Box::<PushZeroOpcodeDetector>::default(),
    ];

    driver::drive_with(
        // notice this is `drive_with` unlike like above
        Args {
            root: root_path.to_str().unwrap().to_string(),
            output: format!("{}/subscription_analysis_report.md", cmd_args.output).to_string(),
            exclude: None,
            no_snippets: false,
            scope: None,
        },
        subscribe_to, // inject subscriptions here
    );

    //////////////////// HYBRID (CUSTOM ONE + aderyn) /////////////////////////////

    // There is a file called `unindexed_events` - let's pretend it is the custom written
    // detector. Now we want to use that along with   `ArbitraryTransferFromDetector`

    let subscribe_to_hybrid: Vec<Box<dyn Detector>> = vec![
        Box::<ArbitraryTransferFromDetector>::default(),
        Box::<UnindexedEventsDetector>::default(),
    ];

    driver::drive_with(
        // notice this is `drive_with` unlike like the first ex.
        Args {
            root: root_path.to_str().unwrap().to_string(),
            output: format!("{}/custom_subscription_analysis_report.md", cmd_args.output)
                .to_string(),
            exclude: None,
            no_snippets: false,
            scope: None,
        },
        subscribe_to_hybrid, // inject subscriptions here
    );
}
