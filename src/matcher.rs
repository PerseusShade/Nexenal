use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::Path;

pub struct FilterMatcher {
    glob_set: GlobSet,
    literal_ignores: Vec<String>,
}

impl FilterMatcher {
    pub fn new(config_ignores: &[String], extra_ignores: &[String]) -> Self {
        let mut builder = GlobSetBuilder::new();
        let mut literal_ignores = Vec::new();

        let all_patterns = config_ignores.iter().chain(extra_ignores.iter());

        for pattern in all_patterns {
            let p = pattern.trim();
            if p.is_empty() || p.starts_with("//") {
                continue;
            }

            let clean_p = p.trim_end_matches('/');
            literal_ignores.push(clean_p.to_string());

            let glob_str = if p.contains('*') || p.contains('/') {
                p.to_string()
            } else {
                format!("**/{clean_p}/**")
            };

            if let Ok(glob) = Glob::new(&glob_str) {
                builder.add(glob);
            }
            if let Ok(glob) = Glob::new(clean_p) {
                builder.add(glob);
            }
        }

        Self {
            glob_set: builder.build().unwrap_or_default(),
            literal_ignores,
        }
    }

    pub fn should_ignore(&self, path: &Path) -> bool {
        let relative_path_str = path.to_string_lossy().replace('\\', "/");
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if self.literal_ignores.iter().any(|lit| lit == &name || lit == &relative_path_str) {
            return true;
        }

        self.glob_set.is_match(path) || self.glob_set.is_match(&relative_path_str)
    }
}