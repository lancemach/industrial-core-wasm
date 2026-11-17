# Industrial Core

Industrial Core is a runtime-independent Rust foundation for reusable industrial computation, with planned Browser WASM, Node Native, Rust Native, and later C ABI integrations. Initial domain areas are Modbus, environmental calculations, SF6 calculations, and industrial data processing. This initialization creates architecture and API skeletons only; it implements no industrial algorithms.

## Repository
- `crates/`: runtime-free foundation, domain module placeholders, and the engine facade.
- `bindings/`: thin WASM and Node Native adapter skeletons.
- `sdks/typescript/`: shared TypeScript contract and runtime loader interfaces.
- `apps/`: Vue/Vite playground and Node demo; Electron demo is reserved for a later stage.
- `docs/`: architecture, algorithm governance, development, testing, release, and roadmap.

## Quick start
Install Rust Stable with rustfmt, Clippy, and the `wasm32-unknown-unknown` target; install Node.js and pnpm. Then run:

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
```

See [Getting started](docs/development/getting-started.md) and [Build and test](docs/development/build-and-test.md).

## Licensing and contributions
Licensed under Apache-2.0; see [LICENSE](LICENSE) and [NOTICE](NOTICE). Third-party dependencies retain their own licenses and must be reviewed and included in distribution notices as required. Contributors retain copyright and license contributions under Apache-2.0; submissions must be authorized by their copyright holders. The project name and marks are not granted under the software license.
