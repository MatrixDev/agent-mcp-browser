# agent-mcp-browser

[<video src="example.mp4" controls width="600">
Your browser does not support the video tag.
</video>
](https://github.com/user-attachments/assets/f149562c-2ad9-4f9a-af33-35363b41d868)

Launches a Chrome instance and exposes the tabs you pick to AI agents over MCP.

The MCP server (streamable HTTP, on a random local port by default) starts together with Chrome. A terminal UI lists
open tabs and lets you choose which ones the agent may control; selection changes apply immediately, and closed tabs are
dropped automatically.

## Tools

| Tool         | Description                                            |
|--------------|--------------------------------------------------------|
| `list_tabs`  | Lists tabs with their ids, titles and URLs             |
| `screenshot` | PNG of a tab's viewport                                |
| `wait`       | Sleep for the given number of milliseconds             |
| `click`      | Left-click at viewport coordinates                     |
| `scroll`     | Scroll the page, or the element under `x,y` when given |

## Building

```sh
cargo build --release
```

The binary is written to `target/release/agent-mcp-browser`.

## Usage

```sh
agent-mcp-browser [OPTIONS]
# or
agent-mcp-browser --help
# or
agent-mcp-browser --address ::1 --port 8080
```

| Option                      | Description                  | Default          |
|-----------------------------|------------------------------|------------------|
| `-a`, `--address <ADDRESS>` | MCP server IPv4/IPv6 address | `127.0.0.1`      |
| `-p`, `--port <PORT>`       | MCP server port              | random free port |
| `-h`, `--help`              | Print help                   |                  |

The MCP server URL is shown in the tab list title. Press `c` to copy the command that starts Claude Code with this
server:

```sh
claude --mcp-config '{"mcpServers":{"mdev_browser":{"type":"http","url":"http://127.0.0.1:<port>/mcp"}}}' \
  --strict-mcp-config \
  --allowedTools mcp__mdev_browser
```

The server is passed via `--mcp-config`, so nothing is saved to the Claude Code configuration. All of its tools are
allowed without prompting, and other configured MCP servers are ignored for that session.

| Key     | Action                     |
|---------|----------------------------|
| `↑` `↓` | Previous / next tab        |
| `←` `→` | First / last tab           |
| `enter` | Toggle tab exposure        |
| `c`     | Copy Claude launch command |
| `q`     | Quit                       |

Chrome uses a persistent profile in `$TMPDIR/agent-mcp-browser`, so logins survive restarts.
