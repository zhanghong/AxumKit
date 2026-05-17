//! Kit Search - Search domain module for AxumKit
//!
//! This crate provides search functionality following DDD architecture.
//!
//! # Architecture Layers
//!
//! - **Domain Layer**: Core business logic (models, services, repository interfaces)
//! - **Application Layer**: Application services coordinating domain operations
//! - **API Layer**: HTTP handlers, routes, and DTOs
//! - **Infrastructure Layer**: Repository implementations and external adapters

pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;

// Re-export commonly used types
pub use api::dto::request::{SearchUsersRequest, SortOrder};
pub use api::dto::response::{SearchUsersResponse, UserSearchItem};
pub use api::handler::search_users;
pub use api::route::search_routes;
pub use api::route::SearchApiDoc;
pub use application::SearchApplicationService;
pub use infrastructure::adapter::MeilisearchClient;
