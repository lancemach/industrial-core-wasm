# Runtime design

| Runtime | Adapter | Intended use | Status |
| --- | --- | --- | --- |
| Browser | `wasm-bindgen`, `wasm32-unknown-unknown` | Web applications and WebViews | Skeleton only |
| Node Native | NAPI-RS | Node.js and NestJS | Skeleton only; build/load unverified |
| Node WASM / WASI | To be selected after host/target evaluation | Portable Node execution | Planned; no loader claimed |
| Rust Native | Direct `engine-facade` dependency | Rust services and gateways | Available as Rust library design |
| C ABI | Future dedicated adapter | Native integrations | Not implemented |

Every adapter delegates to `engine-facade`; only that facade and the TypeScript SDK define public API contracts. Runtime selection must be explicit and report load failures. Never silently fall back when a caller requested a specific runtime.
