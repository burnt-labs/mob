# Resolve dependency vulnerabilities

1. Audit the Rust, npm, and Python dependency surfaces because the repository Dependabot endpoint is unavailable.
2. Remove the unused CosmRS HTTP RPC feature that activates the vulnerable Reqwest 0.11 Rustls stack.
3. Regenerate the Rust graph and verify it against the current RustSec advisory database.
4. Run formatting, lint, and test checks before opening a focused pull request.
