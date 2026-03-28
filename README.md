# asc-mcp

MCP server for the Apple App Store Connect API — Xcode Cloud operations, app management, customer reviews, and sales reports.

Works with Claude Code, Cursor, Windsurf, and any other [MCP](https://modelcontextprotocol.io) client.

## Features

- **Xcode Cloud CI/CD** — list products, workflows, build runs, actions; trigger builds
- **App management** — list and inspect apps in App Store Connect
- **Customer reviews** — fetch reviews with full pagination
- **Sales reports** — download and parse gzip-compressed TSV reports
- **JWT authentication** — ES256 tokens auto-generated and cached (15-minute TTL)
- **Rate-limit handling** — automatic retry with `Retry-After` respect
- **Zero config** — three env vars and you're running

## Installation

```bash
cargo install asc-mcp
```

Or build from source:

```bash
git clone https://github.com/menot-you/asc-mcp
cd asc-mcp
cargo install --path .
```

## Configuration

Generate an API key at [App Store Connect → Users and Access → Integrations → App Store Connect API](https://appstoreconnect.apple.com/access/integrations/api).

| Variable | Description |
|---|---|
| `ASC_KEY_ID` | Key ID shown in App Store Connect |
| `ASC_ISSUER_ID` | Issuer ID shown at the top of the API keys page |
| `ASC_PRIVATE_KEY_PATH` | Path to the `.p8` file downloaded from App Store Connect |

## Usage

### Claude Code

Add to `~/.claude/claude_desktop_config.json` (or your MCP client config):

```json
{
  "mcpServers": {
    "asc-mcp": {
      "command": "asc-mcp",
      "env": {
        "ASC_KEY_ID": "YOUR_KEY_ID",
        "ASC_ISSUER_ID": "YOUR_ISSUER_ID",
        "ASC_PRIVATE_KEY_PATH": "/path/to/AuthKey_XXXX.p8"
      }
    }
  }
}
```

### Cursor / Windsurf / other MCP clients

Same JSON structure — just add it to your client's MCP server configuration.

## Available Tools

### Xcode Cloud

| Tool | Description | Parameters |
|---|---|---|
| `list_products` | List all Xcode Cloud CI products | — |
| `get_product` | Get details of a specific CI product | `product_id` |
| `list_workflows` | List workflows for a CI product | `product_id` |
| `list_build_runs` | List build runs for a workflow | `workflow_id` |
| `get_build_run` | Get details of a specific build run | `build_run_id` |
| `start_build` | Trigger a new build | `workflow_id`, `git_reference_id` |
| `list_build_actions` | List actions in a build run | `build_run_id` |

### Apps

| Tool | Description | Parameters |
|---|---|---|
| `list_apps` | List all apps in App Store Connect | — |
| `get_app` | Get details of a specific app | `app_id` |

### Reviews & Reports

| Tool | Description | Parameters |
|---|---|---|
| `list_customer_reviews` | List customer reviews for an app | `app_id` |
| `get_sales_report` | Download and parse a sales report | `vendor_number`, `report_type`, `report_sub_type`, `frequency`, `report_date` |

## Development

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
cargo doc --no-deps
```

Tests use `wiremock` for real HTTP-level mocking — no Apple credentials needed.

## License

Licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.
