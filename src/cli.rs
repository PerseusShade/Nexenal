use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nexenal")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "The ultimate developer arsenal", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generates a visual tree of your project architecture
    Tree {
        #[arg(short, long, default_value = ".")]
        dir: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long)]
        ignore: Vec<String>,
        #[arg(short, long)]
        clipboard: bool,
        #[arg(short, long)]
        quiet: bool,
    },
    /// Merges all files with a specific extension into a single file
    All {
        ext: String,
        #[arg(short, long, default_value = ".")]
        dir: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long)]
        ignore: Vec<String>,
        #[arg(short, long)]
        clipboard: bool,
        #[arg(short, long)]
        quiet: bool,
    },
    /// Replaces strings across files based on rules or inline arguments
    Replace {
        #[arg(short, long, default_value = "*")]
        ext: String,
        #[arg(short, long, default_value = ".")]
        dir: String,
        #[arg(short, long)]
        ignore: Vec<String>,
        #[arg(short = 's', long, conflicts_with_all = &["rule", "find"])]
        ruleset: Option<String>,
        #[arg(long, conflicts_with_all = &["ruleset", "find"])]
        rule: Option<String>,
        #[arg(short, long, requires = "replace", conflicts_with_all = &["ruleset", "rule"])]
        find: Option<String>,
        #[arg(short = 'r', long, requires = "find")]
        replace: Option<String>,
        #[arg(long)]
        dry_run: bool,
        #[arg(short, long)]
        quiet: bool,
    },
    /// Removes trailing whitespaces and normalizes end of lines across files
    Trim {
        #[arg(short, long, default_value = "*")]
        ext: String,
        #[arg(short, long, default_value = ".")]
        dir: String,
        #[arg(short, long)]
        ignore: Vec<String>,
        #[arg(long)]
        dry_run: bool,
        #[arg(short, long)]
        quiet: bool,
    },
    /// Runs a custom configured workflow by name
    Run {
        command_name: String,
        #[arg(short, long, default_value = "*")]
        ext: String,
        #[arg(short, long, default_value = ".")]
        dir: String,
        #[arg(short, long)]
        ignore: Vec<String>,
        #[arg(long)]
        dry_run: bool,
        #[arg(short, long)]
        quiet: bool,
    },
    /// Cleans generated files (tree, all, or everything)
    Clean { target: String },
    /// Manages persistent configuration
    Config {
        #[command(subcommand)]
        action: ConfigActions,
    },
    /// Updates Nexenal to the latest version from GitHub
    Update,
    /// Opens the official documentation (README.md)
    Docs,
    /// Opens the software license (LICENSE.md)
    License,
}

#[derive(Subcommand)]
pub enum ConfigActions {
    View,
    Edit,
    Ignore { folder: String },
    Unignore { folder: String },
    Reset,
    Update,
}