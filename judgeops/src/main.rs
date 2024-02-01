use std::process::ExitCode;

use aderyn_core::watchtower::lightchaser::LightChaser;
use aderyn_core::watchtower::utils::MetricsDatabase;
use aderyn_core::watchtower::WatchTower;

use clap::{Parser, Subcommand};

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

    /// Optionally, explicitly tag a detector (usually done when introducing a new detector)
    #[clap(subcommand, name = "tag")]
    tag: Option<TagCommand>,
}

#[derive(Debug, Subcommand)]
enum TagCommand {
    Tag {
        detector_name: String,
        message: String,
    },
}

fn main() -> ExitCode {
    let commands = CommandLineArgs::parse();

    let db = MetricsDatabase::from_path("watchtower.metrics_db.json".to_string());

    db.self_delete();
    db.create_if_not_exists();

    let watchtower: Box<dyn WatchTower> = Box::new(LightChaser { metrics_db: db });

    if commands.suggested_changes {
        // If changes are present, exit code will be non zero (helps with GH actions)
        return watchtower.print_suggested_changes_before_init();
    }

    if commands.demanding_changes {
        // If changes are present, exit code will be non zero (helps with GH actions)
        return watchtower.print_demanding_changes_before_init();
    }

    if commands.auto_register_new_detectors {
        utils::auto_register_new_core_detectors(&watchtower);
        return ExitCode::SUCCESS;
    }

    if let Some(tag) = commands.tag {
        if let TagCommand::Tag {
            detector_name,
            message,
        } = tag
        {}
    }

    println!("Try --help to see more");
    ExitCode::SUCCESS
}
