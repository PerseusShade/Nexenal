use crate::config::Config;
use std::fs;
use std::io;
use std::path::Path;

pub fn execute(target: String, config: &Config) -> io::Result<()> {
    let mut cleaned_any = false;

    if target == "tree" || target == "*" {
        let targets = vec![
            config.tree.default_output.clone(),
            "struct.txt".to_string(),
            "tree.txt".to_string(),
        ];
        for t in targets {
            let path = Path::new(&t);
            if path.exists() {
                fs::remove_file(path)?;
                println!("Cleaned: Removed '{}'", t);
                cleaned_any = true;
            }
        }
    }

    if target == "all" || target == "*" {
        let targets = vec![
            config.all.default_output.clone(),
            "merged_code.txt".to_string(),
        ];
        for t in targets {
            let path = Path::new(&t);
            if path.exists() {
                fs::remove_file(path)?;
                println!("Cleaned: Removed '{}'", t);
                cleaned_any = true;
            }
        }
    }

    if !cleaned_any {
        println!("Nothing to clean for target '{}'.", target);
    } else {
        println!("Success! Cleanup completed.");
    }

    Ok(())
}