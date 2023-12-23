use aderyn_driver::driver::{self, Args};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    /// Foundry or Hardhat project root directory (or path to single solidity file)
    root: String,

    /// Desired file path for the final report (will overwrite existing one)
    #[arg(short, long, default_value = "report.md")]
    output: String,

    /// List of path strings to exclude, delimited by comma (no spaces).
    /// Any solidity file path containing these strings will be ignored
    #[clap(short, long, use_value_delimiter = true)]
    exclude: Option<Vec<String>>,

    /// Do not include code snippets in the report (reduces report size in large repos)
    #[arg(short, long)]
    no_snippets: bool,
}

fn main() {
    let cmd_args = CommandLineArgs::parse();

    let args: Args = Args {
        root: cmd_args.root,
        output: cmd_args.output,
        exclude: cmd_args.exclude,
        no_snippets: cmd_args.no_snippets,
    };

    driver::drive(args);
}
