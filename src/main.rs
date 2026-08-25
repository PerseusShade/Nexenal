mod cli;
mod commands;
mod config;
mod matcher;
mod updater;

use clap::{CommandFactory, Parser};
use cli::{Cli, Commands, ConfigActions};
use config::{fetch_online_default_config, load_config, save_config};
use std::env;
use std::io;
use updater::{open_file, update_nexenal};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = load_config();

    if args.len() > 1 && !args[1].starts_with('-') {
        let first_arg = &args[1];
        let native_commands = [
            "tree", "all", "replace", "trim", "run", "clean", "config", "update",
            "docs", "license", "help",
        ];

        if !native_commands.contains(&first_arg.as_str())
            && config.custom_commands.contains_key(first_arg)
        {
            let is_quiet = args.iter().any(|a| a == "-q" || a == "--quiet");
            let is_dry_run = args.iter().any(|a| a == "--dry-run");

            return commands::custom::execute(first_arg, is_dry_run, is_quiet, &config);
        }
    }

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Tree {
            dir,
            output,
            ignore,
            clipboard,
            quiet,
        }) => {
            commands::tree::execute(dir, output, ignore, clipboard, quiet, &config)?;
        }

        Some(Commands::All {
            ext,
            dir,
            output,
            ignore,
            clipboard,
            quiet,
        }) => {
            commands::all::execute(ext, dir, output, ignore, clipboard, quiet, &config)?;
        }

        Some(Commands::Replace {
            ext,
            dir,
            ignore,
            ruleset,
            rule,
            find,
            replace,
            dry_run,
            quiet,
        }) => {
            commands::replace::execute(
                ext, dir, ignore, ruleset, rule, find, replace, dry_run, quiet, &config,
            )?;
        }

        Some(Commands::Trim {
            ext,
            dir,
            ignore,
            dry_run,
            quiet,
        }) => {
            commands::trim::execute(ext, dir, ignore, dry_run, quiet, &config)?;
        }

        Some(Commands::Run {
            command_name,
            dry_run,
            quiet,
            ..
        }) => {
            commands::custom::execute(&command_name, dry_run, quiet, &config)?;
        }

        Some(Commands::Clean { target }) => {
            commands::clean::execute(target, &config)?;
        }

        Some(Commands::Config { action }) => match action {
            ConfigActions::View => {
                let json_output = serde_json::to_string_pretty(&config)?;
                println!("--- Current Nexenal Configuration ---\n{}", json_output);
            }
            ConfigActions::Edit => {
                open_file("config.json");
            }
            ConfigActions::Ignore { folder } => {
                let mut new_config = config.clone();
                if !new_config.global.ignore.contains(&folder) {
                    new_config.global.ignore.push(folder.clone());
                    save_config(&new_config)?;
                    println!("Success! Added '{}' to global ignore list.", folder);
                } else {
                    println!("Info: '{}' is already in the global ignore list.", folder);
                }
            }
            ConfigActions::Unignore { folder } => {
                let mut new_config = config.clone();
                if let Some(pos) = new_config.global.ignore.iter().position(|x| *x == folder) {
                    new_config.global.ignore.remove(pos);
                    save_config(&new_config)?;
                    println!("Success! Removed '{}' from global ignore list.", folder);
                } else {
                    println!("Info: '{}' was not found in the global ignore list.", folder);
                }
            }
            ConfigActions::Reset => {
                let default_cfg = config::Config::default();
                if save_config(&default_cfg).is_ok() {
                    println!("[SUCCESS] Configuration reset to embedded default assets/config.json settings.");
                } else {
                    eprintln!("[ERROR] Failed to write default config file.");
                }
            }
            ConfigActions::Update => {
                println!("Fetching online default configuration...");
                match fetch_online_default_config() {
                    Ok(online_cfg) => {
                        if save_config(&online_cfg).is_ok() {
                            println!("[SUCCESS] Config updated successfully from remote repository!");
                        } else {
                            eprintln!("[ERROR] Failed to save updated config locally.");
                        }
                    }
                    Err(e) => {
                        eprintln!("[ERROR] Failed to download online config: {}", e);
                    }
                }
            }
        },

        Some(Commands::Update) => {
            if let Err(e) = update_nexenal() {
                eprintln!("[ERROR] Update failed: {}", e);
            }
        }

        Some(Commands::Docs) => {
            open_file("README.md");
        }

        Some(Commands::License) => {
            open_file("LICENSE.md");
        }

        None => {
            let _ = Cli::command().print_help();
        }
    }

    Ok(())
}