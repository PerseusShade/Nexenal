use crate::config::Config;
use crate::matcher::FilterMatcher;
use std::fs;
use std::io;
use std::path::Path;

pub fn execute(
    ext: String,
    dir: String,
    ignore: Vec<String>,
    dry_run: bool,
    quiet: bool,
    config: &Config,
) -> io::Result<()> {
    let root = Path::new(&dir);
    let matcher = FilterMatcher::new(&config.global.ignore, &ignore);

    if !quiet {
        println!(
            "Nexenal [Trim] removing trailing whitespaces (ext: '{}', dry-run: {})...",
            ext,
            dry_run
        );
    }

    let mut files_modified = 0;
    let mut total_lines_trimmed = 0;

    run_trim(
        root,
        &ext,
        &matcher,
        dry_run,
        quiet,
        root,
        &mut files_modified,
        &mut total_lines_trimmed,
    )?;

    if !quiet {
        println!(
            "Completed: {} line(s) trimmed across {} file(s).{}",
            total_lines_trimmed,
            files_modified,
            if dry_run { " (DRY RUN - no files changed)" } else { "" }
        );
    }

    Ok(())
}

fn trim_file(path: &Path, dry_run: bool) -> io::Result<usize> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Ok(0),
    };

    let mut lines_trimmed = 0;
    let mut cleaned_lines: Vec<String> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim_end();
        if trimmed.len() != line.len() {
            lines_trimmed += 1;
        }
        cleaned_lines.push(trimmed.to_string());
    }

    if lines_trimmed > 0 {
        let mut new_content = cleaned_lines.join("\n");
        if !new_content.is_empty() {
            new_content.push('\n');
        }

        if !dry_run {
            fs::write(path, new_content)?;
        }
    }

    Ok(lines_trimmed)
}

fn run_trim(
    dir_path: &Path,
    target_ext: &str,
    matcher: &FilterMatcher,
    dry_run: bool,
    quiet: bool,
    base_path: &Path,
    files_modified: &mut usize,
    total_lines_trimmed: &mut usize,
) -> io::Result<()> {
    let entries: Vec<_> = fs::read_dir(dir_path)?.filter_map(Result::ok).collect();
    let clean_ext = target_ext.trim_start_matches('.');

    for entry in entries {
        let path = entry.path();

        if matcher.should_ignore(&path) {
            continue;
        }

        if path.is_dir() {
            run_trim(
                &path,
                target_ext,
                matcher,
                dry_run,
                quiet,
                base_path,
                files_modified,
                total_lines_trimmed,
            )?;
        } else {
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let matches_ext = clean_ext == "*" || name.ends_with(&format!(".{}", clean_ext));

            if matches_ext {
                let trimmed_count = trim_file(&path, dry_run)?;
                if trimmed_count > 0 {
                    if !quiet {
                        let rel_path = path.strip_prefix(base_path).unwrap_or(&path);
                        println!(
                            "  [TRIMMED] {} ({} line(s))",
                            rel_path.display(),
                            trimmed_count
                        );
                    }
                    *files_modified += 1;
                    *total_lines_trimmed += trimmed_count;
                }
            }
        }
    }
    Ok(())
}