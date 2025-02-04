use std::fs::File;
use std::io::{self, Write};

pub fn print_to_console(tree: &str) {
    println!("{}", tree);
}

pub fn write_to_file(filename: &str, tree: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(tree.as_bytes())?;
    Ok(())
}
