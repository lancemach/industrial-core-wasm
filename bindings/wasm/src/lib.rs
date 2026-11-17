//! Thin Browser WASM adapter over engine-facade.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn engine_version() -> String {
    engine_facade::Engine::new().info().version.to_owned()
}

#[wasm_bindgen]
pub fn validate_finite(value: f64) -> Result<(), JsValue> {
    engine_facade::Engine::new()
        .validate_finite(value)
        .map_err(|error| JsValue::from_str(&error.to_string()))
}
