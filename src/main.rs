//! Entrypoint for the asc-mcp stdio MCP server.

use std::sync::Arc;

use asc_mcp::auth::Credentials;
use asc_mcp::client::AscClient;
use asc_mcp::tools::AscMcpServer;
use rmcp::service::ServiceExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Tracing goes to stderr so stdout stays clean for MCP protocol.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("starting asc-mcp server");

    let credentials = Credentials::from_env().map_err(|e| {
        tracing::error!("failed to load credentials: {e}");
        e
    })?;

    let client = Arc::new(AscClient::new(Arc::new(credentials)));
    let server = AscMcpServer::new(client);
    let transport = rmcp::transport::io::stdio();

    let service = server.serve(transport).await?;
    service.waiting().await?;

    Ok(())
}
