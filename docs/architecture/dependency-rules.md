# Dependency rules

```text
core-types
   ↑
foundation crates (errors, units, precision, validation, version)
   ↑
protocol / sensor / domain / data engines
   ↑
engine-facade
   ↑
WASM and NAPI bindings
```

Edges point from consumers to dependencies. The graph must remain acyclic. Foundation crates may share `core-types`; domain crates may use foundations; facade may compose domain crates. Runtime bindings may depend only on `engine-facade` and their runtime SDK. Core and domain crates must not depend on wasm-bindgen, NAPI-RS, Node, browser APIs, application packages, or hardware transports. Applications depend on SDKs and must not become dependencies of core crates.
