# agent-mcp-browser

Launches a Chrome instance and exposes the tabs you pick to AI agents over MCP.

The MCP server (streamable HTTP on a random local port) starts together with Chrome. A terminal UI lists open tabs and
lets you choose which ones the agent may control; selection changes apply immediately, and closed tabs are dropped
automatically.

## Tools

| Tool         | Description                                            |
|--------------|--------------------------------------------------------|
| `list_tabs`  | Lists tabs with their ids, titles and URLs             |
| `screenshot` | PNG of a tab's viewport                                |
| `wait`       | Sleep for the given number of milliseconds             |
| `click`      | Left-click at viewport coordinates                     |
| `scroll`     | Scroll the page, or the element under `x,y` when given |

## Usage

```sh
cargo run --release
```

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
