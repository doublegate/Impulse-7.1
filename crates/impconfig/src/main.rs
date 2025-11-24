//! Configuration management CLI tool for Impulse-Next_BBS
//!
//! This tool provides command-line utilities for managing BBS configuration files:
//! - Generate default configuration files
//! - Validate existing configurations
//! - Display current configuration settings
//! - Compare two configuration files

mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Configuration management CLI for Impulse-Next_BBS
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a default configuration file
    ///
    /// Creates a new configuration file with default values at the specified path.
    /// If no path is provided, creates "config.toml" in the current directory.
    Generate {
        /// Output file path
        #[arg(short, long, default_value = "config.toml")]
        output: PathBuf,

        /// Output format (toml or json)
        #[arg(short, long, default_value = "toml")]
        format: String,

        /// Overwrite existing file without prompting
        #[arg(short = 'f', long)]
        force: bool,
    },

    /// Validate an existing configuration file
    ///
    /// Loads and validates a configuration file, checking for:
    /// - Required fields
    /// - Valid port numbers
    /// - Proper security settings
    /// - Path existence (in strict mode)
    Validate {
        /// Configuration file to validate
        #[arg(short, long, default_value = "config.toml")]
        config: PathBuf,

        /// Validation mode
        #[arg(short, long, default_value = "strict", value_parser = ["config-only", "strict", "deployment"])]
        mode: String,
    },

    /// Display current configuration
    ///
    /// Loads and displays the effective configuration, including any
    /// environment variable overrides.
    Show {
        /// Configuration file to display
        #[arg(short, long, default_value = "config.toml")]
        config: PathBuf,

        /// Output format (toml, json, or table)
        #[arg(short, long, default_value = "toml")]
        format: String,
    },

    /// Compare two configuration files
    ///
    /// Shows the differences between two configuration files,
    /// highlighting added, removed, and changed fields.
    Diff {
        /// First configuration file
        config1: PathBuf,

        /// Second configuration file
        config2: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            output,
            format,
            force,
        } => commands::generate::execute(output, format, force),

        Commands::Validate { config, mode } => commands::validate::execute(config, mode),

        Commands::Show { config, format } => commands::show::execute(config, format),

        Commands::Diff { config1, config2 } => commands::diff::execute(config1, config2),
    }
}
