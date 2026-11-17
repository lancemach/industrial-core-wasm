//! Thin Node Native adapter over engine-facade.

#[napi_derive::napi]
pub fn engine_version() -> String {
    engine_facade::Engine::new().info().version.to_owned()
}

#[napi_derive::napi]
pub fn validate_finite(value: f64) -> napi::Result<()> {
    engine_facade::Engine::new()
        .validate_finite(value)
        .map_err(|error| napi::Error::new(napi::Status::InvalidArg, error.to_string()))
}
