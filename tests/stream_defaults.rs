use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;
use serial_test::serial;
use tempfile::TempDir;
use wiremock::matchers::{body_string_contains, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
#[serial]
async fn streaming_is_enabled_by_default() {
    let server = MockServer::start().await;
    let config_dir = TempDir::new().expect("tempdir");

    Mock::given(method("GET"))
        .and(path("/v1/models"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [{"id": "openai/gpt-oss-20b"}]
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .and(body_string_contains("\"stream\":true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "chatcmpl-default-stream",
            "choices": [
                {
                    "index": 0,
                    "message": {"role": "assistant", "content": "default streaming on"},
                    "finish_reason": "stop"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    cargo_bin_cmd!("aihelp")
        .env("AIHELP_CONFIG_DIR", config_dir.path())
        .env("AIHELP_NONINTERACTIVE", "1")
        .arg("--endpoint")
        .arg(server.uri())
        .arg("hello")
        .assert()
        .success()
        .stdout(contains("default streaming on"));
}

#[tokio::test]
#[serial]
async fn no_stream_disables_streaming_for_single_run() {
    let server = MockServer::start().await;
    let config_dir = TempDir::new().expect("tempdir");

    Mock::given(method("GET"))
        .and(path("/v1/models"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [{"id": "openai/gpt-oss-20b"}]
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .and(body_string_contains("\"stream\":false"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "chatcmpl-no-stream",
            "choices": [
                {
                    "index": 0,
                    "message": {"role": "assistant", "content": "no stream requested"},
                    "finish_reason": "stop"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    cargo_bin_cmd!("aihelp")
        .env("AIHELP_CONFIG_DIR", config_dir.path())
        .env("AIHELP_NONINTERACTIVE", "1")
        .arg("--endpoint")
        .arg(server.uri())
        .arg("--no-stream")
        .arg("hello")
        .assert()
        .success()
        .stdout(contains("no stream requested"));
}
