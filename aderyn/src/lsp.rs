use log::{info, warn};
use notify_debouncer_full::notify::{Event, RecommendedWatcher, Result as NotifyResult};
use std::{collections::HashSet, path::PathBuf, sync::Arc, time::Duration};
use tokio::{
    runtime::Builder,
    sync::{mpsc::Receiver, Mutex},
};
use tower_lsp::{
    jsonrpc::Result, lsp_types::*, Client, ClientSocket, LanguageServer, LspService, Server,
};

use aderyn_driver::driver::{self, Args};

use notify_debouncer_full::notify::{Config, RecursiveMode, Watcher};

#[derive(Debug)]
struct LanguageServerBackend {
    client: Arc<Mutex<Client>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for LanguageServerBackend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        info!("TLSP initialize: {:?}", params.capabilities);

        let code_editor = self.client.lock().await;
        code_editor
            .log_message(MessageType::INFO, "Aderyn LSP received an initialization request!")
            .await;

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(false),
                        change: None,
                        will_save: Some(false),
                        will_save_wait_until: Some(false),
                        save: None,
                    },
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, params: InitializedParams) {
        info!("TLSP initialized: {:?}", params);

        let code_editor = self.client.lock().await;
        code_editor
            .log_message(
                MessageType::INFO,
                "Aderyn LSP has been notified that the edtior's LSP client is initialized.",
            )
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        info!("TLSP shutdown");

        let code_editor = self.client.lock().await;
        code_editor.log_message(MessageType::INFO, "Aderyn LSP has been shutdown").await;
        Ok(())
    }
}

pub fn spin_up_language_server(args: Args) {
    // Setup the logging file
    // NOTE: Uncomment this for debugging purposes
    //_ = log_to_file(
    //    "/Users/tilakmadichetti/Documents/OpenSource/my-first-vscode-lsp/lsp_server.log",
    //    LevelFilter::Info,
    //);

    // Create tokio runtime to run futures

    let async_runtime = Builder::new_multi_thread()
        .worker_threads(20)
        .thread_name("aderyn-async-runtime")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_all()
        .build()
        .expect("unable to start async runtime");

    // Block on this function
    async_runtime.block_on(async {
        // Channel to communicate file system changes (triggered when files are added, removed, or
        // changed)
        let (tx_file_change_event, rx_file_change_event) = tokio::sync::mpsc::channel(10);

        // Create the async watcher
        let mut file_system_watcher = RecommendedWatcher::new(
            move |res| {
                tx_file_change_event
                    .blocking_send(res)
                    .expect("unable to notify file rx_file_change_event receiver");
            },
            Config::default()
                .with_poll_interval(Duration::from_millis(20))
                .with_compare_contents(false),
        )
        .expect("couldn't create file system watcher");

        // Watch for file changes
        file_system_watcher
            .watch(
                PathBuf::from(args.input_config.root.clone()).as_path(),
                RecursiveMode::Recursive,
            )
            .expect("unable to watch for file changes");

        // Most editor's LSP clients communicate through stdout/stdin channels. Theefore use
        // a log file to send debugging statements. Please note EVERY BYTE FLOWING IN
        // STDOUT/STDIN MUST FOLLOW THE LSP PROTOCOL.
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        let (service, socket) =
            create_lsp_service_and_react_to_file_event(rx_file_change_event, args);

        // This loop will run until the client issues a shutdown request to our LSP server
        Server::new(stdin, stdout, socket).serve(service).await;
    });
}

/// Perform 2 things in parallel
/// 1. React to file changes by regenerating diagnostics
/// 2. Talk to the LSP client and push only the required diagnostics
fn create_lsp_service_and_react_to_file_event(
    rx_file_change_event: Receiver<NotifyResult<Event>>,
    args: Args,
) -> (LspService<LanguageServerBackend>, ClientSocket) {
    let (service, socket) = LspService::new(move |client| {
        // Guard the receiver and the client so we can send them across threads
        let guarded_client = Arc::new(Mutex::new(client));
        let guarded_file_change_event_receiver = Arc::new(Mutex::new(rx_file_change_event));

        // Clone the guarded client (but it doesn't actually clone, it just clones the reference).
        // Do not use `.clone()` but rather `Arc::clone()` to make this clear.
        let guarded_client_clone = Arc::clone(&guarded_client);
        let guarded_file_change_event_receiver_clone =
            Arc::clone(&guarded_file_change_event_receiver);

        // Create a guard that keeps track of all the files to which diagnostics were sent in the
        // last run. This is because LSP spec expects to manually push an empty vec![] to clean up
        // stale diagnostics. So we every time, we generate a report, we should check to see if
        // there has been a file left out that previously had errors, if so, then clean it.
        let seen_file_uris: Arc<Mutex<HashSet<Url>>> = Arc::new(Mutex::new(HashSet::new()));

        async fn generate_diagnostics_and_publish(
            args: &Args,
            guarded_client: Arc<Mutex<Client>>,
            seen_file_uris: Arc<Mutex<HashSet<Url>>>,
        ) {
            // Generate diagnostics due to this change
            let guarded_report_results = driver::fetch_report_for_lsp(args.clone());

            // Extract report from the mutex
            let mut diagnostics_mutex = guarded_report_results.lock().await;

            let Some(diagnostics_report) = &mut *diagnostics_mutex else {
                warn!("no diagnostics report generated");
                return;
            };

            info!("sending diagnostics to client {:?}", &diagnostics_report.diagnostics);
            let client_mutex = guarded_client.lock().await;

            for (file_uri, file_diagnostics) in &diagnostics_report.diagnostics {
                client_mutex
                    .publish_diagnostics(file_uri.clone(), file_diagnostics.to_vec(), None)
                    .await;
            }

            // Clear out the diagnostics for file which had reported errors before
            let current_run_file_uris =
                diagnostics_report.diagnostics.keys().cloned().collect::<HashSet<_>>();

            let mut seen_file_uris_mutex = seen_file_uris.lock().await;
            let seen_file_uris = &mut *seen_file_uris_mutex;

            for seen_file_uri in seen_file_uris.iter() {
                if !&current_run_file_uris.contains(seen_file_uri) {
                    // Clear the diagnostics for this seen file uri
                    // It had errors in the past, but not any more
                    client_mutex.publish_diagnostics(seen_file_uri.clone(), vec![], None).await;
                }
            }

            // Now, update the seen_files with files reported in the current run
            for current_run_file_uri in current_run_file_uris {
                seen_file_uris.insert(current_run_file_uri);
            }
        }

        tokio::spawn(async move {
            // For the first time, run it automatically
            let new_guarded_clone = Arc::clone(&guarded_client);
            let seen_files_uris_clone = Arc::clone(&seen_file_uris);
            generate_diagnostics_and_publish(&args, new_guarded_clone, seen_files_uris_clone).await;

            // After that, run it only when you receive file change events from the system
            let mut rxer = guarded_file_change_event_receiver_clone.lock().await;

            while let Some(rxer_change) = rxer.recv().await {
                if rxer_change.is_ok() {
                    info!("rxer change detected");

                    let new_guarded_clone = Arc::clone(&guarded_client);
                    let seen_files_uris_clone = Arc::clone(&seen_file_uris);
                    generate_diagnostics_and_publish(
                        &args,
                        new_guarded_clone,
                        seen_files_uris_clone,
                    )
                    .await;
                } else {
                    warn!("rxer change errored!");
                }
            }
        });

        LanguageServerBackend { client: guarded_client_clone }
    });
    (service, socket)
}
