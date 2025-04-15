use std::path::PathBuf;

use aderyn::{
    aderyn_is_currently_running_newest_version, birdsong, create_aderyn_toml_file_at,
    initialize_niceties, lsp::spin_up_language_server, print_all_detectors_view, print_detail_view,
};
use aderyn_driver::driver::{self, kick_off_report_creation, Args, CliArgsOutputConfig};

use clap::{ArgGroup, Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("stdout_dependent").requires("stdout")))]
pub struct CommandLineArgs {
    /// Commands to initialize a config file and docs help
    #[clap(subcommand)]
    subcommand: Option<MainSubcommand>,

    /// Solidity project root directory
    #[arg(default_value = ".")]
    root: String,

    /// Path to the contracts source directory (relative to the root)
    /// By default, it is auto detected in most projects.
    #[clap(short, long, use_value_delimiter = true, verbatim_doc_comment)]
    src: Option<String>,

    /// List of path fragments to include, delimited by comma (no spaces)
    /// By default, it is auto detected.
    ///
    /// Use this to include only specified source files in the analysis:
    /// Examples:
    ///     -i src/MyContract.sol
    ///     -i src/MyContract.sol,src/MyOtherContract.sol
    #[clap(short = 'i', long, use_value_delimiter = true, verbatim_doc_comment)]
    path_includes: Option<Vec<String>>,

    /// List of path fragments to exclude, delimited by comma (no spaces)
    /// By default, it is auto detected.
    ///
    /// Use this to exclude only specified source files in the analysis:
    /// Examples:
    ///     -x src/MyContract.sol
    ///     -x src/MyContract.sol,src/MyOtherContract.sol
    #[clap(short = 'x', long, use_value_delimiter = true, verbatim_doc_comment)]
    path_excludes: Option<Vec<String>>,

    /// Desired file path for the final report
    /// Output file extension (.json/.md/.sarif) decides the format.
    ///
    /// NOTE: Allowed formats: JSON, Markdown, Sarif
    /// NOTE: Overwrites existing file if found in the same path.
    #[arg(short, long, default_value = "report.md", verbatim_doc_comment)]
    output: String,

    /// Start Aderyn's LSP server on stdout. (Must be accompanied with `--stdout`)
    #[arg(short, long, group = "stdout_dependent")]
    lsp: bool,

    /// Only use the high detectors
    #[arg(long)]
    highs_only: bool,

    /// Serialize the reports to stdout, don't write to files.
    #[arg(long, name = "stdout", hide = true)]
    stdout: bool,

    /// Skip counting number of lines of code.
    #[arg(long, hide = true)]
    skip_cloc: bool,

    /// After generating report, skip checking if a new version of Aderyn is available.
    #[arg(long)]
    skip_update_check: bool,

    /// Run in Auditor mode, which only outputs manual audit helpers
    #[arg(long, hide = true)]
    auditor_mode: bool,

    /// Do not include code snippets in the report (reduces markdown report size in large repos)
    #[arg(long, hide = true)]
    no_snippets: bool,
}

#[derive(Debug, Subcommand)]
enum MainSubcommand {
    /// Browse detector registry
    Registry {
        /// all    - View all available detectors
        ///
        /// <name> - Detail view of a single detector
        #[arg(default_value = "all", verbatim_doc_comment)]
        detector: String,
    },
    /// Initializes aderyn.toml. Required when solidity project root is not the workspace root
    Init {
        /// Optional path inside root where aderyn.toml will be created
        path: Option<String>,
    },
    /// Browse Aderyn documentation
    /// Chat with AI for help - aderyn docs "how to exclude files from scan?"
    Docs {
        /// Ask question
        question: Option<String>,
    },
}

fn main() {
    initialize_niceties();
    let cmd_args = CommandLineArgs::parse();

    if let Some(subcommand) = cmd_args.subcommand {
        match subcommand {
            MainSubcommand::Registry { detector } => {
                if detector == "all" {
                    print_all_detectors_view();
                } else {
                    print_detail_view(&detector);
                }
            }
            MainSubcommand::Init { path } => {
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
            MainSubcommand::Docs { question } => {
                let url = match question {
                    Some(question) => {
                        let encoded_question = urlencoding::encode(&question);
                        format!(
                            "https://cyfrin.gitbook.io/cyfrin-docs/aderyn-cli/readme?q={}&ask=true",
                            encoded_question
                        )
                    }
                    None => "https://cyfrin.gitbook.io/cyfrin-docs/aderyn-cli".to_string(),
                };

                // First try opening the URL in browser, if it fails just print to stdout
                if webbrowser::open(&url).is_err() {
                    println!("Visit {}", url);
                };
            }
        }

        return;
    }

    let mut args = Args {
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

    if cmd_args.auditor_mode {
        driver::kick_off_audit_mode(args.clone());
    } else {
        // Run watcher is watch mode is engaged
        if cmd_args.lsp {
            // FORCE skip cloc
            args.common_config.skip_cloc = true;
            spin_up_language_server(args);
        } else {
            kick_off_report_creation(args.clone());
        }
    }

    // Check for updates on non lsp mode
    if !cmd_args.lsp && !cmd_args.skip_update_check {
        if let Some(yes) = aderyn_is_currently_running_newest_version() {
            if !yes {
                println!();
                println!("NEW VERSION OF ADERYN AVAILABLE! Please upgrade aderyn by following the instruction here - https://github.com/cyfrin/aderyn");
                println!("NOTE: You can skip this check by passing --skip-update-check flag");
            }
        }
    }

    // Ask open source community for stars
    if !cmd_args.lsp {
        birdsong::print_last_words();
    }
}
