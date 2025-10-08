use aderyn::{
    aderyn_is_currently_running_newest_version, birdsong,
    completions::SupportedShellsForCompletions,
    create_aderyn_toml_file_at, initialize_niceties,
    lsp::spin_up_language_server,
    mcp::{spin_up_http_stream_mcp_server, spin_up_stdio_mcp_server},
    print_all_detectors_view, print_detail_view,
};
use aderyn_driver::driver::{self, Args, kick_off_report_creation};
use clap::{ArgGroup, CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::{Shell, generate};
use indoc::indoc;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = indoc!{
        r#"Aderyn - Rust based Solidity Static analyzer.

        Quickstart:
        cd my-solidity-project/
        aderyn

        It outputs report.md if the solidity project is foundry/hardhat/soldeer.

        In the case that it's not, it's important to create a config file via the
        command `aderyn init` in the workspace root.

        For more examples, visit docs: https://cyfrin.gitbook.io/cyfrin-docs/aderyn-cli
        Also ask questions via command line: `aderyn docs "how to configure scan options?"`

        Help Aderyn stay open source by giving us a star on Github.
        Repository: https://github.com/cyfrin/aderyn
    "#},
    group(ArgGroup::new("stdout_dependent").requires("stdout")),
)]
pub struct CommandLineArgs {
    /// Commands to initialize a config file and docs help
    #[clap(subcommand)]
    subcommand: Option<MainSubcommand>,

    /// Solidity project root directory
    #[arg(default_value = ".", value_hint = ValueHint::DirPath)]
    root: String,

    /// Path to the contracts source directory (relative to the root)
    /// By default, it is auto detected in most projects.
    #[arg(short, long, use_value_delimiter = true, verbatim_doc_comment, value_hint =  ValueHint::DirPath)]
    src: Option<String>,

    /// List of path fragments to include, delimited by comma (no spaces)
    /// By default, it is auto detected.
    ///
    /// Use this to include only specified source files in the analysis:
    /// Examples:
    ///     -i src/MyContract.sol
    ///     -i src/MyContract.sol,src/MyOtherContract.sol
    #[arg(short = 'i', long, use_value_delimiter = true, verbatim_doc_comment, value_hint = ValueHint::Other)]
    path_includes: Option<Vec<String>>,

    /// List of path fragments to exclude, delimited by comma (no spaces)
    /// By default, it is auto detected.
    ///
    /// Use this to exclude only specified source files in the analysis:
    /// Examples:
    ///     -x src/MyContract.sol
    ///     -x src/MyContract.sol,src/MyOtherContract.sol
    #[arg(short = 'x', long, use_value_delimiter = true, verbatim_doc_comment, value_hint = ValueHint::Other)]
    path_excludes: Option<Vec<String>>,

    /// Desired file path for the final report
    /// Output file extension (.json/.md/.sarif) decides the format.
    ///
    /// NOTE: Allowed formats: JSON, Markdown, Sarif
    /// NOTE: Overwrites existing file if found in the same path.
    #[arg(short, long, default_value = "report.md", verbatim_doc_comment, value_hint = ValueHint::FilePath)]
    output: String,

    /// Start Aderyn's LSP server on stdout. (Must be accompanied with `--stdout`)
    #[arg(short, long, group = "stdout_dependent")]
    lsp: bool,

    /// Only use the high detectors
    #[arg(long)]
    highs_only: bool,

    /// After generating report, skip checking if a new version of Aderyn is available.
    #[arg(long)]
    skip_update_check: bool,

    // ---------- Hidden arguments --------------- //
    /// Serialize the reports to stdout, don't write to files.
    #[arg(long, name = "stdout", hide = true)]
    stdout: bool,

    /// Skip counting number of lines of code.
    #[arg(long, hide = true)]
    skip_cloc: bool,

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
    /// Generate shell completion scripts
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: SupportedShellsForCompletions,
    },
    /// Initializes aderyn.toml. Required when solidity project root is not the workspace root
    Init {
        /// Optional path inside root where aderyn.toml will be created
        #[arg(value_hint = ValueHint::DirPath)]
        path: Option<String>,
    },
    /// Browse Aderyn documentation
    /// Chat with AI for help - aderyn docs "how to exclude files from scan?"
    Docs {
        /// Ask question
        question: Option<String>,
    },
    /// ⚠️ [BETA] Start an MCP server in the project root
    Mcp {
        #[command(subcommand)]
        transport: McpTransport,
    },
}

#[derive(Debug, Subcommand)]
enum McpTransport {
    /// Run MCP server over streamable HTTP
    HttpStream {
        /// Port to bind the MCP server on (defaults to 6277)
        #[arg(long, default_value_t = 6277)]
        port: u16,
    },
    /// Run MCP server over STDIO
    Stdio,
}

fn main() {
    initialize_niceties();
    let cmd_args = CommandLineArgs::parse();

    // Condense args
    let mut args = Args {
        input_config: driver::CliArgsInputConfig {
            root: cmd_args.root.clone(),
            src: cmd_args.src,
            path_excludes: cmd_args.path_excludes,
            path_includes: cmd_args.path_includes,
        },
        output_config: driver::CliArgsOutputConfig {
            output: cmd_args.output,
            stdout: cmd_args.stdout,
            no_snippets: cmd_args.no_snippets,
        },
        common_config: driver::CliArgsCommonConfig {
            verbose: {
                let is_running_lsp = cmd_args.lsp;
                let is_running_mcp = cmd_args
                    .subcommand
                    .as_ref()
                    .is_some_and(|s| matches!(s, MainSubcommand::Mcp { transport: _ }));
                // In neither of those 2 cases, should aderyn be verbose enough to print metadata.
                !(is_running_lsp || is_running_mcp)
            },
            lsp: cmd_args.lsp,
            skip_cloc: cmd_args.skip_cloc,
            highs_only: cmd_args.highs_only,
        },
    };

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
            MainSubcommand::Mcp { transport } => {
                // FORCE skip cloc
                args.common_config.skip_cloc = true;
                match transport {
                    McpTransport::HttpStream { port } => {
                        spin_up_http_stream_mcp_server(args, port);
                    }
                    McpTransport::Stdio => {
                        spin_up_stdio_mcp_server(args);
                    }
                }
            }
            MainSubcommand::Completions { shell } => {
                let mut cmd = CommandLineArgs::command();
                let name = cmd.get_name().to_string();
                let clap_shell: Shell = shell.into();
                generate(clap_shell, &mut cmd, name, &mut std::io::stdout());
                return;
            }
        }

        return;
    }

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
    if !cmd_args.lsp
        && !cmd_args.skip_update_check
        && let Some(yes) = aderyn_is_currently_running_newest_version()
        && !yes
    {
        println!();
        println!(
            "NEW VERSION OF ADERYN AVAILABLE! Please upgrade aderyn by following the instruction here - https://github.com/cyfrin/aderyn"
        );
        println!("NOTE: You can skip this check by passing --skip-update-check flag");
    }

    // Ask open source community for stars
    if !cmd_args.lsp {
        birdsong::print_last_words();
    }
}
