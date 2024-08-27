# Challenge #1: Implementing `wc` in Rust

This project is a solution to the first challenge from John Crickett's coding challenges: building a custom version of the Unix `wc` tool in Rust.

## Overview

The Unix `wc` (word count) command is a simple but powerful tool that counts the number of bytes, words, and lines in a file. This challenge involves implementing similar functionality in Rust, with incremental steps to build up the tool's capabilities.

### Steps Implemented

1. **Step 1.1:**
   - Read command-line arguments and print them out.

## Project Structure

```
challenge#1-wc/
├── Cargo.toml          # Rust project configuration file
├── src/
│   └── main.rs         # Main source code for the ccwc tool
├── test.txt            # The test file used to verify the functionality
└── README.md           # Documentation for this challenge
```

## Usage

To run the current implementation:

```bash
cargo run -- -c test.txt
```

This will print the command-line arguments passed to the `ccwc` tool.

## Next Steps

- Implement the byte-counting functionality (`-c` option).
- Extend the tool to support line, word, and character counts, as well as handling input from stdin.

## Disclaimer

This is a work in progress and part of my learning journey with Rust. Feedback and suggestions are welcome!
