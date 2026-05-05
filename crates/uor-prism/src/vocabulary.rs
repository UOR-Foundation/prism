//! Foundation surface re-exports — the single-import vocabulary.
//!
//! `vocabulary` realizes the wiki's
//! [Building Block View § Whitebox `prism`][05-prism] component named
//! "vocabulary re-exports": the broad foundation surface a `prism`
//! consumer can reach by `use prism::vocabulary::*;` instead of
//! depending on `uor-foundation` directly. Per ADR-013, every `prism`
//! type ultimately derives from foundation; this module is the
//! convenience entry point.
//!
//! The re-exports are deliberately curated rather than wildcarded: this
//! module is the visible API contract of the `prism` crate, and a
//! wildcard would silently grow with the substrate.
//!
//! # See also
//!
//! - [Wiki: 05 Building Block View § Whitebox `prism`][05-prism]
//! - [Wiki: 08 Concepts § Closure Under uor-foundation](https://github.com/UOR-Foundation/UOR-Framework/wiki/08-Concepts#closure-under-uor-foundation)
//! - [Wiki: 09 Architecture Decisions § ADR-013](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
//! - [Wiki: 12 Glossary § Term Definitions](https://github.com/UOR-Foundation/UOR-Framework/wiki/12-Glossary#term-definitions)
//!
//! # Constraints
//!
//! - **TC-04** — bilateral compile-time enforcement is preserved: every
//!   re-exported type retains the foundation's sealing and trait-bound
//!   discipline at the call site
//! - **ADR-013** — closure of `prism` under `uor-foundation`: this
//!   module is the operational surface of that closure
//!
//! # C4 placement
//!
//! Component `vocabulary re-exports` (Level 3) inside container `prism`
//! (Level 2). Anything the foundation exposes that does not naturally
//! live in [`crate::pipeline`], [`crate::seal`], [`crate::replay`],
//! [`crate::operation`], or [`crate::std_types`] is collected here so
//! consumers do not need to learn the `uor-foundation` namespace to
//! make incidental use of its types.
//!
//! # Behavior
//!
//! ```rust
//! // Given: the curated vocabulary surface
//! // When:  the constants describing wire-format and trace bounds are read
//! // Then:  they match the foundation's normative values verbatim
//! use prism::vocabulary as v;
//! assert_eq!(v::FINGERPRINT_MIN_BYTES, 16);
//! assert_eq!(v::FINGERPRINT_MAX_BYTES, 32);
//! assert_eq!(v::TRACE_MAX_EVENTS, 256);
//! assert_eq!(v::TRACE_REPLAY_FORMAT_VERSION, 2);
//! ```
//!
//! [05-prism]: https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism

// UOR-domain sealed types (the foundation's "Layer 1: Opaque witnesses").
pub use uor_foundation::enforcement::{Datum, FreeRank, Triad};

// Substitution-axis traits (HostTypes is one of the three axes per ADR-007).
pub use uor_foundation::{DefaultHostTypes, HostTypes};

// Builders, declarations, and validation results.
pub use uor_foundation::{
    BindingEntry, BindingsTable, BindingsTableError, BoundConstraint, Calibration,
    CalibrationError, CompileUnit, CompileUnitBuilder,
};

// Address, fingerprint, and the substrate hasher contract.
pub use uor_foundation::{ContentAddress, ContentFingerprint, Hasher};

// Certificate kinds.
pub use uor_foundation::{
    Certificate, CertificateKind, GroundingCertificate, MultiplicationCertificate,
};

// UOR-time and thermodynamic accounting.
pub use uor_foundation::{LandauerBudget, Nanos, UorTime};

// Trace wire format (the verifier's input).
pub use uor_foundation::{Trace, TraceEvent};

// Errors.
pub use uor_foundation::{Derivation, ReplayError, ShapeViolation};

// Normative constants of the wire formats and bounded structures.
pub use uor_foundation::{
    FINGERPRINT_MAX_BYTES, FINGERPRINT_MIN_BYTES, TRACE_MAX_EVENTS, TRACE_REPLAY_FORMAT_VERSION,
};

// Foundation-owned closed enums and ordinals: the Witt-level family and
// the verification-domain family are part of the bilateral compile-time
// contract (TC-04) and are surfaced here so consumers that want to
// `use prism::vocabulary::*;` reach them in one import.
pub use uor_foundation::{Space, VerificationDomain, WittLevel};
