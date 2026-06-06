//! Content loading and validation for Tracewake fixtures.

pub mod load;
pub mod manifest;
pub mod schema;
pub mod serialization;
pub mod validate;

pub fn core_boundary_marker() -> &'static str {
    "tracewake-content depends on tracewake-core"
}
