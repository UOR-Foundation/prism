//! `prism_verify` — the Prism replay façade.
//!
//! This crate is the Rust realization of the **`prism-verify`** container
//! of the Prism system specified by the [UOR-Framework wiki][wiki]. It is
//! a thin verification surface that re-exports
//! [`certify_from_trace`] from [`prism`], [`Certified`] from `prism`,
//! and the trace and certificate wire-format types from
//! [`uor_foundation`]. Verification consumers depend on this crate
//! alone, never on the runtime; this preserves TC-06 (no
//! application-author infrastructure) and minimizes the verifier's
//! attack surface and dependency footprint.
//!
//! The façade is genuinely thin: every item in this crate's API is a
//! re-export of an item defined elsewhere in the architecture. The
//! crate adds zero behavior; it adds a *namespace*.
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
//! - [Wiki: 06 Runtime View § Scenario 2: Trace-Replay Verification](https://github.com/UOR-Foundation/UOR-Framework/wiki/06-Runtime-View#scenario-2-trace-replay-verification)
//! - [Wiki: 12 Glossary § Term Definitions](https://github.com/UOR-Foundation/UOR-Framework/wiki/12-Glossary#term-definitions)
//! - [Wiki: Conceptual Model § SD3 Verification](https://github.com/UOR-Foundation/UOR-Framework/wiki/Conceptual-Model#sd3-verification) — OPM statement of the verification process this façade enacts
//! - [Wiki: Conceptual Model § SD5 Distribute And Run](https://github.com/UOR-Foundation/UOR-Framework/wiki/Conceptual-Model#sd5-distribute-and-run) — `Verification` is the second of the two user-handled processes in SD5 (after `Execution`); this façade is the user-side surface that realizes it
//!
//! # Constraints
//!
//! This crate is normatively bound by:
//!
//! - **TC-05** — replayability of the principal data path without
//!   invoking author deciders or hash functions; this façade is the
//!   user-facing surface of that property
//! - **TC-06** — verification proceeds without any application-author
//!   infrastructure
//! - **QS-03** — local verification: this crate is the dependency
//!   verification consumers pin, exposing nothing beyond the surface
//!   needed to re-derive a `Certified<GroundingCertificate>` from a
//!   `Trace`
//! - **QS-05** — replay equivalence: the round-trip produces a
//!   bit-identical certificate
//! - **ADR-019** — this façade exposes the **anamorphism** dual to
//!   `pipeline::run`'s catamorphism. Together the catamorphism +
//!   anamorphism form Prism's hylomorphism (per ADR-021), and the
//!   trace is the round-trip witness object
//!
//! # C4 placement
//!
//! Container `prism-verify` (Level 2) of the Prism system. Its
//! components mirror the Level 2 building blocks described in the
//! wiki's [Building Block View § Whitebox `prism-verify`][05-verify]:
//! the re-export of `certify_from_trace`, the re-export of `Certified`,
//! and the re-exports of foundation wire-format types.
//!
//! # Behavior
//!
//! ```rust
//! // Given: an empty Trace (the simplest deterministic verifier input)
//! // When:  certify_from_trace is invoked on it
//! // Then:  the structural validator rejects with ReplayError::EmptyTrace,
//! //        proving that the façade's certify_from_trace, ReplayError,
//! //        and Trace re-exports are wired correctly together
//! use prism_verify::{certify_from_trace, ReplayError, Trace};
//! let trace: Trace = Trace::empty();
//! assert!(matches!(certify_from_trace(&trace), Err(ReplayError::EmptyTrace)));
//! ```
//!
//! [wiki]: https://github.com/UOR-Foundation/UOR-Framework/wiki
//! [05-verify]: https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism-verify

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use prism;
pub use uor_foundation;

// The verifier API: one function and its companion result types.
pub use prism::replay::certify_from_trace;
pub use prism::seal::Certified;

// Wire-format types the verifier consumes and emits, plus the
// substitution axes a verifier instantiates them at. `HostBounds`
// carries the capacity constants that used to be free `pub const`s in
// foundation 0.3.0 (`TRACE_MAX_EVENTS` is now
// `<B as HostBounds>::TRACE_MAX_EVENTS`). Per ADR-060 the foundation
// ships no `DefaultHostBounds`; a verifier declares its own
// `impl HostBounds` (matching the producer's capacity policy) — the
// `HostBounds` trait is re-exported for that purpose.
pub use uor_foundation::{
    ContentFingerprint, GroundingCertificate, HostBounds, ReplayError, Trace, TraceEvent,
    TRACE_REPLAY_FORMAT_VERSION,
};

/// Canonical URL of the UOR-Framework wiki, the normative source for the
/// Prism architecture realized by this façade.
///
/// Re-exported from [`prism::WIKI`] so that verification consumers who
/// depend on this façade alone can still surface the architectural
/// origin without a transitive dependency declaration.
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
