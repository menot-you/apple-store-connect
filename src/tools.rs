//! MCP tool definitions for App Store Connect operations.
//!
//! Each tool maps 1:1 to an [`AscClient`] method, serializing results as JSON.

use std::sync::Arc;

use rmcp::handler::server::tool::ToolRouter;
use rmcp::model::*;
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::{ServerHandler, handler::server::wrapper::Parameters, tool, tool_handler, tool_router};
use serde::Deserialize;

use crate::client::AscClient;

/// MCP server exposing App Store Connect tools.
#[derive(Clone)]
pub struct AscMcpServer {
    client: Arc<AscClient>,
    tool_router: ToolRouter<Self>,
}

impl AscMcpServer {
    /// Creates a new MCP server wrapping the given ASC client.
    pub fn new(client: Arc<AscClient>) -> Self {
        Self {
            client,
            tool_router: Self::tool_router(),
        }
    }
}

// ---------------------------------------------------------------------------
// Parameter structs — CI
// ---------------------------------------------------------------------------

/// Parameters for tools that require a product ID.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProductIdParam {
    /// The CI product ID.
    pub product_id: String,
}

/// Parameters for tools that require a workflow ID.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WorkflowIdParam {
    /// The CI workflow ID.
    pub workflow_id: String,
}

/// Parameters for tools that require a build run ID.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BuildRunIdParam {
    /// The CI build run ID.
    pub build_run_id: String,
}

/// Parameters for starting a new build.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StartBuildParam {
    /// The CI workflow ID to trigger.
    pub workflow_id: String,
    /// The SCM git reference ID (branch/tag) to build from.
    pub git_reference_id: String,
}

// ---------------------------------------------------------------------------
// Parameter structs — App / Reviews / Sales
// ---------------------------------------------------------------------------

/// Parameters for tools that require an app ID.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AppIdParam {
    /// The App Store Connect app ID.
    pub app_id: String,
}

/// Parameters for downloading a sales report.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SalesReportParam {
    /// The vendor number for the report (e.g., "85012345").
    pub vendor_number: String,
    /// Report type (e.g., "SALES", "SUBSCRIPTION", "SUBSCRIPTION_EVENT").
    pub report_type: String,
    /// Report sub-type (e.g., "SUMMARY", "DETAILED", "OPT_IN").
    pub report_sub_type: String,
    /// Report frequency (e.g., "DAILY", "WEEKLY", "MONTHLY", "YEARLY").
    pub frequency: String,
    /// Report date in YYYY-MM-DD format.
    pub report_date: String,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Converts a serializable value to MCP `CallToolResult` JSON content.
fn json_result<T: serde::Serialize>(value: &T) -> Result<CallToolResult, rmcp::ErrorData> {
    let text = serde_json::to_string_pretty(value)
        .map_err(|e| rmcp::ErrorData::internal_error(format!("serialization error: {e}"), None))?;
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

/// Converts an API error to an MCP error.
fn api_err(e: crate::client::ApiError) -> rmcp::ErrorData {
    rmcp::ErrorData::internal_error(e.to_string(), None)
}

// ---------------------------------------------------------------------------
// Tool router
// ---------------------------------------------------------------------------

#[tool_router]
impl AscMcpServer {
    // -- CI tools ----------------------------------------------------------

    /// List all Xcode Cloud CI products.
    #[tool(description = "List all Xcode Cloud CI products")]
    async fn list_products(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        let resp = self.client.list_products().await.map_err(api_err)?;
        json_result(&resp)
    }

    /// Get details of a specific CI product.
    #[tool(description = "Get details of a specific CI product")]
    async fn get_product(
        &self,
        params: Parameters<ProductIdParam>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let resp = self
            .client
            .get_product(&params.0.product_id)
            .await
            .map_err(api_err)?;
        json_result(&resp)
    }

    /// List workflows for a CI product.
    #[tool(description = "List workflows for a CI product")]
    async fn list_workflows(
        &self,
        params: Parameters<ProductIdParam>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let resp = self
            .client
            .list_workflows(&params.0.product_id)
            .await
            .map_err(api_err)?;
        json_result(&resp)
    }

    /// List build runs for a workflow.
    #[tool(description = "List build runs for a workflow")]
    async fn list_build_runs(
        &self,
        params: Parameters<WorkflowIdParam>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let resp = self
            .client
            .list_build_runs(&params.0.workflow_id)
            .await
            .map_err(api_err)?;
        json_result(&resp)
    }

    /// Get details of a specific build run.
    #[tool(description = "Get details of a specific build run")]
    async fn get_build_run(
        &self,
        params: Parameters<BuildRunIdParam>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let resp = self
            .client
            .get_build_run(&params.0.build_run_id)
            .await
            .map_err(api_err)?;
        json_result(&resp)
    }

    /// Start a new build for a workflow.
    #[tool(description = "Start a new build for a workflow")]
    async fn start_build(
        &self,
        params: Parameters<StartBuildParam>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let resp = self
            .client
            .start_build(&params.0.workflow_id, &params.0.git_reference_id)
            .await
            .map_err(api_err)?;
        json_result(&resp)
    }

    /// List actions in a build run.
    #[tool(description = "List actions in a build run")]
    async fn list_build_actions(
        &self,
        params: Parameters<BuildRunIdParam>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let resp = self
            .client
            .list_build_actions(&params.0.build_run_id)
            .await
            .map_err(api_err)?;
        json_result(&resp)
    }

    // -- App tools ---------------------------------------------------------

    /// List all apps in App Store Connect.
    #[tool(description = "List all apps in App Store Connect")]
    async fn list_apps(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        let resp = self.client.list_apps().await.map_err(api_err)?;
        json_result(&resp)
    }

    /// Get details of a specific app.
    #[tool(description = "Get details of a specific app")]
    async fn get_app(
        &self,
        params: Parameters<AppIdParam>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let resp = self
            .client
            .get_app(&params.0.app_id)
            .await
            .map_err(api_err)?;
        json_result(&resp)
    }

    // -- Customer Review tools ---------------------------------------------

    /// List customer reviews for an app.
    #[tool(description = "List customer reviews for an app")]
    async fn list_customer_reviews(
        &self,
        params: Parameters<AppIdParam>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let resp = self
            .client
            .list_customer_reviews(&params.0.app_id)
            .await
            .map_err(api_err)?;
        json_result(&resp)
    }

    // -- Sales Report tools ------------------------------------------------

    /// Download and parse a sales report.
    #[tool(description = "Download and parse a sales report")]
    async fn get_sales_report(
        &self,
        params: Parameters<SalesReportParam>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let p = &params.0;
        let rows = self
            .client
            .get_sales_report(
                &p.vendor_number,
                &p.report_type,
                &p.report_sub_type,
                &p.frequency,
                &p.report_date,
            )
            .await
            .map_err(api_err)?;
        json_result(&rows)
    }
}

// ---------------------------------------------------------------------------
// Server handler
// ---------------------------------------------------------------------------

#[tool_handler]
impl ServerHandler for AscMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools_with(ToolsCapability { list_changed: None })
                .build(),
        )
        .with_server_info(Implementation::new("asc-mcp", env!("CARGO_PKG_VERSION")))
        .with_protocol_version(ProtocolVersion::V_2024_11_05)
        .with_instructions(
            "MCP server for Apple App Store Connect API. \
             Provides tools for Xcode Cloud CI (products, workflows, build runs, actions), \
             App Store apps, customer reviews, and sales reports.",
        )
    }
}

#[cfg(test)]
#[path = "tools_tests.rs"]
mod tests;
