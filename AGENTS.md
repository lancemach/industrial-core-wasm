# Industrial Core contributor instructions

## Project and architecture
Industrial Core is a runtime-independent industrial computing engine. Rust core and domain crates own computation. `engine-facade` is the sole public Rust entry point for bindings. Browser WASM, Node Native, and future runtimes are adapters; they must not contain business algorithms. Keep hardware access and transport (serial, sockets, polling) outside the computation core.

## Rust and workspace
- Use the workspace Stable toolchain, edition and shared dependency declarations. Keep crates small, independently checkable, and acyclic.
- Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and `cargo check --workspace` before proposing Rust changes.
- Prefer explicit types and fallible APIs. Do not hide failures behind default values or panics for invalid user input.
- Keep core crates free of WASM, NAPI, Node, browser, and application dependencies.

## TypeScript
- Use pnpm workspaces, strict TypeScript, explicit public types, and the SDK as the single cross-runtime contract.
- Runtime loaders adapt the facade API and map errors; they do not duplicate algorithms.
- Run `pnpm install --frozen-lockfile`, `pnpm lint`, `pnpm typecheck`, `pnpm test`, and `pnpm build` where applicable.

## API and runtime boundaries
- Bindings may call only `engine-facade`; keep data conversion and lifecycle handling explicit.
- Preserve API compatibility after publication; use semantic versioning and document breaking changes.
- Report runtime capabilities accurately. Do not claim WASI compatibility without validating its target and host APIs.

## Industrial algorithms
- Never invent SF6 or other professional formulas. Require a traceable source, formula, units, input domain, operating assumptions, algorithm ID/version, precision tolerance, and reference vectors before implementation.
- Reject invalid inputs explicitly. Every computation result must carry a validity state; errors must distinguish invalid input from unsupported or failed computation.
- Never claim performance gains without reproducible benchmarks. Do not add SIMD or concurrency without correctness and benchmark evidence.

## Testing and errors
- Test public APIs, boundary values, malformed input, and known standard/reference vectors. Protocol implementations require authoritative standard vectors; floating-point tests specify tolerances.
- Fuzz parsers when introduced. Keep errors structured and serializable at runtime boundaries; never silently ignore invalid data.

## Documentation and releases
- Update architecture, API, and algorithm docs with behavior changes. Record actual tool versions and actual validation results.
- Use Apache-2.0-compatible contributions. Contributors retain copyright and grant the project the rights required by Apache-2.0; review third-party code licenses. Project trademarks are not granted by the software license.
- Keep workspace package versions aligned; track algorithm versions separately from crate/package versions. Release artifacts must record commit and build metadata.

## Git
- Use concise imperative commit subjects with a clear scope. Do not commit generated artifacts, secrets, or build outputs. Keep changes reviewable and never rewrite published history without authorization.
