# gitsc: Smart Git Commit Message Generator

`gitsc` is a terminal command-line tool written in Rust, designed to automatically generate coherent and standardized Git commit messages based on the actual changes in a repository. It reduces the overhead of writing commit messages manually, ensuring consistency, semantic correctness, and scalability across projects and teams.

## Features

*   **Automatic Commit Message Generation:** Generates a single commit message for all changes.
*   **Smart Commit Splitting:** Splits large diffs into smaller, meaningful commits in interactive mode.
*   **Custom Formatting:** Supports Conventional Commits, Angular style, or user-defined templates.
*   **AI Integration:** Pluggable design for integration with different AI providers like OpenAI, Gemini, and Ollama.
*   **Caching with Redis:** Reduces API calls to AI providers by caching generated commit messages.

## Installation

To install `gitsc`, you need to have Rust and Cargo installed. If you don't have them, you can install them from [rust-lang.org](https://www.rust-lang.org/tools/install).

```bash
cargo install --path .
```

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

## Usage

To generate a commit message for your staged changes:

```bash
gitsc
```

To enable debug output, including cache hits/misses, AI response times, and diff details:

```bash
gitsc -d
```

Ensure you have changes staged (`git add .`) before running `gitsc`.

## Configuration

`gitsc` is customizable through `~/.config/gitsc/config.yml`. An example configuration:

```yaml
provider: gemini
model: "gemini-2.5-flash"
redis_url: "redis://127.00.1/"
commit_format: "{type}({scope}): {message}"
log:
  path: "/var/logs/gitsc.log"
  format: "nmap"
smart_commit:
  line_threshold: 150
```

### Key Configuration Options:

*   `provider`: The AI provider to use (e.g., `gemini`).
*   `model`: The specific AI model to use (e.g., `gemini-2.5-flash`).
*   `redis_url`: (Optional) The URL for your Redis instance (e.g., `redis://127.0.0.1/`). If provided, `gitsc` will use Redis for caching AI responses.
*   `commit_format`: A template string for the generated commit message (e.g., `{type}({scope}): {message}`).

## Contributing

We welcome contributions to `gitsc`! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
