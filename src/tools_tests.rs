//! Tests for MCP tool registration and server info.

use super::*;

fn test_server() -> AscMcpServer {
    let client = Arc::new(AscClient::new(Arc::new(crate::auth::Credentials::new(
        "K".into(),
        "I".into(),
        vec![],
    ))));
    AscMcpServer::new(client)
}

#[test]
fn test_server_info() {
    let server = test_server();
    let info = server.get_info();
    assert_eq!(info.server_info.name, "asc-mcp");
    assert!(info.capabilities.tools.is_some());
}

#[test]
fn test_tool_router_has_tools() {
    let server = test_server();
    let tools = server.tool_router.list_all();
    assert_eq!(tools.len(), 11, "expected 11 tools registered");
}

#[test]
fn test_tool_names() {
    let server = test_server();
    let tools = server.tool_router.list_all();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    // CI tools
    assert!(names.contains(&"list_products"));
    assert!(names.contains(&"get_product"));
    assert!(names.contains(&"list_workflows"));
    assert!(names.contains(&"list_build_runs"));
    assert!(names.contains(&"get_build_run"));
    assert!(names.contains(&"start_build"));
    assert!(names.contains(&"list_build_actions"));
    // App tools
    assert!(names.contains(&"list_apps"));
    assert!(names.contains(&"get_app"));
    // Customer Review tools
    assert!(names.contains(&"list_customer_reviews"));
    // Sales Report tools
    assert!(names.contains(&"get_sales_report"));
}

#[test]
fn test_server_instructions_mention_new_capabilities() {
    let server = test_server();
    let info = server.get_info();
    let instructions = info.instructions.unwrap();
    assert!(instructions.contains("apps"));
    assert!(instructions.contains("reviews"));
    assert!(instructions.contains("sales"));
}
