use crate::config::Config;
use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;
use std::path::PathBuf;

/// Verbosity level for output control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}

/// CLI output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum OutputFormat {
    #[default]
    Pretty,
    Json,
}

/// Output format for command results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Pretty,
    Json,
}

impl Verbosity {
    /// Convert verbosity to log level string for RUST_LOG
    pub fn to_log_level(self) -> String {
        match self {
            Verbosity::Quiet => "error".to_string(),
            Verbosity::Normal => "info".to_string(),
            Verbosity::Verbose => "debug".to_string(),
        }
    }
}

#[derive(Parser)]
#[command(name = "soroban-debug")]
#[command(about = "A debugger for Soroban smart contracts", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Suppress non-essential output (errors and return value only)
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Show verbose output including internal details
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Show historical budget trend visualization
 pub struct RunArgs {

    /// Initial storage state as JSON object
    #[arg(short, long)]
    pub storage: Option<String>,

    /// Set breakpoint at function name
    #[arg(short, long)]
    pub breakpoint: Vec<String>,

    /// Network snapshot file to load before execution
    #[arg(long)]
    pub network_snapshot: Option<PathBuf>,

    /// Deprecated: use --network-snapshot instead
    #[arg(long, hide = true, alias = "snapshot")]
    pub snapshot: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Output format (text, json)
    #[arg(long)]
    pub format: Option<String>,

    /// Output mode for command result rendering (pretty, json)
    #[arg(long = "output", value_enum, default_value_t = OutputFormat::Pretty)]
    pub output_format: OutputFormat,

    /// Show contract events emitted during execution
    #[arg(long)]
    pub show_events: bool,

    /// Show authorization tree during execution
    #[arg(long)]
    pub show_auth: bool,

    /// Output format as JSON
    #[arg(long)]
    pub json: bool,

    /// Filter events by topic
    #[arg(long)]
    pub filter_topic: Option<String>,

    /// Execute the contract call N times for stress testing
    #[arg(long)]
    pub repeat: Option<u32>,

    /// Mock cross-contract return: CONTRACT_ID.function=return_value (repeatable)
    #[arg(long, value_name = "CONTRACT_ID.function=return_value")]
    pub mock: Vec<String>,

    /// Filter storage output by key pattern (repeatable). Supports:
 pub struct RunArgs {
    #[arg(long)]
    pub overwrite: bool,

    /// Execution timeout in seconds (default: 30)
    #[arg(long, default_value = "30")]
    pub timeout: u64,

    /// Trigger a prominent alert when a critical storage key is modified (repeatable)
    #[arg(long, value_name = "KEY_PATTERN")]
    pub alert_on_change: Vec<String>,

    /// Expected SHA-256 hash of the WASM file. If provided, loading will fail if the computed hash does not match.
    #[arg(long)]
    pub expected_hash: Option<String>,

    /// Show ledger entries accessed during execution
    #[arg(long)]
    pub show_ledger: bool,

    /// TTL warning threshold in ledger sequence numbers (default: 1000)
    #[arg(long, default_value = "1000")]
    pub ttl_warning_threshold: u32,
}

impl RunArgs {
    pub fn is_json_output(&self) -> bool {
        self.output_format == OutputFormat::Json
            || self.json
            || self
                .format
                .as_deref()
                .map(|f| f.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
    }

    pub fn merge_config(&mut self, config: &Config) {
        // Breakpoints
        if self.breakpoint.is_empty() && !config.debug.breakpoints.is_empty() {
            self.breakpoint = config.debug.breakpoints.clone();
        }

        // Show events
        if !self.show_events {
            if let Some(show) = config.output.show_events {
                self.show_events = show;
            }
        }

        // Output Format
        if self.format.is_none() {
            self.format = config.output.format.clone();
        }

        // Verbosity: if config has a level > 0 and CLI verbose is false, enable it
        if !self.verbose {
            if let Some(level) = config.debug.verbosity {
                if level > 0 {
                    self.verbose = true;
                }
            }
 pub struct RemoteArgs {
    pub args: Option<String>,
}

#[derive(Parser)]
pub struct AnalyzeArgs {
    /// Path to the contract WASM file
    #[arg(short, long)]
    pub contract: PathBuf,

    /// Function name to execute for dynamic analysis (optional)
    #[arg(short, long)]
    pub function: Option<String>,

    /// Function arguments as JSON array for dynamic analysis (optional)
    #[arg(short, long)]
    pub args: Option<String>,

    /// Initial storage state as JSON object (optional)
    #[arg(short, long)]
    pub storage: Option<String>,

    /// Output format (text, json)
    #[arg(long, default_value = "text")]
    pub format: String,
}

#[cfg(test)]
mod tests {
    use super::{Cli, Commands, OutputFormat};
    use clap::Parser;

    #[test]
    fn run_output_defaults_to_pretty() {
        let cli = Cli::parse_from([
            "soroban-debug",
            "run",
            "--contract",
            "contract.wasm",
            "--function",
            "increment",
        ]);

        let Commands::Run(args) = cli.command.expect("run command expected") else {
            panic!("run command expected");
        };

        assert_eq!(args.output_format, OutputFormat::Pretty);
        assert!(!args.is_json_output());
    }

    #[test]
    fn run_output_json_enables_json_mode() {
        let cli = Cli::parse_from([
            "soroban-debug",
            "run",
            "--contract",
            "contract.wasm",
            "--function",
            "increment",
            "--output",
            "json",
        ]);

        let Commands::Run(args) = cli.command.expect("run command expected") else {
            panic!("run command expected");
        };

        assert_eq!(args.output_format, OutputFormat::Json);
        assert!(args.is_json_output());
    }

    #[test]
    fn legacy_json_flag_still_enables_json_mode() {
        let cli = Cli::parse_from([
            "soroban-debug",
            "run",
            "--contract",
            "contract.wasm",
            "--function",
            "increment",
            "--json",
        ]);

        let Commands::Run(args) = cli.command.expect("run command expected") else {
            panic!("run command expected");
        };

        assert!(args.is_json_output());
    }

    #[test]
    fn legacy_format_json_still_enables_json_mode() {
        let cli = Cli::parse_from([
            "soroban-debug",
            "run",
            "--contract",
            "contract.wasm",
            "--function",
            "increment",
            "--format",
            "json",
        ]);

        let Commands::Run(args) = cli.command.expect("run command expected") else {
            panic!("run command expected");
        };

        assert!(args.is_json_output());
    }
}
