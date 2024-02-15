use std::{
    fmt::{self, Display},
    process::ExitCode,
};

use serde::{Deserialize, Serialize};
use strum::EnumCount;
use watchtower::lightchaser::LightChaser;
use watchtower::utils::MetricsDatabase;
use watchtower::WatchTower;

use clap::{Parser, Subcommand};

use crate::inference::MetricsChangeSummarizer;

mod extract;
mod inference;
mod utils;
mod watchtower;

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumCount, Clone)]
pub enum IssueSeverity {
    NC,
    Low,
    Medium,
    High,
    Critical,
}

impl Display for IssueSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let issue_description = match self {
            IssueSeverity::NC => "NC (Non Critical)",
            IssueSeverity::Low => "Low",
            IssueSeverity::Medium => "Medium",
            IssueSeverity::High => "High",
            IssueSeverity::Critical => "Critical",
        };
        write!(f, "{}", issue_description).unwrap();
        Ok(())
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    /// Path to database file
    #[arg(short, long, default_value = "watchtower.metrics_db.json")]
    metrics_db: String,

    #[clap(subcommand, name = "my_subcommand")]
    my_subcommand: Option<MySubcommand>,
}

#[derive(Debug, Subcommand)]
enum MySubcommand {
    /// Print a detector's metrics and rating.
    DisplayMetrics { detector_name: Option<String> },
    /// Give feedback on the aderyn report (affects detectors' ratings and metrics)
    ApplyJudgement { file: String },
}

fn main() -> ExitCode {
    let commands = CommandLineArgs::parse();

    let db = MetricsDatabase::from_path(commands.metrics_db);

    db.create_if_not_exists();

    let watchtower: Box<dyn WatchTower> = Box::new(LightChaser { metrics_db: db });

    if let Some(subcmd) = commands.my_subcommand {
        return match subcmd {
            MySubcommand::ApplyJudgement { file } => {
                // Step 1 - Register detectors not seen before
                utils::register_unseen_detectors(&watchtower, &file);

                // Step 2 - Apply the judgement
                let before_snapshot = watchtower.all_metrics();
                let exit_code = utils::apply_judgement(&watchtower, &file);
                let after_snapshot = watchtower.all_metrics();
                let change_summarizer = MetricsChangeSummarizer {
                    before_metrics: before_snapshot,
                    after_metrics: after_snapshot,
                };
                change_summarizer.print_summary_of_changes(&watchtower);
                exit_code
            }
            MySubcommand::DisplayMetrics { detector_name } => {
                utils::display_metrics(&watchtower, detector_name.as_deref())
            }
        };
    }

    println!("Try --help to see more");
    ExitCode::SUCCESS
}
