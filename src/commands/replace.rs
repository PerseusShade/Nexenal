use crate::config::{Config, ReplaceRule};
use crate::matcher::FilterMatcher;
use std::fs;
use std::io;
use std::path::Path;

pub fn execute(
    ext: String,
    dir: String,
    ignore: Vec<String>,
    ruleset: Option<String>,
    rule: Option<String>,
    find: Option<String>,
    replace: Option<String>,
    dry_run: bool,
    quiet: bool,
    config: &Config,
) -> io::Result<()> {
    let root = Path::new(&dir);
    let matcher = FilterMatcher::new(&config.global.ignore, &ignore);

    let mut rules_to_apply: Vec<ReplaceRule> = Vec::new();

    if let Some(rs_name) = ruleset {
        if let Some(rs) = config.replace.rulesets.get(&rs_name) {
            for r_name in &rs.rules {
                if let Some(r) = config.replace.rules.get(r_name) {
                    rules_to_apply.push(r.clone());
                } else if !quiet {
                    eprintln!("[WARN] Rule '{}' in ruleset '{}' not found.", r_name, rs_name);
                }
            }
        } else {
            if !quiet {
                eprintln!("[ERROR] Ruleset '{}' not found in configuration.", rs_name);
            }
            return Ok(());
        }
    } else if let Some(r_name) = rule {
        if let Some(r) = config.replace.rules.get(&r_name) {
            rules_to_apply.push(r.clone());
        } else {
            if !quiet {
                eprintln!("[ERROR] Rule '{}' not found in configuration.", r_name);
            }
            return Ok(());
        }
    } else if let (Some(f), Some(r)) = (find, replace) {
        rules_to_apply.push(ReplaceRule::new(f, r, true));
    } else if let Some(rs) = config.replace.rulesets.get("weird-chars") {
        for r_name in &rs.rules {
            if let Some(r) = config.replace.rules.get(r_name) {
                rules_to_apply.push(r.clone());
            }
        }
    } else {
        if !quiet {
            eprintln!("[ERROR] No rules specified. Use --ruleset, --rule, or --find/--replace.");
        }
        return Ok(());
    }

    if !quiet {
        println!(
            "Nexenal [Replace] applying {} rule(s) (ext: '{}', dry-run: {})...",
            rules_to_apply.len(),
            ext,
            dry_run
        );
    }

    let mut files_modified = 0;
    let mut total_matches = 0;

    run_replace(
        root,
        &ext,
        &matcher,
        &rules_to_apply,
        dry_run,
        quiet,
        root,
        &mut files_modified,
        &mut total_matches,
    )?;

    if !quiet {
        println!(
            "Completed: {} substitutions in {} file(s).{}",
            total_matches,
            files_modified,
            if dry_run { " (DRY RUN - no files changed)" } else { "" }
        );
    }

    Ok(())
}

fn execute_replacements_in_file(
    path: &Path,
    rules: &[ReplaceRule],
    dry_run: bool,
) -> io::Result<usize> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Ok(0),
    };

    let mut modified = content.clone();
    let mut total_replacements = 0;

    for rule in rules {
        if rule.literal {
            let count = modified.matches(&rule.find).count();
            if count > 0 {
                total_replacements += count;
                modified = modified.replace(&rule.find, &rule.replace);
            }
        }
    }

    if total_replacements > 0 && !dry_run {
        fs::write(path, modified)?;
    }

    Ok(total_replacements)
}

fn run_replace(
    dir_path: &Path,
    target_ext: &str,
    matcher: &FilterMatcher,
    rules: &[ReplaceRule],
    dry_run: bool,
    quiet: bool,
    base_path: &Path,
    files_modified: &mut usize,
    total_matches: &mut usize,
) -> io::Result<()> {
    let entries: Vec<_> = fs::read_dir(dir_path)?.filter_map(Result::ok).collect();
    let clean_ext = target_ext.trim_start_matches('.');

    for entry in entries {
        let path = entry.path();

        if matcher.should_ignore(&path) {
            continue;
        }

        if path.is_dir() {
            run_replace(
                &path,
                target_ext,
                matcher,
                rules,
                dry_run,
                quiet,
                base_path,
                files_modified,
                total_matches,
            )?;
        } else {
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let matches_ext = clean_ext == "*" || name.ends_with(&format!(".{}", clean_ext));

            if matches_ext {
                let count = execute_replacements_in_file(&path, rules, dry_run)?;
                if count > 0 {
                    if !quiet {
                        let rel_path = path.strip_prefix(base_path).unwrap_or(&path);
                        println!(
                            "  [MODIFIED] {} ({} substitutions)",
                            rel_path.display(),
                            count
                        );
                    }
                    *files_modified += 1;
                    *total_matches += count;
                }
            }
        }
    }
    Ok(())
}