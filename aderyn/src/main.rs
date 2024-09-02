#![allow(clippy::borrowed_box)]

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use aderyn::{
    aderyn_is_currently_running_newest_version, print_all_detectors_view, print_detail_view,
};
use log::{info, warn, LevelFilter};
use notify_debouncer_full::notify::RecommendedWatcher;
use simple_logging::log_to_file;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use aderyn_driver::driver::{self, Args};

use clap::{ArgGroup, Parser, Subcommand};
use notify_debouncer_full::notify::{Config, RecursiveMode, Watcher};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("stdout_dependent").requires("stdout")))]
pub struct CommandLineArgs {
    /// Print every detector available
    #[clap(subcommand, name = "registry")]
    registry: Option<RegistryCommand>,

    /// Foundry or Hardhat project root directory (or path to single solidity file)
    #[arg(default_value = ".")]
    root: String,

    /// Path to the source contracts. If not provided, the ROOT directory will be used.
    ///
    /// For example, in a foundry repo:
    ///
    ///     --src=src/
    ///
    /// In a hardhat repo:
    ///
    ///    --src=contracts/
    #[clap(short, long, use_value_delimiter = true)]
    src: Option<Vec<String>>,

    /// List of path strings to include, delimited by comma (no spaces).
    /// Any solidity file path not containing these strings will be ignored
    #[clap(short = 'i', long, use_value_delimiter = true)]
    path_includes: Option<Vec<String>>,

    /// List of path strings to exclude, delimited by comma (no spaces).
    /// Any solidity file path containing these strings will be ignored
    #[clap(short = 'x', long, use_value_delimiter = true)]
    path_excludes: Option<Vec<String>>,

    /// Desired file path for the final report (will overwrite existing one)
    #[arg(short, long, default_value = "report.md")]
    output: String,

    /// Watch for file changes and continuously generate report
    #[arg(short, long, group = "stdout_dependent")]
    watch: bool,

    /// Do not include code snippets in the report (reduces report size in large repos)
    #[arg(long)]
    no_snippets: bool,

    /// Only use the high detectors
    #[arg(long)]
    highs_only: bool,

    /// Print the output to stdout instead of a file
    #[arg(long, name = "stdout")]
    stdout: bool,

    /// Skip contract build step
    #[arg(long)]
    skip_build: bool,

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
enum RegistryCommand {
    /// Browse detector registry
    Registry {
        /// all    - View all available detectors
        ///
        /// <name> - Detail view of a single detector
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

    let mut args: Args = Args {
        root: cmd_args.root,
        output: cmd_args.output,
        src: cmd_args.src,
        path_includes: cmd_args.path_includes,
        path_excludes: cmd_args.path_excludes,
        no_snippets: cmd_args.no_snippets,
        skip_build: cmd_args.skip_build,
        skip_cloc: cmd_args.skip_cloc,
        skip_update_check: cmd_args.skip_update_check,
        stdout: cmd_args.stdout,
        auditor_mode: cmd_args.auditor_mode,
        highs_only: cmd_args.highs_only,
    };

    // Run watcher is watch mode is engaged
    if cmd_args.watch {
        // Default to JSON
        args.output = "report.json".to_string();

        // Run it once, for the first time
        driver::drive(args.clone());

        watch_asynchronously_and_report(args);
    } else {
        driver::drive(args.clone());
    }

    // Check for updates
    if !cmd_args.skip_update_check {
        if let Ok(yes) = aderyn_is_currently_running_newest_version() {
            if !yes {
                println!();
                println!("NEW VERSION OF ADERYN AVAILABLE! Please run `cyfrinup` to upgrade.");
            }
        }
    }
}

#[derive(Debug)]
struct Backend {
    _client: Arc<Mutex<Client>>,
    // rx_arc: Arc<Mutex<Receiver<NotifyResult<Event>>>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        info!("TLSP initialize");
        info!("{:?}", params.capabilities);

        //self.client
        //    .log_message(
        //        MessageType::INFO,
        //        format!("server initialized! {:#?}", params.capabilities),
        //    )
        //    .await;
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: None,
                        will_save: Some(false),
                        will_save_wait_until: Some(false),
                        save: Some(TextDocumentSyncSaveOptions::Supported(true)),
                    },
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, params: InitializedParams) {
        info!("TLSP initialized: {:?}", params);
        //self.client
        //    .log_message(
        //        MessageType::INFO,
        //        format!("server initialized! {:?}", params),
        //    )
        //    .await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        info!("TLSP didSave caught : {:?}", params);
        // let mut res = self.rx_arc.lock().await;

        // Let's wait for upto 3 seconds to see if we receive the change
        //   if let Ok(Some(Ok(event_result))) = timeout(Duration::from_secs(3), res.recv()).await {
        // Do something
        //   }
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

//impl Backend {
//    async fn spawn_diagnostic_watcher(&self) {
//        tokio::spawn(async move {
//            let rec = Arc::clone(&self.rx_arc);
//            let _lock = rec.lock().await;
//            // do something
//        });
//    }
//}

fn watch_asynchronously_and_report(args: Args) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let (tx, rx) = tokio::sync::mpsc::channel(1);

        let config = Config::default()
            .with_poll_interval(Duration::from_secs(2))
            .with_compare_contents(false);

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                tx.blocking_send(res).unwrap();
            },
            config,
        )
        .unwrap();

        watcher
            .watch(
                PathBuf::from(args.root.clone()).as_path(),
                RecursiveMode::Recursive,
            )
            .unwrap();

        //// Then run again only if file events are observed
        //for result in rx {
        //    match result {
        //        Ok(_) => {
        //
        //            // do stuff here
        //        }
        //        Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        //    }
        //    println!();
        //}
        _ = log_to_file(
            "/Users/tilakmadichetti/Documents/OpenSource/my-first-vscode-lsp/lsp_server.log",
            LevelFilter::Info,
        );

        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();

        let (service, socket) = LspService::new(move |client| {
            let rx_arc = Arc::new(Mutex::new(rx));
            let client_arc = Arc::new(Mutex::new(client));
            let c2 = Arc::clone(&client_arc);
            tokio::spawn(async move {
                let t = Arc::clone(&rx_arc);
                let c = Arc::clone(&client_arc);
                let mut rxer = t.lock().await;
                while let Some(change) = rxer.recv().await {
                    if let Ok(_event) = change {
                        // do something
                        //

                        let a = args.clone();
                        let report_results = driver::drive_and_get_results(a);
                        let report_results_lock = report_results.lock().await;

                        let mut rcl = report_results_lock;
                        // do some

                        if let Some(_x) = &mut *rcl {
                            // do some
                        }
                        let diagnostic = Diagnostic::new_simple(
                            Range {
                                start: Position {
                                    line: 1,
                                    character: 3,
                                },
                                end: Position {
                                    line: 2,
                                    character: 20,
                                },
                            },
                            "BAD CODE".to_string(),
                        );

                        let c_lock = c.lock().await;

                        c_lock
                            .publish_diagnostics(
                                Url::parse("file:///example/example.txt").unwrap(),
                                vec![diagnostic],
                                None,
                            )
                            .await;
                    } else {
                        warn!("Error from rexr receiver");
                    }
                }
            });
            Backend {
                _client: c2,
                //  rx_arc: Arc::clone(&rx_arc),
            }
        });

        Server::new(stdin, stdout, socket).serve(service).await;
    });
}
