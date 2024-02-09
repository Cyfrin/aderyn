use std::process::ExitCode;

use aderyn_core::watchtower::lightchaser::LightChaser;
use aderyn_core::watchtower::utils::MetricsDatabase;
use aderyn_core::watchtower::WatchTower;

use clap::{Parser, Subcommand};

use crate::inference::MetricsChangeSummarizer;

mod extract;
mod inference;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    /// Print demanding changes
    #[arg(short, long)]
    demanding_changes: bool,

    /// Print suggested changes
    #[arg(short, long)]
    suggested_changes: bool,

    /// Auto-register all new detectors
    #[arg(short, long)]
    auto_register_new_detectors: bool,

    /// Unregister detector that has failed expectations
    #[arg(short, long)]
    unregister_detector: Option<String>,

    /// Path to database file
    #[arg(short, long, default_value = "watchtower.metrics_db.json")]
    metrics_db: String,

    /// Reset the database file before performing any operations
    #[arg(short, long)]
    reset: bool,

    #[clap(subcommand, name = "my_subcommand")]
    my_subcommand: Option<MySubcommand>,
}

#[derive(Debug, Subcommand)]
enum MySubcommand {
    /// Print a detector's metrics and rating.
    DisplayMetrics { detector_name: String },
    /// Give feedback on the aderyn report (affects detectors' ratings and metrics)
    GiveFeedback { file: String },
    /// Explicitly tag a detector to help indicate manual review required in report.
    AddTag {
        detector_name: String,
        message: String,
    },
    /// Remove all explicitly assigned tags to a detector
    RemoveTags { detector_name: String },
}

fn main() -> ExitCode {
    let commands = CommandLineArgs::parse();

    let db = MetricsDatabase::from_path(commands.metrics_db);

    if commands.reset {
        db.self_delete();
    }
    db.create_if_not_exists();

    let watchtower: Box<dyn WatchTower> = Box::new(LightChaser { metrics_db: db });

    if commands.auto_register_new_detectors {
        utils::auto_register_new_core_detectors(&watchtower);
        return ExitCode::SUCCESS;
    }

    if commands.suggested_changes {
        // If changes are present, exit code will be non zero (helps with GH actions)
        return watchtower.print_suggested_changes_before_init();
    }

    if commands.demanding_changes {
        // If changes are present, exit code will be non zero (helps with GH actions)
        return watchtower.print_demanding_changes_before_init();
    }

    if let Some(detector_name) = commands.unregister_detector {
        // When you are "suggested" by the above command "suggested_changes" to repair a detector,
        // you will have to use this command to unregister it. Then, either
        // adjust the severity in the core detector repo or just get rid of it.
        // After tht run the command to "auto_register_new_detectors" to reflect the latest changes
        return utils::unregister_detector(&watchtower, &detector_name);
    }

    if let Some(subcmd) = commands.my_subcommand {
        return match subcmd {
            MySubcommand::AddTag {
                detector_name,
                message,
            } => utils::tag_detector(&watchtower, &detector_name, &message),
            MySubcommand::RemoveTags { detector_name } => {
                utils::remove_tag(&watchtower, &detector_name)
            }
            MySubcommand::GiveFeedback { file } => {
                let before_snapshot = watchtower.all_metrics();
                let exit_code = utils::give_feedback(&watchtower, &file);
                let after_snapshot = watchtower.all_metrics();
                let change_summarizer = MetricsChangeSummarizer {
                    before_metrics: before_snapshot,
                    after_metrics: after_snapshot,
                };
                change_summarizer.print_summary_of_changes(&watchtower);
                exit_code
            }
            MySubcommand::DisplayMetrics { detector_name } => {
                utils::display_metrics(&watchtower, &detector_name)
            }
        };
    }

    println!("Try --help to see more");
    ExitCode::SUCCESS
}
