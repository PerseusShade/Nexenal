use crate::cli::{Cli, Commands};
use crate::commands::{all, clean, replace, tree, trim};
use crate::config::Config;
use clap::Parser;
use std::io;

pub fn execute(
    command_name: &str,
    global_dry_run: bool,
    global_quiet: bool,
    config: &Config,
) -> io::Result<()> {
    let custom_cmd = match config.custom_commands.get(command_name) {
        Some(cmd) => cmd,
        None => {
            eprintln!("[ERROR] Commande custom '{}' non trouvée dans config.json.", command_name);
            println!("\nCommandes custom disponibles :");
            for (name, cmd) in &config.custom_commands {
                println!("  - {}: {}", name, cmd.description);
            }
            return Ok(());
        }
    };

    for step_str in &custom_cmd.steps {
        // Découpe la ligne d'arguments définie dans le JSON (ex: "replace -s weird-chars -q")
        let mut tokens = vec!["nexenal".to_string()];
        tokens.extend(step_str.split_whitespace().map(|s| s.to_string()));

        // Injection des flags globaux si passés lors de l'appel principal
        if global_dry_run && !tokens.iter().any(|t| t == "--dry-run") {
            tokens.push("--dry-run".to_string());
        }
        if global_quiet && !tokens.iter().any(|t| t == "-q" || t == "--quiet") {
            tokens.push("-q".to_string());
        }

        match Cli::try_parse_from(&tokens) {
            Ok(cli) => match cli.command {
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
                    replace::execute(
                        ext, dir, ignore, ruleset, rule, find, replace, dry_run, quiet, config,
                    )?;
                }
                Some(Commands::Trim {
                    ext,
                    dir,
                    ignore,
                    dry_run,
                    quiet,
                }) => {
                    trim::execute(ext, dir, ignore, dry_run, quiet, config)?;
                }
                Some(Commands::Tree {
                    dir,
                    output,
                    ignore,
                    clipboard,
                    quiet,
                }) => {
                    tree::execute(dir, output, ignore, clipboard, quiet, config)?;
                }
                Some(Commands::All {
                    ext,
                    dir,
                    output,
                    ignore,
                    clipboard,
                    quiet,
                }) => {
                    all::execute(ext, dir, output, ignore, clipboard, quiet, config)?;
                }
                Some(Commands::Clean { target }) => {
                    clean::execute(target, config)?;
                }
                _ => {
                    eprintln!("[WARN] Étape non supportée dans un workflow custom : '{}'", step_str);
                }
            },
            Err(e) => {
                eprintln!("[ERROR] Erreur de syntaxe dans l'étape '{}' : {}", step_str, e);
            }
        }
    }

    Ok(())
}