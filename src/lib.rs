//! A simple Tuple Space implementation for Rust.

/// Tuple Space specific Error
pub mod error;
/// Thread safe Tuple Space store wrapper
pub mod mutex_store;
pub mod query_tuple;
pub mod query_types;
/// Tuple Space specific Result
pub mod result;
/// Trait required to be a Tuple store
pub mod store;
/// Storage unit for the Tuple Space
pub mod tuple;
/// Types that can be stored in a Tuple
pub mod types;
/// Simple Vec based Tuple Space store
pub mod vec_store;
