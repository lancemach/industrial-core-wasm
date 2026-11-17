# Toolchain selection record

Versions recorded during initialization on 2026-11-17:

| Tool | Selected / detected | Reason and status |
| --- | --- | --- |
| Rust | 1.100.0 detected; MSRV 1.88 | Rust 2024 edition requires Rust 1.85 or newer; NAPI-RS v3 requires 1.88. `wasm32-unknown-unknown` and Windows x64 MSVC targets are installed. |
| Node.js | v26.13.0 default; v24.22.1 used for NAPI load test | Both are installed through Volta. Node 26 is LTS per the [official release schedule](https://nodejs.org/en/about/previous-releases); Node 24 load was separately tested. |
| pnpm | v12.12.1 detected and pinned | Current pnpm 12 line; workspace manager version is pinned for reproducibility. |
| TypeScript | 6.x selected by package range | Current stable major according to [TypeScript 6.0 release notes](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-6-0.html). |
| Vitest | 5.x selected by package range | Current stable major; requires Vite 6.4+ and Node 22.12+, compatible with selected Node and Vite 8. See [Vitest 5 migration guide](https://vitest.dev/guide/migration/). |
| Vite | 8.x selected by package range | Current stable major; [Vite blog](https://vite.dev/blog) lists 8.1 as the latest release at initialization. |
| Vue plugin | 6.x selected by package range | Registry metadata verified `@vitejs/plugin-vue` 6.0.11 supports Vue 3 and Vite 8. |
| NAPI Rust crates | `napi` 3.16.2, `napi-derive` 3.7.1, `napi-build` 2.6.0 | Resolved together from crates.io in `Cargo.lock`; the current [NAPI crate lockfile](https://docs.rs/crate/napi/latest/source/Cargo.lock) also resolves `napi-build` 2.6.0. |
| NAPI CLI | `@napi-rs/cli` 3.10.12 | Separate npm build tool, not the Rust `napi-build` crate. Official [NAPI-RS v3 migration guide](https://napi.rs/docs/more/v2-v3-migration-guide) documents `binaryName`/`targets` and the Rust 1.88 minimum. Windows x64 build and Node 24 load/error tests passed. |
| wasm-bindgen | 0.2.x selected in workspace manifest | Current compatible crate line; actual WASM build remains unverified. |

JavaScript dependency ranges are semver-major bounded; `pnpm-lock.yaml` records the exact workspace resolutions. `Cargo.lock` records exact Rust dependency resolutions. Node.js version evidence: [official Node.js releases](https://nodejs.org/en/about/previous-releases). pnpm line evidence: [pnpm installation](https://pnpm.io/installation). Vitest compatibility evidence: [Vitest migration guide](https://vitest.dev/guide/migration/).
