# m3u8-mcp

MCP server for downloading HLS/m3u8 video streams. Works with Claude Desktop and Claude Code.

## What It Does

- **Parse** m3u8 playlists
- **Probe** stream metadata (duration, codecs, resolution)  
- **Download** streams via FFmpeg
- **Extract** segment URLs

## Requirements

- **FFmpeg**
  - macOS: `brew install ffmpeg`
  - Windows: `choco install ffmpeg` or download from [ffmpeg.org](https://ffmpeg.org/download.html)
- **Chrome Dev Tools MCP** (to find m3u8 URLs on web pages) - [Chrome Dev Tools MCP](https://github.com/AcademicGuard/devtools-mcp-server)

---

## Install

### macOS (Homebrew)

```bash
brew install brookcs3/tap/m3u8-mcp
```

### macOS (Manual)

```bash
git clone https://github.com/brookcs3/m3u8-mcp.git
cd m3u8-mcp
chmod +x m3u8-mcp
sudo cp m3u8-mcp /usr/local/bin/
```

### Windows

**Build from source** (requires [Rust](https://rustup.rs/)):
```cmd
git clone https://github.com/brookcs3/m3u8-mcp.git
cd m3u8-mcp\src-tauri
cargo build --release
```

Binary will be at `target\release\m3u8-mcp.exe`

Move it somewhere permanent like `C:\Program Files\m3u8-mcp\m3u8-mcp.exe`

### Build from Source (any platform)

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/brookcs3/m3u8-mcp.git
cd m3u8-mcp/src-tauri
cargo build --release
```

---

## Setup

### Claude Code

**macOS (Homebrew):**
```bash
claude mcp add m3u8 --scope user -- m3u8-mcp --stdio
```

**macOS (Manual):**
```bash
claude mcp add m3u8 --scope user -- /usr/local/bin/m3u8-mcp --stdio
```

**Windows:**
```cmd
claude mcp add m3u8 --scope user -- "C:\Program Files\m3u8-mcp\m3u8-mcp.exe" --stdio
```

### Claude Desktop

1. Open the config file:

**macOS:**
```bash
open ~/Library/Application\ Support/Claude/claude_desktop_config.json
```

**Windows:**
```
%APPDATA%\Claude\claude_desktop_config.json
```

2. Add this inside the `"mcpServers"` section:

**macOS (Homebrew):**
```json
"m3u8": {
  "command": "m3u8-mcp",
  "args": ["--stdio"]
}
```

**macOS (Manual):**
```json
"m3u8": {
  "command": "/usr/local/bin/m3u8-mcp",
  "args": ["--stdio"]
}
```

**Windows:**
```json
"m3u8": {
  "command": "C:\\Program Files\\m3u8-mcp\\m3u8-mcp.exe",
  "args": ["--stdio"]
}
```

3. Restart Claude Desktop

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
