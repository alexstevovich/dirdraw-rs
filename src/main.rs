mod ignore;
mod tree;
mod output;

use clap::{Arg, Command, ArgAction};
use std::path::Path;

fn main() {
    let matches = Command::new("dirdraw")
        .version("1.0.0")
        .author("Your Name")
        .about("Draws a directory tree in the terminal")
        .arg(Arg::new("directory")
            .required(true)
            .help("The directory to scan"))
        .arg(Arg::new("ignore")
            .short('i')
            .long("ignore")
            .value_name("PATTERN")
            .num_args(1..)
            .help("Ignore specific files or folders"))
        .arg(Arg::new("ignore-file")
            .short('f')
            .long("ignore-file")
            .value_name("FILE")
            .help("Use an ignore file similar to .gitignore"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Save output to a file instead of printing"))
        .arg(Arg::new("icon")
            .short('x')
            .long("icon")
            .help("Enable folder/file icons")
            .action(ArgAction::SetTrue))
        .get_matches();

    let directory = matches.get_one::<String>("directory").unwrap();
    let ignore_patterns: Vec<String> = matches.get_many::<String>("ignore")
        .map(|v| v.map(|s| s.clone()).collect())
        .unwrap_or_else(Vec::new);
    let ignore_file = matches.get_one::<String>("ignore-file").map(|s| s.as_str());
    let output_file = matches.get_one::<String>("output").map(|s| s.as_str());
    let use_icons = matches.get_flag("icon");

    let ignored = ignore::load_ignore_patterns(ignore_file, &ignore_patterns);
    let tree_structure = tree::generate_tree(Path::new(directory), &ignored, use_icons);

    if let Some(output) = output_file {
        output::write_to_file(output, &tree_structure).expect("Failed to write to file");
    } else {
        output::print_to_console(&tree_structure);
    }
}
