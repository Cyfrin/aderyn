/**
 *
 * WELCOME !
 *
 * FAQ
 *
 * > How to create custom detectors ?
 *      - Run `nyth new issue my_issue_name` or `nyth new reusable my_reusable_detector_name`
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
 *      - Subscriptions can include a combination of custom and core detectors as you please
 *      - Use the command `cargo run` as usual
 *
 * ADERYN-PILOT // IN MOST CASES DO NOT MODIFY THIS FILE. - Please go to `runner.rs`
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
    // These are commands are intended to be used by judging systems
    #[clap(subcommand, name = "pilot")]
    pilot: Option<PilotCommand>,
}

#[derive(Debug, Subcommand)]
enum PilotCommand {
    /// Update the metadata json file with upto date detectors
    RefreshMetadata,
    /// Make *.judge.md from custom detectors only on specified root and output
    GenerateReportForJudge {
        /// Root folder of competition's project
        root: String,
        /// Markdown file for judging path/to/*.judge.md
        output: String,
    },
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
        PilotCommand::GenerateReportForJudge { root, output } => {
            bot_brain::generate_report_for_judge(root.as_str(), output.as_str());
        }
    }
}
