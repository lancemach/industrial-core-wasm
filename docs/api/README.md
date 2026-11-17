# API documentation

The initialization smoke API exposes engine version metadata and finite-number validation through `engine-facade`. The NAPI and WASM adapters map invalid finite-number inputs to their runtime error types; this is infrastructure validation, not an industrial domain algorithm. Generate Rust API docs with `cargo doc --workspace --no-deps`.
