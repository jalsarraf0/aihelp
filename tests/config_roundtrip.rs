use serial_test::serial;
use tempfile::TempDir;

use aihelp::config::{
    config_file_path, load_config, load_or_init_config, save_config, AppConfig, McpAllowPolicy,
    McpConfig,
};

#[test]
#[serial]
fn config_load_save_roundtrip() {
    let temp = TempDir::new().expect("tempdir");
    std::env::set_var("AIHELP_CONFIG_DIR", temp.path());

    let cfg = AppConfig {
        endpoint: "http://localhost:9999".to_string(),
        model: "openai/gpt-oss-20b".to_string(),
        mcp: McpConfig {
            enabled_by_default: true,
            allow_policy: McpAllowPolicy::AllowList,
            ..McpConfig::default()
        },
        ..AppConfig::default()
    };

    let path = config_file_path().expect("path");
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    save_config(&path, &cfg).expect("save");

    let loaded = load_config(&path).expect("load");
    assert_eq!(loaded.endpoint, cfg.endpoint);
    assert_eq!(loaded.model, cfg.model);
    assert!(loaded.mcp.enabled_by_default);
    assert_eq!(loaded.mcp.allow_policy, McpAllowPolicy::AllowList);

    std::env::remove_var("AIHELP_CONFIG_DIR");
}

#[test]
#[serial]
fn noninteractive_first_run_creates_safe_default() {
    let temp = TempDir::new().expect("tempdir");
    std::env::set_var("AIHELP_CONFIG_DIR", temp.path());

    let cfg = load_or_init_config(false, true).expect("init config");
    assert!(!cfg.mcp.enabled_by_default);

    let path = config_file_path().expect("path");
    assert!(path.exists());

    std::env::remove_var("AIHELP_CONFIG_DIR");
}
