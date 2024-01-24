/**
 *
 * WELCOME !
 *
 * FAQ
 *
 * > How to create custom detectors ?
 *      - Run `nyth generate bot_starter_pack/src/my_detector_name`
 *      - Code it out in the newly created `my_detector/detector.rs`
 *      - Write your tests
 *      - Hook up the tests with the desired solidity json out files in `config_tests.rs`
 *      - Run `cargo test`
 *
 * > How to delete a custom detector ?
 *      - Remove the detector's folder, follow the trailing errors and rectify them
 *
 * > How to analyze a codebase and generate report ?
 *      - Head over to `runner.rs`. Inside `run()`, define your subscriptions
 *      - you could include your own detectors as well as the core ones
 *      - Run `cargo run` - This will call the run() function
 *
 * ADERYN-PILOT // DO NOT TOUCH THIS FILE. - Go to `runner.rs`
 *
 * NOTE: These other flags will be used by nyth. DO NOT MODIFY any existing
 * flags. Only if you really know what you are doing feel free to ADD new flags but by
 * any means DO NOT MODIFY / DELETE existing ones.
 *
 */
use clap::{Parser, Subcommand};
use my_bot::{bot_brain, runner};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    // These are commands that will be invoked by `nyth`.
    #[clap(subcommand, name = "pilot")]
    pilot: Option<PilotCommand>,
}

#[derive(Debug, Subcommand)]
enum PilotCommand {
    RefreshMetadata,
}

fn main() {
    let cmd_args = CommandLineArgs::parse();

    if cmd_args.pilot.is_none() {
        println!("[*] Running bot ");
        runner::run();
        return;
    }

    match cmd_args.pilot.unwrap() {
        PilotCommand::RefreshMetadata => bot_brain::refresh_metadata(),
    }
}
