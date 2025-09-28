# Contributing to gitsc

We welcome contributions to `gitsc`! To ensure a smooth and collaborative process, please follow these guidelines.

## How to Contribute

1.  **Fork the Repository:** Start by forking the `gitsc` repository to your GitHub account.
2.  **Clone Your Fork:** Clone your forked repository to your local machine:
    ```bash
    git clone https://github.com/YOUR_USERNAME/gitsc.git
    cd gitsc
    ```
3.  **Create a New Branch:** Create a new branch for your feature or bug fix:
    ```bash
    git checkout -b feature/your-feature-name
    # or
    git checkout -b bugfix/your-bug-fix-name
    ```
4.  **Make Your Changes:** Implement your changes, adhering to the existing code style and conventions.
5.  **Run Tests:** Ensure all existing tests pass and add new tests for your changes if applicable:
    ```bash
    cargo test
    ```
6.  **Lint and Format:** Make sure your code is properly formatted and passes lint checks:
    ```bash
    cargo fmt --all
    cargo clippy --all-targets
    ```
7.  **Commit Your Changes:** Write clear and concise commit messages. We follow the Conventional Commits specification (e.g., `feat(scope): Add new feature`).
    ```bash
    git commit -m "feat(module): Briefly describe your change"
    ```
8.  **Push to Your Fork:** Push your changes to your forked repository:
    ```bash
    git push origin feature/your-feature-name
    ```
9.  **Create a Pull Request:** Open a pull request from your branch to the `main` branch of the original `gitsc` repository. Provide a clear description of your changes.

## Code Style and Conventions

*   Follow standard Rust conventions.
*   Ensure your code is well-documented where necessary.
*   Keep functions and modules focused on a single responsibility.

## Reporting Bugs

If you find a bug, please open an issue on GitHub with a clear description, steps to reproduce, and expected behavior.

## Feature Requests

For new features or enhancements, please open an issue first to discuss the idea before submitting a pull request.
