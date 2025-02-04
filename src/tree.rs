use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub fn generate_tree(root: &Path, ignored: &HashSet<String>, use_icons: bool) -> String {
    let mut output = String::new();

    let root_canonical: PathBuf = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let root_name = root_canonical
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "[root]".to_string());

    // Print root without a connector
    output.push_str(&format!("{}\n", root_name));

    build_tree(root, "", true, ignored, use_icons, &mut output);
    output
}

fn build_tree(path: &Path, prefix: &str, is_last: bool, ignored: &HashSet<String>, use_icons: bool, output: &mut String) {
    let entry_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("[unknown]");

    if ignored.contains(entry_name) {
        return;
    }

    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            entries.sort_by_key(|e| e.path());

            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

            for (i, entry) in entries.iter().enumerate() {
                let entry_path = entry.path();
                let entry_name = entry_path.file_name().and_then(|s| s.to_str()).unwrap_or("[unknown]");

                if ignored.contains(entry_name) {
                    continue;
                }

                let connector = if i == entries.len() - 1 { "└── " } else { "├── " };
                let icon = if use_icons {
                    if entry_path.is_dir() { "📂 " } else { "📄 " }
                } else {
                    ""
                };

                output.push_str(&format!("{}{}{}{}\n", new_prefix, connector, icon, entry_name));

                build_tree(&entry_path, &new_prefix, i == entries.len() - 1, ignored, use_icons, output);
            }
        }
    }
}
