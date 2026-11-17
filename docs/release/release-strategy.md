# Release strategy

The Cargo workspace uses one shared package version. Public Rust and TypeScript APIs follow semantic versioning; domain algorithms have independent IDs and versions. Release records include Git commit, toolchain, target, and build metadata. Publish Rust crates, the TypeScript SDK, WASM bundles, and platform-specific NAPI packages as separate artifacts with dependency license inventories and Apache-2.0 notices. Do not publish placeholder bindings as supported runtime packages.
