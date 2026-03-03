use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;
use serial_test::serial;
use tempfile::TempDir;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

#[derive(Clone)]
struct SequenceResponder {
    counter: Arc<AtomicUsize>,
}

impl Respond for SequenceResponder {
    fn respond(&self, _request: &Request) -> ResponseTemplate {
        let step = self.counter.fetch_add(1, Ordering::SeqCst);

        match step {
            0 => ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "chatcmpl-1",
                "choices": [
                    {
                        "index": 0,
                        "message": {
                            "role": "assistant",
                            "content": null,
                            "tool_calls": [
                                {
                                    "id": "call_1",
                                    "type": "function",
                                    "function": {
                                        "name": "mcp_list_tools",
                                        "arguments": "{}"
                                    }
                                }
                            ]
                        },
                        "finish_reason": "tool_calls"
                    }
                ]
            })),
            1 => ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "chatcmpl-2",
                "choices": [
                    {
                        "index": 0,
                        "message": {
                            "role": "assistant",
                            "content": null,
                            "tool_calls": [
                                {
                                    "id": "call_2",
                                    "type": "function",
                                    "function": {
                                        "name": "mcp_call_tool",
                                        "arguments": "{\"server_label\":\"noserver\",\"tool_name\":\"read_file\",\"arguments\":{}}"
                                    }
                                }
                            ]
                        },
                        "finish_reason": "tool_calls"
                    }
                ]
            })),
            _ => ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "chatcmpl-3",
                "choices": [
                    {
                        "index": 0,
                        "message": {
                            "role": "assistant",
                            "content": "Final answer after tool loop"
                        },
                        "finish_reason": "stop"
                    }
                ]
            })),
        }
    }
}

#[tokio::test]
#[serial]
async fn mcp_virtual_tool_loop_runs_to_completion() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/models"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [{"id": "openai/gpt-oss-20b"}]
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(SequenceResponder {
            counter: Arc::new(AtomicUsize::new(0)),
        })
        .mount(&server)
        .await;

    let config_dir = TempDir::new().expect("tempdir");

    cargo_bin_cmd!("aihelp")
        .env("AIHELP_CONFIG_DIR", config_dir.path())
        .env("AIHELP_NONINTERACTIVE", "1")
        .arg("--endpoint")
        .arg(server.uri())
        .arg("--mcp")
        .arg("--mcp-max-tool-calls")
        .arg("8")
        .arg("--mcp-max-round-trips")
        .arg("6")
        .arg("find tools and summarize")
        .assert()
        .success()
        .stdout(contains("Final answer after tool loop"));
}
