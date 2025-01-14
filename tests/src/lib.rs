#![cfg_attr(test, feature(internal_output_capture))]
//! Namada integrations and WASM tests and testing helpers.

#![doc(html_favicon_url = "https://dev.namada.net/master/favicon.png")]
#![doc(html_logo_url = "https://dev.namada.net/master/rustdoc-logo.png")]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]

pub use namada;

mod vm_host_env;
pub use vm_host_env::{ibc, tx, vp};
#[cfg(test)]
mod e2e;
#[cfg(test)]
#[allow(dead_code)]
mod integration;
pub mod native_vp;
pub mod storage;
#[cfg(test)]
mod storage_api;

/// Using this import requires `tracing` and `tracing-subscriber` dependencies.
/// Set env var `RUST_LOG=info` to see the logs from a test run (and
/// `--nocapture` if the test is not failing).
pub mod log {
    pub use test_log::test;
}
