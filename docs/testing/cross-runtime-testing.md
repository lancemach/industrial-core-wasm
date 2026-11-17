# Cross-runtime testing

Maintain one language-neutral fixture set for each public facade operation. Run identical vectors through direct Rust Native calls, Browser WASM, and Node Native; compare structured validity/error states exactly and numeric values against the algorithm's documented absolute/relative tolerance. Include malformed inputs and runtime load failures. Node WASM/WASI comparisons begin only after a concrete target and host API are chosen. A forced runtime option must fail when its runtime is unavailable; it must not silently exercise another implementation.
