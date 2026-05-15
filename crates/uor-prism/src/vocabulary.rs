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
//! // When:  the wire-format version and the canonical `HostBounds`
//! //        defaults are read
//! // Then:  they match the foundation's normative values verbatim
//! use prism::vocabulary::{DefaultHostBounds, HostBounds, TRACE_REPLAY_FORMAT_VERSION};
//! assert_eq!(<DefaultHostBounds as HostBounds>::FINGERPRINT_MIN_BYTES, 16);
//! assert_eq!(<DefaultHostBounds as HostBounds>::FINGERPRINT_MAX_BYTES, 32);
//! assert_eq!(<DefaultHostBounds as HostBounds>::TRACE_MAX_EVENTS, 256);
//! assert_eq!(<DefaultHostBounds as HostBounds>::WITT_LEVEL_MAX_BITS, 64);
//! assert_eq!(TRACE_REPLAY_FORMAT_VERSION, 8);
//! ```
//!
//! [05-prism]: https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism

// UOR-domain sealed types (the foundation's "Layer 1: Opaque witnesses").
pub use uor_foundation::enforcement::{Datum, FreeRank, Triad};

// Substitution-axis traits — two of the three axes named in ADR-007.
// `HostTypes` carries the three host-side type slots; `HostBounds` carries
// the four capacity bounds (`FINGERPRINT_MIN_BYTES`, `FINGERPRINT_MAX_BYTES`,
// `TRACE_MAX_EVENTS`, `WITT_LEVEL_MAX_BITS`) the principal data path
// const-generic instantiations resolve against. ADR-018 ratified
// `HostBounds` as a first-class substitution axis (capacity completeness),
// so the (HostTypes, HostBounds, Hasher) triple is now the full
// substitution-axis surface. The third axis, `Hasher`, is below in the
// substrate-hasher block.
pub use uor_foundation::{DefaultHostBounds, DefaultHostTypes, HostBounds, HostTypes};

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

// Errors and impossibility witnesses (Error Model § of wiki page 08).
pub use uor_foundation::enforcement::GenericImpossibilityWitness;
pub use uor_foundation::{Derivation, ReplayError, ShapeViolation};

// Wire-format version constant. The capacity constants
// (`FINGERPRINT_MIN_BYTES`, `FINGERPRINT_MAX_BYTES`, `TRACE_MAX_EVENTS`)
// are no longer free — they are associated consts on `HostBounds`,
// reachable as `<DefaultHostBounds as HostBounds>::FINGERPRINT_MAX_BYTES`
// and so on. Selecting a different `HostBounds` impl rescales them
// without code changes.
pub use uor_foundation::TRACE_REPLAY_FORMAT_VERSION;

// Foundation-owned closed enums and ordinals: the Witt-level family and
// the verification-domain family are part of the bilateral compile-time
// contract (TC-04) and are surfaced here so consumers that want to
// `use prism::vocabulary::*;` reach them in one import.
pub use uor_foundation::{Space, VerificationDomain, WittLevel};
