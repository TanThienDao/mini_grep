# An I/O Project: Building a Command Line Program

`minigrep` is a simplified version of the classic `grep` command-line utility, implemented in Rust. This project was built to practice fundamental Rust concepts such as ownership, error handling, and file I/O.

## Features

- **Argument Parsing**: Efficiently captures a search query and a file path from the command line.
- **Robust Error Handling**: Uses idiomatic Rust patterns like `Result` and `unwrap_or_else` to provide user-friendly error messages instead of technical panics.
- **Dynamic Error Propagation**: Utilizes `Box<dyn Error>` to handle various error types gracefully across the application.

## Getting Started

### Prerequisites

Ensure you have the Rust toolchain (Rustc and Cargo) installed. If not, you can get it from rustup.rs.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/minigrep.git
   cd minigrep
   ```
2. Build the project:
   ```bash
   cargo build --release   ```

## Usage

Run the program using `cargo run`, passing the string you want to search for and the path to the target file as arguments.

```bash
cargo run -- <query_string> <file_path>
```

### Example

```bash
cargo run -- search-term example.txt
```