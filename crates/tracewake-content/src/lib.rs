//! Content loading and validation for Tracewake fixtures.

pub mod fixtures;

pub use tracewake_core::content::{load, manifest, schema, serialization, validate};

pub fn core_boundary_marker() -> &'static str {
    "tracewake-content depends on tracewake-core"
}
