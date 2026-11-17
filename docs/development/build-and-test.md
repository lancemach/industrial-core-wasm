# Build and test

```powershell
pnpm install
pnpm lint
pnpm typecheck
pnpm test
pnpm build
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build -p industrial-core-wasm --target wasm32-unknown-unknown
```

The last command verifies the low-level target only. A production Browser package also requires wasm-bindgen packaging and browser integration checks. Node Native requires building and loading the `.node` artifact on each supported platform. Do not report either as complete without executing those checks.
