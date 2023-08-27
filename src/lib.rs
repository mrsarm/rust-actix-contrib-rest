//! Util types and functions for REST and webapp projects built on top of
//! the [Actix Web framework](https://actix.rs/).
//!
//! It does include structs and methods to:
//!
//! - Managing errors.
//! - Properly serialize errors, with a JSON response explaining the reason.
//! - Pagination and query search structs.
//! - Basic types for managing DB connections and transactions (`sqlx-postgres` feature).
//! - Basic methods to easily deals with streams and integration tests.

pub mod page;
pub mod query;
pub mod response;
pub mod result;
pub mod stream;
pub mod test;

#[cfg(feature = "sqlx-postgres")]
pub mod db;
