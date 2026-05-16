//! Authentication route definitions.
//!
//! This module provides route configuration for all authentication endpoints.

mod auth_routes;

#[cfg(feature = "openapi")]
mod openapi;

pub use auth_routes::auth_routes;

#[cfg(feature = "openapi")]
pub use openapi::AuthApiDoc;
