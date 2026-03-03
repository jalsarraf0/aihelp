pub mod agent;
pub mod client;
pub mod config;
pub mod mcp;
pub mod prompt;

use std::io::IsTerminal;

use anyhow::{Context, Result};
use clap::Parser;
use tracing_subscriber::EnvFilter;

use crate::agent::{run_agent, AgentRunOptions};
use crate::client::OpenAiClient;
use crate::config::{AppConfig, McpAllowPolicy};
use crate::mcp::RmcpBackend;
use crate::prompt::read_stdin_context;

#[derive(Debug, Parser, Clone)]
#[command(
    name = "aihelp",
    version,
    about = "CLI helper for LM Studio + optional MCP tools"
)]
pub struct Cli {
    #[arg(value_name = "QUESTION", required = true)]
    pub question: Vec<String>,

    #[arg(long)]
    pub endpoint: Option<String>,

    #[arg(long = "api-key")]
    pub api_key: Option<String>,

    #[arg(long)]
    pub model: Option<String>,

    #[arg(long)]
    pub stream: bool,

    #[arg(long = "max-stdin-bytes")]
    pub max_stdin_bytes: Option<usize>,

    #[arg(long = "timeout-secs")]
    pub timeout_secs: Option<u64>,

    #[arg(long)]
    pub json: bool,

    #[arg(long)]
    pub quiet: bool,

    #[arg(long = "print-model")]
    pub print_model: bool,

    #[arg(long = "dry-run")]
    pub dry_run: bool,

    #[arg(long, conflicts_with = "no_mcp")]
    pub mcp: bool,

    #[arg(long = "no-mcp", conflicts_with = "mcp")]
    pub no_mcp: bool,

    #[arg(long = "mcp-policy")]
    pub mcp_policy: Option<McpAllowPolicy>,

    #[arg(long = "mcp-max-tool-calls")]
    pub mcp_max_tool_calls: Option<usize>,

    #[arg(long = "mcp-max-round-trips")]
    pub mcp_max_round_trips: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct EffectiveSettings {
    pub endpoint: String,
    pub api_key: String,
    pub model: String,
    pub max_stdin_bytes: usize,
    pub timeout_secs: u64,
    pub mcp_enabled: bool,
    pub mcp_policy: McpAllowPolicy,
    pub mcp_max_tool_calls: usize,
    pub mcp_max_round_trips: usize,
    pub stream: bool,
    pub json: bool,
    pub quiet: bool,
    pub print_model: bool,
    pub dry_run: bool,
}

pub async fn run(cli: Cli) -> Result<()> {
    init_tracing(cli.quiet);

    let noninteractive_forced = std::env::var("AIHELP_NONINTERACTIVE")
        .map(|v| v == "1")
        .unwrap_or(false);

    let stdin_is_tty = std::io::stdin().is_terminal();
    let stdout_is_tty = std::io::stdout().is_terminal();
    let interactive = stdin_is_tty && stdout_is_tty;

    let config = config::load_or_init_config(interactive, noninteractive_forced)
        .context("failed to load configuration")?;

    let settings = resolve_settings(&cli, &config);

    if settings.print_model && !settings.quiet {
        eprintln!("model: {}", settings.model);
    }

    let stdin_context = read_stdin_context(settings.max_stdin_bytes)?;
    let question = cli.question.join(" ");

    let client = OpenAiClient::new(
        settings.endpoint.clone(),
        settings.api_key.clone(),
        settings.timeout_secs,
    )?;

    if !settings.dry_run {
        client
            .verify_model_presence(&settings.model)
            .await
            .context("model verification failed")?;
    }

    let mcp_backend = if settings.mcp_enabled {
        Some(
            RmcpBackend::connect(
                config.mcp.servers.clone(),
                settings.mcp_policy,
                settings.quiet,
            )
            .await
            .context("failed to initialize MCP backend")?,
        )
    } else {
        None
    };

    let agent_opts = AgentRunOptions {
        model: settings.model.clone(),
        stream: settings.stream,
        json: settings.json,
        dry_run: settings.dry_run,
        quiet: settings.quiet,
        mcp_enabled: settings.mcp_enabled,
        mcp_max_tool_calls: settings.mcp_max_tool_calls,
        mcp_max_round_trips: settings.mcp_max_round_trips,
    };

    run_agent(
        &client,
        mcp_backend
            .as_ref()
            .map(|b| b as &dyn crate::mcp::McpBackend),
        &question,
        stdin_context.as_ref(),
        &agent_opts,
    )
    .await
}

fn resolve_settings(cli: &Cli, config: &AppConfig) -> EffectiveSettings {
    let endpoint = cli
        .endpoint
        .clone()
        .unwrap_or_else(|| config.endpoint.clone());
    let api_key = cli
        .api_key
        .clone()
        .unwrap_or_else(|| config.api_key.clone().unwrap_or_default());
    let model = cli.model.clone().unwrap_or_else(|| config.model.clone());
    let max_stdin_bytes = cli.max_stdin_bytes.unwrap_or(config.max_stdin_bytes);
    let timeout_secs = cli.timeout_secs.unwrap_or(config.timeout_secs);

    let mcp_enabled = if cli.mcp {
        true
    } else if cli.no_mcp {
        false
    } else {
        config.mcp.enabled_by_default
    };

    let mcp_policy = cli.mcp_policy.unwrap_or(config.mcp.allow_policy);
    let mcp_max_tool_calls = cli
        .mcp_max_tool_calls
        .unwrap_or(config.mcp.max_tool_calls.max(1));
    let mcp_max_round_trips = cli
        .mcp_max_round_trips
        .unwrap_or(config.mcp.max_round_trips.max(1));

    EffectiveSettings {
        endpoint,
        api_key,
        model,
        max_stdin_bytes,
        timeout_secs,
        mcp_enabled,
        mcp_policy,
        mcp_max_tool_calls,
        mcp_max_round_trips,
        stream: cli.stream,
        json: cli.json,
        quiet: cli.quiet,
        print_model: cli.print_model,
        dry_run: cli.dry_run,
    }
}

fn init_tracing(quiet: bool) {
    let filter = if quiet {
        EnvFilter::new("warn")
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_writer(std::io::stderr)
        .try_init();
}
