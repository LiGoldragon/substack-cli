mod client;
mod mcp;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn".into()),
        )
        .with_writer(std::io::stderr)
        .init();

    let api_key =
        std::env::var("SUBSTACK_API_KEY").expect("SUBSTACK_API_KEY must be set");
    let hostname =
        std::env::var("SUBSTACK_HOSTNAME").expect("SUBSTACK_HOSTNAME must be set");

    let client = Arc::new(client::Client::new(&hostname, &api_key));
    let server = mcp::Server::new(client);

    let service = rmcp::ServiceExt::serve(server, rmcp::transport::stdio()).await
        .expect("failed to start MCP server");

    let _ = service.waiting().await;
}
