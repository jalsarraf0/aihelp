# aihelp

`aihelp` is a cross-platform Rust CLI that sends your question (plus optional piped stdin context) to an OpenAI-compatible LM Studio endpoint, with optional MCP tool/resource discovery.

## Install

```bash
cargo install --path .
```

## Quick Usage

Linux/macOS:

```bash
aihelp "Hello can you hear me?"
ls | aihelp "what is in this directory?"
cat script.sh | aihelp "explain what this script does and any risky commands"
aihelp --mcp "find the right tool to search my docs for X, then summarize"
```

Windows PowerShell:

```powershell
aihelp "Hello can you hear me?"
Get-ChildItem | aihelp "what is in this directory?"
Get-Content .\script.ps1 | aihelp "what does this script do?"
aihelp --mcp "find docs about topic X and summarize"
```

## First-Run MCP Prompt

On first interactive run (when `config.toml` does not exist), `aihelp` asks:

```text
Enable MCP server tools by default? (y/N):
```

Then it prints one-time guidance:

- If yes: `MCP enabled by default. To disable MCP for a single run: aihelp --no-mcp ...`
- If no: `MCP disabled by default. To enable MCP for a single run: aihelp --mcp ...`
- Always: `Override anytime with --mcp or --no-mcp.`

Non-interactive first run defaults to MCP disabled.

## Flags

Core:

- `--endpoint <URL>`
- `--api-key <KEY>`
- `--model <ID>`
- `--stream`
- `--max-stdin-bytes <N>`
- `--timeout-secs <N>`
- `--json`
- `--quiet`
- `--print-model`
- `--dry-run`

MCP:

- `--mcp`
- `--no-mcp`
- `--mcp-policy <read_only|allow_list|all>`
- `--mcp-max-tool-calls <N>`
- `--mcp-max-round-trips <N>`

## Defaults

- Endpoint: `http://192.168.50.2:1234`
- Model: `openai/gpt-oss-20b`
- Paths:
  - Linux: `~/.config/aihelp/config.toml`
  - macOS: `~/Library/Application Support/aihelp/config.toml`
  - Windows: `%APPDATA%\aihelp\config.toml`

## Config Example

```toml
endpoint = "http://192.168.50.2:1234"
model = "openai/gpt-oss-20b"
max_stdin_bytes = 200000
timeout_secs = 60

[mcp]
enabled_by_default = false
allow_policy = "read_only"
max_tool_calls = 8
max_round_trips = 6

[[mcp.servers]]
label = "mytools"
transport = "http"
endpoint = "http://127.0.0.1:7000/mcp"
allowed_tools = ["search_docs", "read_file"]
headers = { Authorization = "Bearer XYZ" }

[[mcp.servers]]
label = "internal"
transport = "stdio"
command = "node"
args = ["./path/to/mcp-server.js"]
allowed_tools = ["list_things"]
```

## Docker MCP Server Example

If your MCP server container listens on port `7000` internally:

```bash
docker run --rm -p 7000:7000 your-mcp-image
```

Then point config endpoint to `http://127.0.0.1:7000/mcp`.

## Safety Model

- `aihelp` never executes shell commands from model text.
- `aihelp` never edits local files from model text.
- MCP default allow policy is `read_only`.
- `allow_list` and `all` must be explicitly chosen.

## Troubleshooting

### LM Studio server not reachable

- Verify server is running and endpoint is correct.
- Example: `curl http://192.168.50.2:1234/v1/models`

### Model not loaded

- `aihelp` fails fast if `openai/gpt-oss-20b` is absent from `/v1/models`.
- Load that model in LM Studio or use `--model <ID>`.

### MCP server unreachable

- Check endpoint or stdio command path.
- Confirm Docker port mapping exposes host endpoint.

### MCP tool blocked by allow policy

- `read_only` blocks names with write/exec semantics.
- Use `--mcp-policy allow_list` + `allowed_tools`, or `--mcp-policy all`.

## CI/CD Runner in Docker

This repo includes a self-hosted GitHub Actions runner stack for full regression/security checks.

- Repo assets: `ops/runner/`
- Host install target: `/docker/aihelp/runner`

Install to host path:

```bash
bash ops/runner/install_to_docker_aihelp.sh
```

Then edit `/docker/aihelp/runner/.env` and start:

```bash
cd /docker/aihelp/runner
docker compose up -d
```

