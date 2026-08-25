use crate::config::Config;
use crate::matcher::FilterMatcher;
use crate::updater::copy_to_clipboard;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn execute(
    ext: String,
    dir: String,
    output: Option<String>,
    ignore: Vec<String>,
    clipboard: bool,
    quiet: bool,
    config: &Config,
) -> io::Result<()> {
    let root = Path::new(&dir);
    let mut extra_ignores = ignore.clone();

    let final_output = output.clone().unwrap_or_else(|| {
        if config.all.default_output.is_empty() {
            "merged_code.txt".to_string()
        } else {
            config.all.default_output.clone()
        }
    });

    extra_ignores.push(final_output);
    extra_ignores.push(config.all.default_output.clone());
    extra_ignores.push("merged_code.txt".to_string());

    let matcher = FilterMatcher::new(&config.global.ignore, &extra_ignores);

    let abs_path = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let mut clean_scan_path = abs_path.display().to_string();
    if clean_scan_path.starts_with(r"\\?\") {
        clean_scan_path = clean_scan_path.replacen(r"\\?\", "", 1);
    }

    if !quiet {
        if ext == "*" {
            println!("Nexenal [All] gathering ALL files from {}...", clean_scan_path);
        } else {
            println!(
                "Nexenal [All] gathering '.{}' files from {}...",
                ext.trim_start_matches('.'),
                clean_scan_path
            );
        }
    }

    let mut merged_content = String::new();
    run_all(root, &ext, &mut merged_content, &matcher, root)?;

    let line_count = merged_content.lines().count();

    if clipboard {
        if let Err(e) = copy_to_clipboard(&merged_content) {
            if !quiet {
                eprintln!("[ERROR] Failed to copy to clipboard: {}", e);
            }
        } else if !quiet {
            println!(
                "[SUCCESS] Code successfully copied to clipboard! ({} lines copied)",
                line_count
            );
        }
    } else {
        let final_output_file = output.unwrap_or_else(|| {
            if config.all.default_output.is_empty() {
                "merged_code.txt".to_string()
            } else {
                config.all.default_output.clone()
            }
        });
        let mut file = File::create(&final_output_file)?;
        file.write_all(merged_content.as_bytes())?;
        if !quiet {
            println!(
                "Success! Code merged into '{}'. ({} lines)",
                final_output_file, line_count
            );
        }
    }
    Ok(())
}

fn run_all(
    dir_path: &Path,
    target_ext: &str,
    out: &mut String,
    matcher: &FilterMatcher,
    base_path: &Path,
) -> io::Result<()> {
    let entries: Vec<_> = fs::read_dir(dir_path)?.filter_map(Result::ok).collect();
    let clean_ext = target_ext.trim_start_matches('.');

    for entry in entries {
        let path = entry.path();

        if matcher.should_ignore(&path) {
            continue;
        }

        if path.is_dir() {
            run_all(&path, target_ext, out, matcher, base_path)?;
        } else {
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let matches_ext = clean_ext == "*" || name.ends_with(&format!(".{}", clean_ext));

            if matches_ext {
                let relative_path = path.strip_prefix(base_path).unwrap_or(&path);

                out.push_str(&format!("{} :\n\n", relative_path.display()));

                match fs::read_to_string(&path) {
                    Ok(content) => out.push_str(&content),
                    Err(e) => out.push_str(&format!("[READ ERROR: {}]", e)),
                }

                out.push_str(&format!("\n{}\n\n", "=".repeat(50)));
            }
        }
    }
    Ok(())
}