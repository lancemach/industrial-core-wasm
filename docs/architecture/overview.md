# Architecture overview

Industrial Core keeps deterministic computation in runtime-free Rust crates. `engine-facade` is the only core API consumed by runtime bindings. Bindings translate values, lifecycle operations, and errors; the TypeScript SDK presents a common JavaScript contract.

```text
Applications (Vue / Node / Electron / gateway)
                 ↓
       TypeScript SDK (optional)
          ↙              ↘
 Browser WASM         Node Native
          ↘              ↙
            engine-facade
                  ↓
  protocol / sensor / domain / data
                  ↓
       core foundation crates
```

Core modules contain no UI, runtime, network, serial, or hardware access. Domain crates may depend on foundation crates; they must not depend on bindings or apps. The current crates are initialization skeletons and intentionally contain no industrial calculation algorithms.
