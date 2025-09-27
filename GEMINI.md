# Project Overview

`gitsc` is a terminal command-line tool written in Rust, designed to automatically generate coherent and standardized Git commit messages based on the actual changes in a repository. It reduces the overhead of writing commit messages manually, ensuring consistency, semantic correctness, and scalability across projects and teams.

## Features

*   **Automatic Commit Message Generation:** Generates a single commit message for all changes.
*   **Smart Commit Splitting:** Splits large diffs into smaller, meaningful commits in interactive mode.
*   **Custom Formatting:** Supports Conventional Commits, Angular style, or user-defined templates.
*   **AI Integration:** Pluggable design for integration with different AI providers like OpenAI, Gemini, and Ollama.
*   **Configuration:** Customizable through `~/.config/gitsc/config.yml`.

## Building and Running

*   **Build the project:**
    ```bash
    cargo build
    ```
*   **Run the project:**
    ```bash
    cargo run
    ```
*   **Run tests:**
    ```bash
    cargo test
    ```

## Development Conventions

*   The project follows standard Rust conventions.
*   Source code is located in the `src` directory.
*   Dependencies are managed in the `Cargo.toml` file.
*   The project uses a layered architecture, with modules for CLI, Git operations, AI integration, and more.
*   Testing includes unit, integration, and end-to-end tests.