use std::collections::HashMap;

use aihelp::config::{sanitized_for_display, AppConfig, McpServerConfig};

#[test]
fn sanitizes_api_keys_and_sensitive_headers() {
    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        "Bearer super-secret".to_string(),
    );
    headers.insert("X-Trace".to_string(), "keep-me".to_string());

    let mut cfg = AppConfig {
        api_key: Some("api-key-value".to_string()),
        ..AppConfig::default()
    };
    cfg.mcp.servers.push(McpServerConfig::Http {
        label: "mcp".to_string(),
        endpoint: "http://127.0.0.1:7000/mcp".to_string(),
        allowed_tools: vec![],
        headers,
    });

    let sanitized = sanitized_for_display(&cfg);
    assert_eq!(sanitized.api_key.as_deref(), Some("***REDACTED***"));

    let headers = match &sanitized.mcp.servers[0] {
        McpServerConfig::Http { headers, .. } => headers,
        _ => panic!("expected http server"),
    };

    assert_eq!(
        headers.get("Authorization").map(String::as_str),
        Some("***REDACTED***")
    );
    assert_eq!(headers.get("X-Trace").map(String::as_str), Some("keep-me"));
}
