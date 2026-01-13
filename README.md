# Check Versions MCP Server

A Model Context Protocol (MCP) server that checks available versions of packages across multiple package managers.

## Features

- **Cargo (Rust)**: Check versions of crates on crates.io
- **Docker**: Check tags for images on Docker Hub
- **Helm**: Check versions of charts in repositories
- **Terraform**: Check provider and module versions on the Terraform Registry

## Usage

### Docker (Recommended)

```bash
docker run -i --rm ghcr.io/artyom-k-dev/check-versions-mcp:latest
```

### Local Development

1. Install dependencies:
   ```bash
   cargo build --release
   ```

2. Run the server:
   ```bash
   cargo run --release
   ```

## Tools Available

- `get_versions`: Get versions for a package from various package managers.
  - Arguments:
    - `package_manager`: One of "cargo", "docker", "helm", "terraform"
    - `package_name`: Name of the package/image/chart

## Configuration

No environment variables are required for this server.
