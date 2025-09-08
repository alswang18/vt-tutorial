# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a simple Rust tutorial project (`vt-tutorial`) using Rust 2024 edition. The project consists of a basic "Hello, world!" program.

## Common Commands

### Build and Run
- `cargo build` - Build the project
- `cargo run` - Build and run the project
- `cargo build --release` - Build optimized release version
- `cargo check` - Check code without building

### Testing
- `cargo test` - Run all tests

### Development
- `cargo fmt` - Format code
- `cargo clippy` - Run linter for code quality checks

## Project Structure

- `src/main.rs` - Main entry point containing the basic "Hello, world!" program
- `Cargo.toml` - Project configuration and dependencies
- `target/` - Build artifacts (excluded from version control)