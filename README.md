# m3u8-mcp

MCP server for downloading HLS/m3u8 video streams. Works with Claude Desktop and Claude Code.

## What It Does

- **Parse** m3u8 playlists
- **Probe** stream metadata (duration, codecs, resolution)  
- **Download** streams via FFmpeg
- **Extract** segment URLs

## Requirements

- **FFmpeg** - `brew install ffmpeg`
- **Chrome Dev Tools MCP** (to find m3u8 URLs on web pages) - [Chrome Dev Tools MCP](https://github.com/ChromeDevTools/chrome-devtools-mcp)

---

## Install

### Option 1: Pre-built Binary (macOS)

```bash
git clone https://github.com/brookcs3/m3u8-mcp.git
cd m3u8-mcp
chmod +x m3u8-mcp
sudo cp m3u8-mcp /usr/local/bin/
```

### Option 2: Build from Source

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/brookcs3/m3u8-mcp.git
cd m3u8-mcp/src-tauri
cargo build --release
sudo cp target/release/m3u8-mcp /usr/local/bin/
```

---

## Setup

### Claude Code

```bash
claude mcp add m3u8 -- /usr/local/bin/m3u8-mcp --stdio
```

### Claude Desktop

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "m3u8": {
      "command": "/usr/local/bin/m3u8-mcp",
      "args": ["--stdio"]
    }
  }
}
```

Restart Claude Desktop after saving.

---

## Usage

```
# Direct URL:
Download https://example.com/video.m3u8 to ~/Downloads/video.mp4

# With Chrome Dev Tools MCP to find URL:
Go to this Canvas page, find the m3u8 in network requests, download it
```

---

## Tools

| Tool | Description |
|------|-------------|
| `m3u8_parse` | Parse playlist structure |
| `m3u8_probe` | Get stream info (duration, codecs) |
| `m3u8_download` | Download stream to file |
| `m3u8_extract_segments` | List segment URLs |

---

## License

MIT
