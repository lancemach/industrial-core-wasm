# Getting started on Windows

1. Install Rust Stable through rustup (minimum supported version 1.88). The repository toolchain file requests `rustfmt`, `clippy`, and `wasm32-unknown-unknown`.
2. Install a supported Node.js LTS release and pnpm (the workspace records the pnpm version used to initialize it).
3. From the repository root run `pnpm install`, then use the commands in [Build and test](build-and-test.md).
4. To use Node Native bindings, install the platform's native build prerequisites and a compatible NAPI-RS CLI. Native compilation and loading remain unverified until these are available.

Verify with `rustc --version`, `cargo --version`, `node --version`, and `pnpm --version`. Do not infer installed prerequisites from this document.
