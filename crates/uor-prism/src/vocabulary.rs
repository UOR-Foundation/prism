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
//! Per wiki ADR-060 the foundation ships **no** `DefaultHostBounds`:
//! "there is no 'default' application, so the foundation supplies no
//! default policy. Every application declares its own `impl
//! HostBounds`." The standard library re-exports the [`HostBounds`]
//! trait so application authors declare their capacity policy
//! explicitly; the per-carrier byte widths derive from the declared
//! structural-count primitives via foundation `const fn`s, with no
//! application-chosen byte-width literals.
//!
//! ```rust
//! // Given: the curated vocabulary surface
//! // When:  an application declares its own `HostBounds` policy
//! // Then:  the trait + wire-format version resolve through the façade
//! use prism::vocabulary::{HostBounds, TRACE_REPLAY_FORMAT_VERSION};
//!
//! struct MyBounds;
//! impl HostBounds for MyBounds {
//!     const FINGERPRINT_MIN_BYTES: usize = 16;
//!     const FINGERPRINT_MAX_BYTES: usize = 32;
//!     const TRACE_MAX_EVENTS: usize = 256;
//!     const WITT_LEVEL_MAX_BITS: u32 = 64;
//!     const FOLD_UNROLL_THRESHOLD: usize = 8;
//!     const BETTI_DIMENSION_MAX: usize = 8;
//!     const NERVE_CONSTRAINTS_MAX: usize = 8;
//!     const NERVE_SITES_MAX: usize = 8;
//!     const JACOBIAN_SITES_MAX: usize = 8;
//!     const RECURSION_TRACE_DEPTH_MAX: usize = 16;
//!     const OP_CHAIN_DEPTH_MAX: usize = 8;
//!     const AFFINE_COEFFS_MAX: usize = 8;
//!     const CONJUNCTION_TERMS_MAX: usize = 8;
//!     const UNFOLD_ITERATIONS_MAX: usize = 256;
//! }
//! assert_eq!(<MyBounds as HostBounds>::FINGERPRINT_MAX_BYTES, 32);
//! assert_eq!(TRACE_REPLAY_FORMAT_VERSION, 10);
//! ```
//!
//! [05-prism]: https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism

// UOR-domain sealed types (the foundation's "Layer 1: Opaque witnesses").
pub use uor_foundation::enforcement::{Datum, FreeRank, Triad};

// Substitution-axis traits — two of the three axes named in ADR-007.
// `HostTypes` carries the three host-side type slots; `HostBounds` carries
// the 14 capacity primitives (the 4 pre-ADR-018 bounds
// `FINGERPRINT_MIN_BYTES`, `FINGERPRINT_MAX_BYTES`, `TRACE_MAX_EVENTS`,
// `WITT_LEVEL_MAX_BITS` plus the 10 structural-count caps) the principal
// data path const-generic instantiations resolve against. ADR-018
// ratified `HostBounds` as a first-class substitution axis (capacity
// completeness), so the (HostTypes, HostBounds, Hasher) triple is the
// full substitution-axis surface. The third axis, `Hasher`, is below in
// the substrate-hasher block.
//
// Per ADR-060 the foundation ships NO `DefaultHostBounds` — there is no
// "default" application, so the standard library re-exports only the
// `HostBounds` trait, and application authors declare their own impl
// (every honored constant traces to an explicit application
// declaration; per-carrier byte widths derive from these primitives via
// foundation `const fn`s, never a pinned literal). `DefaultHostTypes`
// is retained: the host-type slots have a canonical foundation identity
// (it is not a capacity-policy default).
pub use uor_foundation::{DefaultHostTypes, HostBounds, HostTypes};

// Builders, declarations, and validation results.
pub use uor_foundation::{
    BindingEntry, BindingsTable, BindingsTableError, BoundConstraint, Calibration,
    CalibrationError, CompileUnit, CompileUnitBuilder,
};

// `Binding` is the public-fielded input-slot binding an application
// constructs to content-address a value into a `CompileUnit`'s
// binding table (`CompileUnitBuilder::bindings`). Its `content_address`
// is a `u64` the application computes — for a value that fits the
// ADR-060 inline carrier, by serializing through `IntoBindingValue`;
// for a **large input** that exceeds `carrier_inline_bytes::<B>()`, by
// stream-hashing the full input through the application's `Hasher`
// (`fold_bytes`, chunk-by-chunk, never materialized) and taking the
// leading-8-byte big-endian digest. The latter is the uncapped
// large-input grounding path that the convenience `run_route` cap does
// not expose — see `tests/large_input_grounding.rs`.
pub use uor_foundation::enforcement::Binding;

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
// are associated consts on `HostBounds`, reachable as
// `<MyBounds as HostBounds>::FINGERPRINT_MAX_BYTES` on the
// application's own impl (no foundation default exists per ADR-060).
// Selecting a different `HostBounds` impl rescales them without code
// changes.
pub use uor_foundation::TRACE_REPLAY_FORMAT_VERSION;

// Foundation-owned closed enums and ordinals: the Witt-level family and
// the verification-domain family are part of the bilateral compile-time
// contract (TC-04) and are surfaced here so consumers that want to
// `use prism::vocabulary::*;` reach them in one import.
pub use uor_foundation::{Space, VerificationDomain, WittLevel};
