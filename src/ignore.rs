use std::collections::HashSet;
use std::fs;

pub fn load_ignore_patterns(ignore_file: Option<&str>, cli_ignores: &[String]) -> HashSet<String> {
    let mut ignored = HashSet::new();

    // Load patterns from ignore file
    if let Some(file) = ignore_file {
        if let Ok(contents) = fs::read_to_string(file) {
            for line in contents.lines() {
                if !line.trim().is_empty() && !line.starts_with('#') {
                    ignored.insert(line.trim().to_string());
                }
            }
        }
    }

    // Add CLI ignores
    for pattern in cli_ignores {
        ignored.insert(pattern.clone());
    }

    ignored
}
