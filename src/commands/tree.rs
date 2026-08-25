use crate::config::Config;
use crate::matcher::FilterMatcher;
use crate::updater::copy_to_clipboard;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn execute(
    dir: String,
    output: Option<String>,
    ignore: Vec<String>,
    clipboard: bool,
    quiet: bool,
    config: &Config,
) -> io::Result<()> {
    let root = Path::new(&dir);
    let matcher = FilterMatcher::new(&config.global.ignore, &ignore);

    let mut tree_content = String::new();
    let abs_path = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());

    let mut clean_scan_path = abs_path.display().to_string();
    if clean_scan_path.starts_with(r"\\?\") {
        clean_scan_path = clean_scan_path.replacen(r"\\?\", "", 1);
    }

    let root_name = abs_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    tree_content.push_str(&format!("{}/\n", root_name));

    if !quiet {
        println!("Nexenal [Tree] scanning: {}", clean_scan_path);
    }
    run_tree(root, "", &mut tree_content, &matcher)?;

    let line_count = tree_content.lines().count();

    if clipboard {
        if let Err(e) = copy_to_clipboard(&tree_content) {
            if !quiet {
                eprintln!("[ERROR] Failed to copy to clipboard: {}", e);
            }
        } else if !quiet {
            println!(
                "[SUCCESS] Architecture copied to clipboard! ({} lines copied)",
                line_count
            );
        }
    } else {
        let final_output = output.unwrap_or_else(|| {
            if config.tree.default_output.is_empty() {
                "struct.txt".to_string()
            } else {
                config.tree.default_output.clone()
            }
        });
        let mut file = File::create(&final_output)?;
        file.write_all(tree_content.as_bytes())?;
        if !quiet {
            println!(
                "Success! Architecture generated in '{}'. ({} lines)",
                final_output, line_count
            );
        }
    }
    Ok(())
}

fn run_tree(
    dir_path: &Path,
    prefix: &str,
    out: &mut String,
    matcher: &FilterMatcher,
) -> io::Result<()> {
    let entries: Vec<_> = fs::read_dir(dir_path)?.filter_map(Result::ok).collect();
    let mut entries: Vec<_> = entries
        .into_iter()
        .filter(|entry| !matcher.should_ignore(&entry.path()))
        .collect();

    entries.sort_by(|a, b| {
        let a_is_dir = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        let b_is_dir = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        b_is_dir
            .cmp(&a_is_dir)
            .then(a.file_name().cmp(&b.file_name()))
    });

    let count = entries.len();

    for (i, entry) in entries.iter().enumerate() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        let is_last = i == count - 1;
        let connector = if is_last { "└── " } else { "├── " };

        let display_name = if is_dir {
            format!("{}/", name)
        } else {
            name.clone()
        };
        out.push_str(&format!("{}{}{}\n", prefix, connector, display_name));

        if is_dir {
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            run_tree(&path, &new_prefix, out, matcher)?;
        }
    }
    Ok(())
}