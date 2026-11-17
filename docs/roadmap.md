# Implementation roadmap

Each stage is a gate. Stop after its acceptance review; proceed only on PASS or manually approved PASS WITH FINDINGS.

0. Project initialization and architecture baseline.
1. Core Foundation: types, errors, units, precision, validation, version.
2. Engine Facade: lifecycle, capabilities, version, unified errors.
3. Protocol Engine: Modbus RTU/TCP, CRC, register conversion, standard vectors.
4. Sensor Engine: sourced temperature, humidity, vapor pressure, and dew point formulas.
5. Domain Engine: review existing SF6 implementation, inventory formulas, approve vectors, migrate and compare.
6. Data Engine: validation, mapping, batch operations, quality flags, timestamp helpers.
7. Browser WASM: wasm-bindgen adapter, SDK integration, Vue playground.
8. Node Native: NAPI-RS adapter and platform builds.
9. Node WASM/WASI: select and validate target/runtime before implementing loader.
10. Unified TypeScript SDK: consistent runtime selection, API, errors, and lifecycle.
11. Native/FFI/cross-platform quality: C ABI evaluation, consistency, fuzzing, benchmarks.
12. v0.1.0 release: documentation, license inventory, CI, artifacts, integration examples.

This initialization is Stage 0 scaffolding only; formal Stage 0 acceptance remains pending validation and review.
