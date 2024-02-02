use std::process::ExitCode;

use aderyn_core::watchtower::lightchaser::LightChaser;
use aderyn_core::watchtower::utils::MetricsDatabase;
use aderyn_core::watchtower::WatchTower;

use clap::{Parser, Subcommand};

mod extract;
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

    #[clap(subcommand, name = "my_subcommand")]
    my_subcommand: Option<MySubcommand>,
}

#[derive(Debug, Subcommand)]
enum MySubcommand {
    /// Explicitly tag a detector to be reviewed manually.
    AddTag {
        detector_name: String,
        message: String,
    },
    /// Remove all explicitly assigned tags
    RemoveTags { detector_name: String },
    /// Give feedback on the tagged report for better tagging next time
    GiveFeedback { file: String },
}

fn main() -> ExitCode {
    let commands = CommandLineArgs::parse();

    let db = MetricsDatabase::from_path("watchtower.metrics_db.json".to_string());

    db.self_delete(); // Asks confirmation before deleting
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

    if let Some(subcmd) = commands.my_subcommand {
        return match subcmd {
            MySubcommand::AddTag {
                detector_name,
                message,
            } => utils::tag_detector(&watchtower, &detector_name, &message),
            MySubcommand::RemoveTags { detector_name } => {
                utils::remove_tag(&watchtower, &detector_name)
            }
            MySubcommand::GiveFeedback { file } => utils::give_feedback(&watchtower, &file),
        };
    }

    println!("Try --help to see more");
    ExitCode::SUCCESS
}
