# Contributing to clawrtc

Thanks for your interest in contributing to the RustChain miner crate.

## Setup

```bash
git clone https://github.com/Scottcjn/clawrtc-rs.git
cd clawrtc-rs
cargo build
cargo test
```

Requires Rust 1.70+ (2021 edition).

## Making Changes

1. Fork the repo and create a branch from `main`
2. Write your code and add tests if applicable
3. Run `cargo fmt` and `cargo clippy` before committing
4. Open a PR against `main` with a clear description

## Code Style

- Follow standard Rust conventions (`rustfmt` defaults)
- Use `thiserror` for error types
- Keep public API surface minimal — add docs for any new public items
- Prefer `Result<T, E>` over panics

## Pull Request Guidelines

- Keep PRs focused — one feature or fix per PR
- Include a brief description of what changed and why
- Reference any related issues (e.g., `Fixes #123`)
- All CI checks must pass before merge

## Reporting Issues

Open an issue with:
- What you expected vs. what happened
- Steps to reproduce
- Rust version (`rustc --version`) and OS

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
