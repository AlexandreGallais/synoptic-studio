//! The secret scan (EN-110): the rules of `@secretlint/secretlint-rule-preset-recommend` 13.0.7,
//! secretlint's recommended preset, carried into Rust with their comment filter, so that the
//! staged files at each commit and the tracked files in `check:all` are scanned without Node.js
//! (`docs/tooling/versions-and-security.md`, Secrets). Ported under the preset's MIT licence,
//! Copyright (c) 2020 azu (ADR-0076, `NOTICE`); its cases are the preset's own snapshots, in
//! `tests/fixtures/preset/`.
//!
//! Rust's `regex` has neither look-around nor backreference: [`pattern`] checks the preset's
//! look-arounds and [`js`] its one backreference and JavaScript's classes; the GCP rule's
//! PKCS #12 check reads the DER and recomputes the MAC with [`sha`]'s hashes.

pub mod base64;
pub mod comments;
pub mod js;
pub mod pattern;
pub mod pkcs12;
pub mod report;
pub mod rules;
pub mod sha;
pub mod source;

use report::Located;
use rules::Options;
use source::Source;

/// The findings of one file's bytes, with their positions.
#[must_use]
pub fn scan(path: &str, bytes: &[u8], options: Options) -> Vec<Located> {
    let source = Source::new(path, bytes);
    let findings = rules::scan(&source, options);

    if findings.is_empty() {
        return Vec::new();
    }
    report::locate(findings, &source.line_starts())
}
