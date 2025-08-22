# Contributing to Hanabi

Thank you for your interest in contributing to Hanabi! This guide will help you get set up, find ways to contribute, and submit high‑quality changes.

Hanabi is a blazingly fast, configurable message broker and event streaming platform written in Rust. The project is in early development — contributions of all sizes are welcome.

## Table of contents
- Getting help and asking questions
- Ways to contribute
- Development setup
- Coding standards and tooling
- Git, branches, and commit messages
- Opening a Pull Request (PR)
- Code review process
- Issue triage and labels
- Security and responsible disclosure
- License and contributor certification
- Acknowledgements


## Getting help and asking questions
- Issues: https://github.com/Northwind-Software/hanabi/issues
- Discussions: https://github.com/Northwind-Software/hanabi/discussions
- For private or sensitive matters (including security), see the Security section below.

Please search existing issues before opening a new one.

## Ways to contribute
- Report bugs and performance problems.
- Propose and discuss new features or improvements.
- Improve documentation, examples, and developer experience.
- Write tests and increase coverage.
- Review pull requests and share feedback.

If you plan a significant change, please open an issue first to discuss the approach.

## Development setup
Hanabi uses the Rust 2024 edition and currently focuses on Rust 1.89.0 or newer.

1. Install Rust using rustup:
   - https://rustup.rs
2. Make sure you’re on the right toolchain:
   - rustup default stable
   - rustup update
3. Clone the repository:
   - git clone https://github.com/Northwind-Software/hanabi.git
   - cd hanabi
4. Build:
   - cargo build
5. Run (example binary):
   - cargo run -p hanabi
6. Test:
   - cargo test --all
7. Lint with clippy:
   - cargo clippy --all-targets --all-features -- -D warnings
8. Format:
   - cargo fmt --all

Tip: Run formatting and clippy before committing to keep CI green.

## Coding standards and tooling
- Formatting: Use rustfmt (cargo fmt --all) with the default style unless the repository includes a rustfmt.toml.
- Linting: Use clippy (cargo clippy ...) and fix warnings; treat clippy warnings as errors (-D warnings).
- Safety: Prefer safe Rust. If unsafe is necessary, isolate it and document invariants.
- Tests: Add or update tests alongside your changes. Keep tests fast and deterministic.
- Documentation: Public APIs should have doc comments. Update README and inline docs if behavior changes.

## Git, branches, and commit messages
- Workflow: Use feature branches from the latest main.
  - naming suggestion: feature/<short-description>, fix/<short-description>, docs/<short-description>
- Keep commits small and logically scoped. Rebase/squash to tidy history before opening a PR if needed.
- Commit messages: Prefer Conventional Commits where practical, e.g.:
  - feat: add topic-based routing to broker
  - fix: handle empty payloads without panicking
  - docs: update README with build instructions
  - test: add benchmarks for publish latency
  - refactor: extract connection pool builder
  - perf: reduce lock contention in dispatcher
  - chore: bump deps and rust-toolchain

Include a short subject line (<= 72 chars) and a helpful body if context is needed (what/why, not just how).

## Opening a Pull Request (PR)
Before submitting:
- Ensure builds, tests, formatting, and clippy all pass locally.
- Update docs, changelogs (if applicable), and examples.
- Verify that it works on Linux.
- Link the related issue with keywords (e.g., "Fixes #123").

PR checklist:
- [ ] Description of change and rationale
- [ ] Tests added/updated or explanation why not needed
- [ ] Docs updated (README, module docs, examples)
- [ ] cargo fmt and cargo clippy pass

Small, focused PRs are easier to review and merge.

## Code review process
- One or more maintainers will review your PR.
- Be responsive to review comments; follow-up commits are preferred over force-push during review. Squash before merge if requested.
- Reviews aim to ensure correctness, performance, maintainability, and consistency with a project direction.

## Issue triage and labels
If you help triage issues:
- Confirm reproducibility and add steps to reproduce if missing.
- Apply labels such as bug, enhancement, good first issue, help wanted.
- Close duplicates with a reference to the canonical issue.

Contributors are welcome to propose labels and improvements to the triage process.

## Security and responsible disclosure
If you discover a security vulnerability, please do not open a public issue.

Instead, email: lucas@znth.systems

We will acknowledge receipt within a reasonable timeframe and work with you on a coordinated disclosure. Please provide clear steps to reproduce and any potential impact assessment.

## Code of Conduct
We follow the spirit of the Contributor Covenant. Be respectful and inclusive. Harassment and personal attacks are not tolerated.

If conduct concerns arise, contact lucas@znth.systems.

## License and contributor certification
By contributing, you agree that your contributions will be dual-licensed under the terms of this project’s license: MIT or Apache-2.0, at your option, as noted in the repository.

If you submit code that you did not write, you must ensure it is compatible with the project license(s) and that you have the right to contribute it.

## Acknowledgements
Thank you for helping make Hanabi better! Your time and expertise are greatly appreciated.
