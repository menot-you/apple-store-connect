# Contributing to asc-mcp

Thanks for taking the time to contribute. Here's everything you need to get started.

## Development Setup

```bash
git clone https://github.com/menot-you/apple-store-connect
cd apple-store-connect
cargo build
cargo test
```

No Apple credentials required — tests use `wiremock` for HTTP-level mocking.

## Project Layout

```
src/
├── main.rs              # Entrypoint — stdio MCP server startup
├── lib.rs               # Public crate surface
├── auth.rs              # JWT ES256 generation + 15-minute cache
├── client.rs            # HTTP transport: auth, retry, pagination
├── client_endpoints.rs  # Domain methods (CI, Apps, Reviews, Reports)
├── tools.rs             # MCP tool router via rmcp
└── models/
    ├── common.rs        # JSON:API envelope types
    ├── ci.rs            # Xcode Cloud CI types
    ├── app.rs           # App + CustomerReview types
    ├── sales.rs         # Sales report row + TSV parser
    └── scm.rs           # SCM git reference types
```

Test files live next to their source module (e.g. `auth_tests.rs` beside `auth.rs`).

## Workflow

1. Fork the repo and create a branch: `git checkout -b feat/your-feature`
2. Write tests first — this project uses red-green-refactor
3. Implement the change
4. Confirm everything passes:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```
5. Open a pull request with a clear description of what changed and why

## Adding a New Tool

Tools are defined in `src/tools.rs` using `rmcp`'s `#[tool]` macro. The pattern is:

1. Add a parameter struct with `#[derive(Debug, Deserialize, JsonSchema)]`
2. Add the endpoint method to `src/client_endpoints.rs`
3. Add the tool method in the `#[tool_router] impl AscMcpServer` block
4. Add tests in `src/tools_tests.rs`

Keep one tool per ASC API action. No tool should combine multiple API calls.

## Adding a New Endpoint

Endpoint methods go in `src/client_endpoints.rs`. Every new endpoint needs:

- A unit test in the matching `*_tests.rs` file using `wiremock`
- Corresponding model types in `src/models/` if the response shape is new

## Code Style

- No `.unwrap()` in library code — propagate with `?`
- All public items need rustdoc comments
- Files stay under ~300 lines; split when you hit that
- No God structs — one responsibility per type

## Opening Issues

Use GitHub Issues for:
- Bug reports (include the tool name, parameters, and the error you saw)
- Feature requests (which ASC API endpoint you'd like covered)
- Questions about the MCP protocol integration

## License

By contributing, you agree that your contributions will be licensed under AGPL-3.0-or-later, matching the project license.
