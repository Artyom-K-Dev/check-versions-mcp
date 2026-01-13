# Check Versions MCP Server

A Model Context Protocol (MCP) server written in Rust that allows AI agents to query package version information from multiple package managers.

## Supported Package Managers

*   **Cargo** (Crates.io)
*   **Helm** (Artifact Hub)
*   **Docker** (Docker Hub)
*   **Terraform** (Terraform Registry - Providers & Modules)

## Installation & Configuration

### Prerequisites
*   Rust toolchain installed.

### Build
```bash
cargo build --release
```

### Configuration (Cursor)
Add the server to your `.cursor/mcp.json` (project specific) or your global MCP settings.

**Copy-pasteable entry:**

```json
{
  "mcpServers": {
    "check-versions-mcp": {
      "command": "/workspaces/MachineLearning/check-versions-mcp/target/release/check-versions-mcp",
      "args": []
    }
  }
}
```

## Usage

The server exposes a single unified tool: `get_versions`.

### Tool: `get_versions`

**Arguments:**
*   `package_manager`: String. One of `"cargo"`, `"helm"`, `"docker"`, `"terraform"`.
*   `package_name`: String. The identifier for the package.

## Development

1.  **Clone the repository**.
2.  **Run locally**:
    ```bash
    cargo run
    ```
    The server communicates over `stdio` using JSON-RPC 2.0.

## License

MIT
