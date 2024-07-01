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

#![deny(
    warnings,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::restriction,
    clippy::cargo
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::blanket_clippy_restriction_lints,
    clippy::missing_inline_in_public_items,
    clippy::single_char_lifetime_names,
    clippy::implicit_return,
    clippy::pattern_type_mismatch,
    clippy::question_mark_used,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::pub_with_shorthand,
    clippy::absolute_paths,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::multiple_crate_versions,
    clippy::missing_docs_in_private_items,
    clippy::pub_use,
    clippy::infinite_loop, // Allowed because of bug: https://github.com/rust-lang/rust-clippy/issues/12338
    clippy::unseparated_literal_suffix,
    clippy::self_named_module_files,
    clippy::big_endian_bytes,
    clippy::single_call_fn,
    clippy::missing_trait_methods,
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    clippy::print_stdout,
    clippy::shadow_unrelated,
    clippy::undocumented_unsafe_blocks,
    clippy::as_conversions,
    clippy::ref_as_ptr,
    clippy::doc_markdown,
    clippy::unwrap_used,
    clippy::unreachable,
    clippy::impl_trait_in_params,
    clippy::missing_errors_doc,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::alloc_instead_of_core,
    clippy::min_ident_chars
)]
#![forbid(unreachable_pub, missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "fs")]
#[cfg_attr(docsrs, doc(cfg(feature = "fs")))]
pub mod fs;
#[cfg(feature = "net")]
#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
pub mod net;
#[cfg(feature = "process")]
#[cfg_attr(docsrs, doc(cfg(feature = "process")))]
pub mod process;
#[cfg(feature = "rt")]
#[cfg_attr(docsrs, doc(cfg(feature = "rt")))]
pub mod task;
#[cfg(feature = "time")]
mod time;

/// Struct representing tokio global runtime usage.
pub struct TokioGlobalRuntime;
