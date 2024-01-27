use aderyn_driver::{
    detector::get_all_detectors_with_ids,
    driver::{self, Args},
};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    /// Foundry or Hardhat project root directory (or path to single solidity file)
    #[arg(default_value = ".")]
    root: String,

    /// Desired file path for the final report (will overwrite existing one)
    #[arg(short, long, default_value = "report.md")]
    output: String,

    /// List of path strings to include, delimited by comma (no spaces).
    /// Any solidity file path not containing these strings will be ignored
    #[clap(short, long, use_value_delimiter = true)]
    scope: Option<Vec<String>>,

    /// List of path strings to exclude, delimited by comma (no spaces).
    /// Any solidity file path containing these strings will be ignored
    #[clap(short, long, use_value_delimiter = true)]
    exclude: Option<Vec<String>>,

    /// Do not include code snippets in the report (reduces report size in large repos)
    #[arg(short, long)]
    no_snippets: bool,

    #[clap(subcommand, name = "registry")]
    registry: Option<RegistryCommand>,
}

#[derive(Debug, Subcommand)]
enum RegistryCommand {
    /// Browse detector registry
    Registry {
        /// all - View all available detectors
        ///
        /// <id> - Detail view of a single detector
        #[arg(default_value = "all")]
        detector: String,
    },
}

fn main() {
    let cmd_args = CommandLineArgs::parse();

    if let Some(reg) = cmd_args.registry {
        match reg {
            RegistryCommand::Registry { detector } => {
                if detector == "all" {
                    print_all_detectors_view();
                } else {
                    print_detail_view(&detector);
                }
            }
        }
        return;
    }

    let args: Args = Args {
        root: cmd_args.root,
        output: cmd_args.output,
        scope: cmd_args.scope,
        exclude: cmd_args.exclude,
        no_snippets: cmd_args.no_snippets,
    };

    driver::drive(args);
}

fn print_detail_view(detector_id: &str) {
    let detectors_with_ids = get_all_detectors_with_ids();
    let detector = detectors_with_ids.get(detector_id);
    match detector {
        Some(detector) => {
            println!("\nDetector {}", detector_id);
            println!();
            println!("Title");
            println!("{}", detector.title());
            println!();
            println!("Description");
            println!("{}", detector.description());
            println!();
        }
        None => {
            println!("No detector found with ID {}.", detector_id);
        }
    }
}

fn print_all_detectors_view() {
    println!("\nDetector Registry");
    println!();
    println!("{}   {}", right_pad("ID", 30), "Title");
    println!();
    for (id, detector) in get_all_detectors_with_ids() {
        println!("{} - {}", right_pad(&id, 30), detector.title());
    }
    println!();
}

fn right_pad(s: &str, by: usize) -> String {
    if s.len() > by {
        return s.to_string();
    }
    let extra_spaces = by - s.len();
    let spaces = " ".repeat(extra_spaces);
    let mut new_string = s.to_string();
    new_string.push_str(&spaces);
    new_string
}
