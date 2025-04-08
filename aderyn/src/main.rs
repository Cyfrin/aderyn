use std::path::PathBuf;

use aderyn::{
    aderyn_is_currently_running_newest_version, create_aderyn_toml_file_at, initialize_niceties,
    lsp::spin_up_language_server, print_all_detectors_view, print_detail_view,
};
use aderyn_driver::driver::{self, Args, CliArgsOutputConfig};

use clap::{ArgGroup, Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("stdout_dependent").requires("stdout")))]
pub struct CommandLineArgs {
    /// Commands to initialize a config file for aderyn [BETA] and other utils
    #[clap(subcommand)]
    subcommand: Option<Command>,

    /// Foundry or Hardhat project root directory (or path to single solidity file)
    #[arg(default_value = ".")]
    root: String,

    /// Path to the source contracts.
    /// Used to avoid analyzing libraries, tests or scripts and focus on the contracts.
    ///
    /// In Foundry projects, it's auto-captured by foundry.toml and it's usually
    /// not necessary to provide it.
    ///
    /// In a Hardhat project:
    ///
    ///    --src=contracts/
    #[clap(short, long, use_value_delimiter = true)]
    src: Option<String>,

    /// List of path strings to include, delimited by comma (no spaces).
    ///
    /// It allows to include only one or more specific contracts in the analysis:
    ///     aderyn -i src/MyContract.sol
    ///     aderyn -i src/MyContract.sol,src/MyOtherContract.sol
    #[clap(short = 'i', long, use_value_delimiter = true)]
    path_includes: Option<Vec<String>>,

    /// List of path strings to exclude, delimited by comma (no spaces).
    ///
    /// It allows to exclude one or more specific contracts from the analysis:
    ///     aderyn -x src/MyContract.sol
    ///     aderyn -x src/MyContract.sol,src/MyOtherContract.sol
    #[clap(short = 'x', long, use_value_delimiter = true)]
    path_excludes: Option<Vec<String>>,

    /// Desired file path for the final report (will overwrite existing one)
    #[arg(short, long, default_value = "report.md")]
    output: String,

    /// [BETA] Start Aderyn's LSP server on stdout
    #[arg(short, long, group = "stdout_dependent")]
    lsp: bool,

    /// Do not include code snippets in the report (reduces report size in large repos)
    #[arg(long)]
    no_snippets: bool,

    /// Only use the high detectors
    #[arg(long)]
    highs_only: bool,

    /// Print the output to stdout instead of a file
    #[arg(long, name = "stdout")]
    stdout: bool,

    /// Skip cloc analysis (line numbers, etc.)
    #[arg(long)]
    skip_cloc: bool,

    /// Skip checking for new versions of Aderyn
    #[arg(long)]
    skip_update_check: bool,

    /// Run in Auditor mode, which only outputs manual audit helpers
    #[arg(long)]
    auditor_mode: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Browse detector registry
    Registry {
        /// all    - View all available detectors
        ///
        /// <name> - Detail view of a single detector
        #[arg(default_value = "all")]
        detector: String,
    },
    /// Initialize aderyn.toml in the root directory or in an optional subdirectory
    Init {
        /// Optional path inside root where aderyn.toml will be created
        path: Option<String>,
    },
}

fn main() {
    initialize_niceties();
    let cmd_args = CommandLineArgs::parse();

    if let Some(subcommand) = cmd_args.subcommand {
        match subcommand {
            Command::Registry { detector } => {
                if detector == "all" {
                    print_all_detectors_view();
                } else {
                    print_detail_view(&detector);
                }
            }
            Command::Init { path } => {
                let creation_path = match path {
                    Some(optional_path) => {
                        let mut target_dir = PathBuf::from(&cmd_args.root);
                        target_dir.push(optional_path);

                        let can_initialize = target_dir.exists()
                            && std::fs::metadata(&target_dir).is_ok_and(|p| p.is_dir());

                        if !can_initialize {
                            eprintln!("Failed to initialize aderyn.toml in non-existent directory");
                            std::process::exit(1);
                        }

                        target_dir.to_string_lossy().to_string()
                    }
                    None => cmd_args.root,
                };

                // Create aderyn.toml at the target directory
                create_aderyn_toml_file_at(creation_path);
            }
        }

        return;
    }

    let mut args = Args {
        auditor_mode: cmd_args.auditor_mode,
        input_config: driver::CliArgsInputConfig {
            root: cmd_args.root,
            src: cmd_args.src,
            path_excludes: cmd_args.path_excludes,
            path_includes: cmd_args.path_includes,
        },
        output_config: CliArgsOutputConfig {
            output: cmd_args.output,
            stdout: cmd_args.stdout,
            no_snippets: cmd_args.no_snippets,
        },
        common_config: driver::CliArgsCommonConfig {
            lsp: cmd_args.lsp,
            skip_cloc: cmd_args.skip_cloc,
            highs_only: cmd_args.highs_only,
        },
    };

    // Run watcher is watch mode is engaged
    if cmd_args.lsp {
        // FORCE skip cloc
        args.common_config.skip_cloc = true;
        spin_up_language_server(args);
    } else {
        driver::kick_off_report_creation(args.clone());
    }

    // Check for updates
    if !cmd_args.skip_update_check {
        if let Some(yes) = aderyn_is_currently_running_newest_version() {
            if !yes {
                println!();
                println!("NEW VERSION OF ADERYN AVAILABLE! Please upgrade aderyn by following the instruction here - https://github.com/cyfrin/aderyn");
                println!("NOTE: You can skip this check by passing --skip-update-check flag");
            }
        }
    }
}
