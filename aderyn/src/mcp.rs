use aderyn_driver::{driver, SingletonMcpServer};
use indoc::indoc;

use rmcp::transport::{
    streamable_http_server::session::local::LocalSessionManager, StreamableHttpService,
};
use tokio::runtime::Builder;

/// Starts an MCP server with streamable HTTP transport on given port
pub fn spin_up_mcp_server(args: driver::Args, port: u16) {
    let mcp_server = driver::create_mcp_server(args).unwrap_or_else(|| {
        eprintln!("Unable to generate workspace context. Likely, issue compiling solidity files.");
        std::process::exit(1);
    });

    let async_runtime = Builder::new_multi_thread()
        .worker_threads(20)
        .thread_name("aderyn-async-runtime")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_all()
        .build()
        .expect("unable to start async runtime");

    eprintln!(
        indoc! {"
            Dear human,

            The MCP Server is now running at:

                http://127.0.0.1:{}/mcp

            You can connect using any MCP-compatible client—such as an editor, an AI agent, 
            or any other tool that supports the protocol.

            If you'd simply like to explore the available tools, you can use the free MCP 
            Inspector by running:

                npx -y @modelcontextprotocol/inspector

            in another terminal, and then enter the server URL shown above.

            ⚠️ Live reload is disabled to keep session data consistent.
            Restart the MCP server whenever you need to apply changes from updated files.
        "},
        port
    );

    async_runtime.block_on(async move {
        let mcp_server = SingletonMcpServer::new(mcp_server);

        let service = StreamableHttpService::new(
            move || Ok(mcp_server.clone()),
            LocalSessionManager::default().into(),
            Default::default(),
        );

        let router = axum::Router::new().nest_service("/mcp", service);

        let tcp_listener =
            tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

        let _ = axum::serve(tcp_listener, router).with_graceful_shutdown(shutdown_signal()).await;
    });

    // dbg!(args.input_config);
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{signal, SignalKind};
        let mut term = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        term.recv().await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    eprintln!("Signal received, shutting down gracefully...");
    std::process::exit(0);
}
