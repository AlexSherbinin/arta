//! # arta-tokio
//! Arta-tokio is a crate that provides an implementation of `arta` abstractions for Tokio runtime.
//!
//! ## Installation
//! Add a following dependencies to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! arta-tokio = "0.1.0"
//! arta = "0.1.0"
//! tokio = { version = "^1", features = ["full"] }
//! ```
//! ## Usage
//!
//! ```ignore
//! #[tokio::main]
//! async fn main() {
//!     // After tokio runtime was initialized just call methods on `TokioGlobalRuntime` to use
//!     // this crate.
//!     // Example:
//!     let hosts = TokioGlobalRuntime.read_to_string("/etc/hosts").await.unwrap();
//!     println!("Hosts: {hosts}");
//! }
//! ```

pub mod fs;
pub mod net;
pub mod process;
pub mod task;
mod time;

/// Struct representing tokio global runtime usage.
pub struct TokioGlobalRuntime;
