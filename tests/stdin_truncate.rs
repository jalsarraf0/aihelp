use aihelp::prompt::{build_user_message, truncate_stdin_bytes, StdinContext};

#[test]
fn stdin_truncation_logic_is_correct() {
    let input = b"abcdefghijklmnopqrstuvwxyz";
    let (out, truncated) = truncate_stdin_bytes(input, 10);

    assert!(truncated);
    assert_eq!(out, b"abcdefghij");

    let (out2, truncated2) = truncate_stdin_bytes(input, 26);
    assert!(!truncated2);
    assert_eq!(out2, input);
}

#[test]
fn prompt_mentions_truncation_when_needed() {
    let ctx = StdinContext {
        content: "abc".to_string(),
        truncated: true,
        bytes_read: 3,
        max_bytes: 3,
    };

    let msg = build_user_message("what is this?", Some(&ctx));
    assert!(msg.contains("stdin was truncated"));
}
