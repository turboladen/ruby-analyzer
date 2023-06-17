use tower_lsp::{LspService, Server};
use tracing_appender::non_blocking::WorkerGuard;

use ruby_analyzer_lsp_server::Backend;

#[tokio::main]
async fn main() {
    let _guard = setup_tracing();

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, client_socket) = LspService::new(Backend::new);
    tracing::trace!("about to start server");

    Server::new(stdin, stdout, client_socket)
        .serve(service)
        .await;
}

fn setup_tracing() -> WorkerGuard {
    // TODO: Either switch back to stdout or pick a better location.
    let file_appender = tracing_appender::rolling::RollingFileAppender::new(
        tracing_appender::rolling::Rotation::NEVER,
        "/Users/steve.loveless/Development/projects/ruby_analyzer/crates/lsp_server/",
        "ruby_analyzer.log",
    );
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_env_filter("ruby_analyzer_lsp_server=trace,ruby_analyzer_tree_sitter_parser=trace")
        .with_writer(non_blocking)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .pretty()
        .init();
    guard
}
