//! `prism_verify` — the Prism replay façade.
//!
//! This crate is the Rust realization of the **`prism-verify`** container of
//! the Prism system specified by the [UOR-Framework wiki][wiki]. It is a
//! thin verification surface that re-exports `certify_from_trace` and
//! `Certified` from [`prism`], together with the trace and certificate
//! wire-format type definitions from [`uor_foundation`]. Verification
//! consumers depend on this crate alone, never on the runtime.
//!
//! The crate is published to crates.io under the package name
//! [`uor-prism-verify`](https://crates.io/crates/uor-prism-verify); the
//! library name is `prism_verify` so that import paths track wiki
//! nomenclature (`use prism_verify::certify_from_trace;`).
//!
//! # See also
//!
//! - [Wiki: 01 Introduction and Goals](https://github.com/UOR-Foundation/UOR-Framework/wiki/01-Introduction-and-Goals)
//! - [Wiki: 03 Context and Scope](https://github.com/UOR-Foundation/UOR-Framework/wiki/03-Context-and-Scope)
//! - [Wiki: 05 Building Block View § Whitebox `prism-verify`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism-verify)
//! - [Wiki: 12 Glossary](https://github.com/UOR-Foundation/UOR-Framework/wiki/12-Glossary)
//!
//! # Constraints
//!
//! This crate is normatively bound by:
//!
//! - **TC-05** — replayability of the principal data path without invoking
//!   author deciders or hash functions; this façade is the user-facing
//!   surface of that property
//! - **TC-06** — verification proceeds without any application-author
//!   infrastructure
//! - **QS-03** — local verification: this crate is the dependency
//!   verification consumers pin, exposing nothing beyond the surface
//!   needed to re-derive a `Certified<GroundingCertificate>` from a `Trace`
//!
//! # C4 placement
//!
//! Container `prism-verify` (Level 2) of the Prism system. The crate's
//! components mirror the Level 2 building blocks described in the wiki's
//! [Building Block View § Whitebox `prism-verify`][05-verify]: the
//! re-export of `certify_from_trace`, the re-export of `Certified`, and the
//! re-exports of foundation wire-format types.
//!
//! # Behavior
//!
//! ```rust
//! // Given: prism_verify is loaded
//! // When:  the runtime crate `prism` is reachable through it
//! // Then:  consumers can resolve the wiki origin via either entry point
//! use prism as _;
//! use uor_foundation as _;
//! assert_eq!(prism_verify::WIKI, prism::WIKI);
//! ```
//!
//! [wiki]: https://github.com/UOR-Foundation/UOR-Framework/wiki
//! [05-verify]: https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism-verify

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use prism;
pub use uor_foundation;

/// Canonical URL of the UOR-Framework wiki, the normative source for the
/// Prism architecture realized by this façade.
///
/// Re-exported from [`prism::WIKI`] so that verification consumers who
/// depend on this façade alone can still surface the architectural origin
/// without a transitive dependency declaration.
///
/// # See also
///
/// - [Wiki: Home](https://github.com/UOR-Foundation/UOR-Framework/wiki)
///
/// # Constraints
///
/// - **CV-02** — code identifiers appear in monospace without paraphrase
///
/// # Behavior
///
/// ```rust
/// // Given: prism_verify is loaded
/// // When:  the wiki URL is queried through the façade
/// // Then:  it equals the same constant as on the runtime crate
/// assert_eq!(prism_verify::WIKI, prism::WIKI);
/// ```
pub const WIKI: &str = prism::WIKI;
