use std::collections::HashMap;

use aderyn_core::watchtower::lightchaser::LightChaser;
use aderyn_core::watchtower::utils::MetricsDatabase;
use aderyn_core::watchtower::WatchTower;

use clap::Parser;

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
}

fn main() {
    let commands = CommandLineArgs::parse();

    let db = MetricsDatabase {
        metrics: HashMap::new(),
        db_path: "lc-metrics.json".to_string(),
    };

    db.self_delete();
    db.create_if_not_exists();

    let watchtower: Box<dyn WatchTower> = Box::new(LightChaser { metrics_db: db });

    if commands.suggested_changes {
        watchtower.print_suggested_changes_before_init();
    } else if commands.demanding_changes {
        watchtower.print_demanding_changes_before_init();
    } else if commands.auto_register_new_detectors {
        utils::auto_register_new_core_detectors(&watchtower);
    } else {
        println!("Try --help to see more");
    }
}
