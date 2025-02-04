# dirdraw

`dirdraw` is a Rust CLI tool that generates an ASCII tree representation of a directory. It supports ignoring files via command-line arguments or `.gitignore`-like ignore files. Optionally, the output can be saved to a text file.

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/dirdraw-rs.git
   cd dirdraw-rs
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

3. Run the executable:
   ```sh
   ./target/release/dirdraw .
   ```

## Usage

### Basic Usage
```sh
 dirdraw <directory>
```
Example:
```sh
 dirdraw .
```
This will print the directory tree of the current folder.

### Ignoring Specific Folders or Files
```sh
 dirdraw <directory> --ignore <patterns>
```
Example:
```sh
 dirdraw . --ignore node_modules dist
```
This will exclude `node_modules/` and `dist/` from the tree.

### Using an Ignore File (like `.gitignore`)
```sh
 dirdraw <directory> --ignore-file <file>
```
Example:
```sh
 dirdraw . --ignore-file .dirdrawignore
```
This will exclude files and folders listed in `.dirdrawignore`.

Example `.dirdrawignore` file:
```
node_modules/
dist/
target/
*.log
```

### Saving Output to a File
```sh
 dirdraw <directory> --output <filename>
```
Example:
```sh
 dirdraw . --output tree.txt
```
This will save the directory structure to `tree.txt` instead of printing it to the console.

## Example Output
```
📂 my-folder
 ├── 📂 src
 │   ├── 📂 components
 │   ├── 📄 main.rs
 │   ├── 📄 tree.rs
 ├── 📄 README.md
 ├── 📄 Cargo.toml
```

## License
MIT License
