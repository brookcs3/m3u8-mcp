# m3u8-mcp Dependency

Add this to your skill's requirements when you need HLS/m3u8 video downloading capabilities.

---

## For Claude Skills (SKILL.md)

Add to your skill's dependencies section:

```markdown
## Dependencies

- **m3u8-mcp** - MCP server for downloading HLS/m3u8 streams
  - Install: `brew install brookcs3/tap/m3u8-mcp`
  - Repo: https://github.com/brookcs3/m3u8-mcp
```

---

## For MCP Config (claude_desktop_config.json)

```json
{
  "mcpServers": {
    "m3u8": {
      "command": "m3u8-mcp",
      "args": ["--stdio"]
    }
  }
}
```

---

## For Claude Code

```bash
claude mcp add m3u8 --scope user -- m3u8-mcp --stdio
```

---

## Quick Install Commands

**macOS (Homebrew):**
```bash
brew install brookcs3/tap/m3u8-mcp
```

**macOS (Manual):**
```bash
git clone https://github.com/brookcs3/m3u8-mcp.git
cd m3u8-mcp && chmod +x m3u8-mcp
sudo cp m3u8-mcp /usr/local/bin/
```

**Windows:**
```cmd
git clone https://github.com/brookcs3/m3u8-mcp.git
cd m3u8-mcp\src-tauri
cargo build --release
copy target\release\m3u8-mcp.exe "C:\Program Files\m3u8-mcp\"
```

---

## Available Tools

Once installed, these tools become available:

| Tool | Description |
|------|-------------|
| `m3u8_parse` | Parse playlist structure |
| `m3u8_probe` | Get stream info (duration, codecs) |
| `m3u8_download` | Download stream to file |
| `m3u8_extract_segments` | List segment URLs |

---

## Also Requires

- **FFmpeg** - `brew install ffmpeg` (macOS) or `choco install ffmpeg` (Windows)
