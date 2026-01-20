// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::{BufRead, BufReader, Write};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: Option<String>,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // If --stdio flag is passed, run as MCP server via stdio
    if args.contains(&"--stdio".to_string()) {
        run_stdio_mcp();
    } else {
        // Otherwise run the GUI app
        m3u8_mcp_lib::run()
    }
}

fn run_stdio_mcp() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if line.trim().is_empty() {
            continue;
        }

        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let error_response = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: Value::Null,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: format!("Parse error: {}", e),
                    }),
                };
                let _ = writeln!(stdout, "{}", serde_json::to_string(&error_response).unwrap());
                let _ = stdout.flush();
                continue;
            }
        };

        // Notifications (no id) don't get responses
        if request.id.is_none() {
            // Handle notification silently
            continue;
        }

        let id = request.id.unwrap();
        let response = handle_request(&request.method, request.params, id);
        let _ = writeln!(stdout, "{}", serde_json::to_string(&response).unwrap());
        let _ = stdout.flush();
    }
}

fn handle_request(method: &str, params: Option<Value>, id: Value) -> JsonRpcResponse {
    match method {
        "initialize" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "m3u8-mcp",
                    "version": "0.1.0"
                }
            })),
            error: None,
        },
        "tools/list" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "tools": [
                    {
                        "name": "m3u8_parse",
                        "description": "Parse an m3u8 playlist URL and return its structure",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "url": {
                                    "type": "string",
                                    "description": "URL of the m3u8 playlist"
                                }
                            },
                            "required": ["url"]
                        }
                    },
                    {
                        "name": "m3u8_download",
                        "description": "Download m3u8 stream to a file using FFmpeg",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "url": {
                                    "type": "string",
                                    "description": "URL of the m3u8 stream"
                                },
                                "output_path": {
                                    "type": "string",
                                    "description": "Output file path"
                                }
                            },
                            "required": ["url", "output_path"]
                        }
                    },
                    {
                        "name": "m3u8_probe",
                        "description": "Probe m3u8 stream for information (duration, codecs, etc)",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "url": {
                                    "type": "string",
                                    "description": "URL of the m3u8 stream"
                                }
                            },
                            "required": ["url"]
                        }
                    },
                    {
                        "name": "m3u8_extract_segments",
                        "description": "Extract all segment URLs from an m3u8 playlist",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "url": {
                                    "type": "string",
                                    "description": "URL of the m3u8 playlist"
                                }
                            },
                            "required": ["url"]
                        }
                    },
                    {
                        "name": "open_capture_ui",
                        "description": "Open the m3u8 capture UI to intercept streams from websites",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "url": {
                                    "type": "string",
                                    "description": "Optional URL to navigate to"
                                }
                            }
                        }
                    }
                ]
            })),
            error: None,
        },
        "tools/call" => handle_tool_call(id, params),
        "ping" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({})),
            error: None,
        },
        _ => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: format!("Method not found: {}", method),
            }),
        },
    }
}

fn handle_tool_call(id: Value, params: Option<Value>) -> JsonRpcResponse {
    let params = match params {
        Some(p) => p,
        None => {
            return JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: "Missing params".to_string(),
                }),
            };
        }
    };

    let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

    let result = match tool_name {
        "m3u8_parse" => {
            let url = arguments.get("url").and_then(|v| v.as_str()).unwrap_or("");
            match reqwest::blocking::get(url) {
                Ok(resp) => {
                    match resp.text() {
                        Ok(content) => json!({
                            "content": [{
                                "type": "text",
                                "text": format!("Parsed m3u8 from {}:\n{}", url, content)
                            }]
                        }),
                        Err(e) => json!({
                            "content": [{
                                "type": "text",
                                "text": format!("Failed to read response: {}", e)
                            }],
                            "isError": true
                        })
                    }
                }
                Err(e) => json!({
                    "content": [{
                        "type": "text",
                        "text": format!("Failed to fetch m3u8: {}", e)
                    }],
                    "isError": true
                })
            }
        }
        "m3u8_download" => {
            let url = arguments.get("url").and_then(|v| v.as_str()).unwrap_or("");
            let output = arguments.get("output_path").and_then(|v| v.as_str()).unwrap_or("output.mp4");
            
            let result = std::process::Command::new("ffmpeg")
                .args(["-y", "-i", url, "-c", "copy", "-bsf:a", "aac_adtstoasc", output])
                .output();
            
            match result {
                Ok(output_result) => {
                    if output_result.status.success() {
                        json!({
                            "content": [{
                                "type": "text",
                                "text": format!("Downloaded to: {}", output)
                            }]
                        })
                    } else {
                        json!({
                            "content": [{
                                "type": "text",
                                "text": format!("FFmpeg error: {}", String::from_utf8_lossy(&output_result.stderr))
                            }],
                            "isError": true
                        })
                    }
                }
                Err(e) => json!({
                    "content": [{
                        "type": "text",
                        "text": format!("Failed to run ffmpeg: {}", e)
                    }],
                    "isError": true
                })
            }
        }
        "m3u8_probe" => {
            let url = arguments.get("url").and_then(|v| v.as_str()).unwrap_or("");
            
            let result = std::process::Command::new("ffprobe")
                .args(["-v", "quiet", "-print_format", "json", "-show_format", "-show_streams", url])
                .output();
            
            match result {
                Ok(output) => {
                    json!({
                        "content": [{
                            "type": "text",
                            "text": String::from_utf8_lossy(&output.stdout).to_string()
                        }]
                    })
                }
                Err(e) => json!({
                    "content": [{
                        "type": "text",
                        "text": format!("Failed to probe: {}", e)
                    }],
                    "isError": true
                })
            }
        }
        "m3u8_extract_segments" => {
            let url = arguments.get("url").and_then(|v| v.as_str()).unwrap_or("");
            
            match reqwest::blocking::get(url) {
                Ok(resp) => {
                    match resp.text() {
                        Ok(content) => {
                            let segments: Vec<&str> = content
                                .lines()
                                .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
                                .collect();
                            json!({
                                "content": [{
                                    "type": "text",
                                    "text": serde_json::to_string_pretty(&segments).unwrap_or_default()
                                }]
                            })
                        }
                        Err(e) => json!({
                            "content": [{
                                "type": "text",
                                "text": format!("Failed to read: {}", e)
                            }],
                            "isError": true
                        })
                    }
                }
                Err(e) => json!({
                    "content": [{
                        "type": "text",
                        "text": format!("Failed to fetch: {}", e)
                    }],
                    "isError": true
                })
            }
        }
        "open_capture_ui" => {
            let url = arguments.get("url").and_then(|v| v.as_str());
            
            let exe_path = std::env::current_exe().unwrap_or_default();
            let mut cmd = std::process::Command::new(&exe_path);
            if let Some(u) = url {
                cmd.arg("--url").arg(u);
            }
            let _ = cmd.spawn();
            
            json!({
                "content": [{
                    "type": "text",
                    "text": "Opened m3u8 capture UI"
                }]
            })
        }
        _ => json!({
            "content": [{
                "type": "text",
                "text": format!("Unknown tool: {}", tool_name)
            }],
            "isError": true
        })
    };

    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(result),
        error: None,
    }
}
