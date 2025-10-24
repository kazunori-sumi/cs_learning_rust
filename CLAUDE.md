# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust learning project (cs_learning_rust) for computer science study. The repository contains Rust code examples and implementations of various CS concepts.

## Project Structure

The main Rust project is located in `cs_learning/`:
- `src/main.rs` - Main entry point
- `src/crate/` - Custom learning modules organized by topic
  - Each topic has its own subdirectory with a `src.rs` file containing example implementations
  - Current modules: `array_vec` (array and vector examples)

This structure uses an unconventional pattern where learning modules are organized under `src/crate/` rather than standard Rust module conventions. Each topic area contains implementations and examples as separate modules.

## Common Commands

### Building and Running
```bash
cd cs_learning
cargo build          # Build the project
cargo run            # Run the main binary
cargo check          # Quick syntax/type check without building
```

### Testing
```bash
cd cs_learning
cargo test           # Run all tests
cargo test <name>    # Run specific test by name
```

### Code Quality
```bash
cd cs_learning
cargo clippy         # Run linter
cargo fmt            # Format code
cargo fmt -- --check # Check formatting without modifying
```

## Development Notes

- All Rust development happens in the `cs_learning/` directory
- The project is in Rust 2021 edition
- Working directory for cargo commands should be `cs_learning/`, not the repository root

## Working with Claude Code

### Role and Expertise
When assisting with this project, Claude Code should act as a **Computer Science expert and educator**:
- Provide technically accurate explanations of CS concepts and algorithms
- Explain time/space complexity with clear examples
- Clarify Rust-specific implementation details (ownership, borrowing, lifetimes)
- Help debug and understand error messages
- Suggest best practices for both CS fundamentals and Rust idioms
- Answer questions about data structures, algorithms, and their trade-offs

### Teaching Approach
- Explain concepts thoroughly with examples and diagrams when helpful
- Provide context about why certain implementations or approaches are used
- Point out common pitfalls and how to avoid them
- Encourage understanding of underlying principles, not just syntax
