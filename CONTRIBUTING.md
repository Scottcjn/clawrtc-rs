# Contributing to clawrtc-rs

Thank you for your interest in contributing to clawrtc-rs! This guide will help you get started.

## 🚀 Quick Start

### Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs)
- **Git**: Version control system
- **Node.js** (optional): For testing with JavaScript bindings

### Setup

```bash
# Clone the repository
git clone https://github.com/Scottcjn/clawrtc-rs.git
cd clawrtc-rs

# Build the project
cargo build

# Run tests
cargo test
```

## 📝 Pull Request Guidelines

### Before Submitting

1. **Fork the repo** and create your branch from `main`
2. **Add tests** if you're adding new functionality
3. **Ensure tests pass**: `cargo test`
4. **Check code style**: `cargo clippy`
5. **Update documentation** if you change APIs

### PR Template

Please include in your PR description:

- **What** this PR changes
- **Why** this change is needed
- **How** you tested it
- **Related issues** (if any)

### Code Review

- All PRs require at least one review
- Be respectful and constructive in reviews
- Address review feedback promptly

## 🎨 Code Style

### Rust Conventions

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` before committing
- Run `cargo clippy` for linting
- Prefer descriptive variable names
- Document public APIs with `///` doc comments

### Example

```rust
/// Creates a new WebRTC connection.
///
/// # Arguments
///
/// * `config` - Connection configuration
///
/// # Returns
///
/// * `Result<Connection, Error>` - New connection or error
pub fn new_connection(config: Config) -> Result<Connection, Error> {
    // Implementation
}
```

## 🧪 Testing

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture
```

### Writing Tests

- Place tests in `tests/` directory or `#[cfg(test)]` modules
- Use descriptive test names: `test_feature_under_condition()`
- Test edge cases and error conditions

## 📚 Documentation

- Update `README.md` for user-facing changes
- Add `///` doc comments for public APIs
- Include usage examples in documentation
- Run `cargo doc` to preview docs

## 🐛 Reporting Issues

### Bug Reports

Include:

- **Steps to reproduce**
- **Expected behavior**
- **Actual behavior**
- **Environment** (Rust version, OS, etc.)
- **Code sample** (if applicable)

### Feature Requests

- Describe the use case
- Explain why it's valuable
- Suggest implementation approach (optional)

## 💬 Community

- Be respectful and inclusive
- Help others learn
- Share your knowledge

## 📜 License

By contributing, you agree that your contributions will be licensed under the project's license.

---

**Questions?** Open an issue or reach out to the maintainers.
