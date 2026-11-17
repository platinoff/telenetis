//! Test-scratch root for unit/integration tests.
//!
//! The owner canon: never use the Windows %TEMP% dir for scratch (it is
//! the most heavily EDR-scanned location on this box and breaks
//! deterministic unit tests). Test fixtures live under `target/test-tmp/`
//! inside the repo instead — cleaned by `cargo clean`, invisible to the
//! production runtime.
#![cfg(test)]

use std::path::{Path, PathBuf};

/// Repo-local scratch root. Drop-in replacement for
/// `std::env::temp_dir().join(p)` — joins onto `target/test-tmp/`.
pub(crate) fn test_tmp(p: impl AsRef<Path>) -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("test-tmp");
    let _ = std::fs::create_dir_all(&root);
    root.join(p)
}
