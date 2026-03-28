# Architecture

## Overview

`asc-mcp` is a stdio MCP server written in Rust. It exposes Apple App Store Connect API operations as MCP tools consumable by any MCP-compatible AI client (Claude Code, Cursor, Windsurf, etc.).

```
MCP Client (Claude Code, Cursor…)
        │  stdio (MCP protocol)
        ▼
  ┌─────────────┐
  │  AscMcpServer│  tools.rs — #[tool_router] via rmcp
  └──────┬──────┘
         │
  ┌──────▼──────┐
  │  AscClient  │  client.rs + client_endpoints.rs
  │  (reqwest)  │  auth, retry, pagination
  └──────┬──────┘
         │  HTTPS / JSON:API
         ▼
  App Store Connect API v1
  api.appstoreconnect.apple.com
```

## Modules

### `auth` — JWT ES256

Generates ES256-signed JWTs per Apple's specification:

- Header: `{ "alg": "ES256", "kid": "<KEY_ID>", "typ": "JWT" }`
- Payload: `{ "iss": "<ISSUER_ID>", "iat": <now>, "exp": <now+20m>, "aud": "appstoreconnect-v1" }`

Tokens are cached inside a `Mutex<Option<CachedToken>>` and reused for 15 minutes (Apple's max is 20 minutes; 5-minute buffer for clock skew).

**Key type:** `Credentials` — created once at startup, shared via `Arc` across the client.

### `client` — HTTP Transport

Wraps `reqwest::Client` with three cross-cutting behaviors:

1. **Auth injection** — every request gets a `Bearer` token from `Credentials::token()`
2. **Rate-limit retry** — on HTTP 429, reads `Retry-After` and sleeps; retries up to 3 times before returning `ApiError::RateLimited`
3. **Pagination** — `get_all_pages()` follows `links.next` until exhausted

`get_raw()` is a separate path for non-JSON endpoints (sales reports) that returns raw bytes with a custom `Accept` header.

### `client_endpoints` — Domain Methods

Thin `impl AscClient` block keeping transport logic and domain logic separate. Each method maps 1:1 to a single ASC API endpoint:

| Method | ASC endpoint |
|---|---|
| `list_products` | `GET /ciProducts` |
| `get_product` | `GET /ciProducts/{id}` |
| `list_workflows` | `GET /ciProducts/{id}/workflows` |
| `list_build_runs` | `GET /ciWorkflows/{id}/buildRuns` |
| `get_build_run` | `GET /ciBuildRuns/{id}` |
| `start_build` | `POST /ciBuildRuns` |
| `list_build_actions` | `GET /ciBuildRuns/{id}/actions` |
| `list_apps` | `GET /apps` |
| `get_app` | `GET /apps/{id}` |
| `list_customer_reviews` | `GET /apps/{id}/customerReviews` |
| `get_sales_report` | `GET /salesReports` (gzip TSV) |

### `tools` — MCP Tool Router

Uses `rmcp`'s proc-macro API (`#[tool_router]`, `#[tool_handler]`, `#[tool]`) to declare tools as async methods. Each tool:

1. Deserializes parameters from MCP's JSON payload into a typed struct (`#[derive(Deserialize, JsonSchema)]`)
2. Calls the corresponding `AscClient` method
3. Serializes the result as pretty JSON text inside `CallToolResult`

`AscMcpServer` holds an `Arc<AscClient>` and the `ToolRouter<Self>` generated at build time.

### `models` — JSON:API Types

All types derive `Serialize + Deserialize`. The generic envelope is:

```rust
struct JsonApiResponse<T> {
    data: T,
    links: Option<PagedDocumentLinks>,
}

struct Resource<A> {
    id: String,
    resource_type: String,
    attributes: A,
}
```

Enum variants use `#[serde(other)] Unknown` to handle unknown values from Apple's API without panicking.

Sales report rows are parsed from gzip-compressed TSV using `flate2` + `csv`.

## Key Design Decisions

### Why stdio transport?

The MCP spec supports both stdio and HTTP transports. Stdio is the universal default — it requires no port management, works identically in local and container environments, and every MCP client supports it.

### Why `wiremock` for tests?

Integration tests hit a real HTTP stack (including header parsing and JSON deserialization) without requiring Apple credentials or network access. This catches serialization bugs that unit tests miss.

### Why split `client.rs` and `client_endpoints.rs`?

Keeps the transport concerns (retry, pagination, auth) isolated from the domain model. New endpoints can be added without touching the core transport logic.

### Why `Arc<Credentials>` shared between client and server?

Credentials hold the token cache. Sharing via `Arc` ensures the 15-minute cache is effective even when the client is cloned across concurrent tool calls.

## Adding Support for a New ASC Resource

1. Add model types to `src/models/` with full serde derives
2. Add endpoint method(s) to `src/client_endpoints.rs`
3. Add parameter struct + tool method to `src/tools.rs`
4. Add wiremock-based tests for both the endpoint and the tool
