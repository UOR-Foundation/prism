//! Standard type library.
//!
//! `std_types` is `prism`'s realization of the wiki's
//! [Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
//! component named "standard type library" — the catalog of pre-declared
//! types built from `uor-foundation`'s vocabulary, available so
//! application authors do not have to derive common shape patterns
//! from first principles. Per ADR-017 the catalog is **canonical**: it
//! is the addressing surface that schema-import tools and applications
//! target so traces and certificates address consistently across the
//! ecosystem.
//!
//! The catalog is layered:
//!
//! - **Foundation-supplied surface (re-exports).** The ten morphism
//!   kinds (`BinaryGroundingMap`, …, `Utf8ProjectionMap`), the
//!   structural marker traits (`Total`, `Invertible`,
//!   `PreservesStructure`, `PreservesMetric`), the sealed
//!   `GroundedValue`/`GroundedShape` family, `ConstrainedTypeInput`,
//!   `CartesianProductShape` and its `kunneth_compose` helper, the
//!   partition-algebra families (`*Witness`, `*Evidence`,
//!   `*MintInputs`, `PartitionResolver`, `PartitionHandle`,
//!   `NullPartition`, `VerifiedMint`), and the `OntologyVerifiedMint`
//!   sealed mint trait.
//! - **First-class prism-defined surface.** [`FixedSites<N>`],
//!   [`Bytes<N>`], and the byte-aligned numeric / character / boolean
//!   primitives (`U8` … `I256`, `F32`, `F64`, `Bool`, `Char`).
//!
//! ## IRI rule (closure under `uor-foundation`)
//!
//! The IRI of every prism-defined stdlib type is **derived from its
//! constraint declaration, not from the Rust type name** — this is the
//! direct quote from
//! [Concepts § Closure Under uor-foundation][08-closure]
//! and the binding rule of ADR-017. Concretely: every prism stdlib
//! type with empty `CONSTRAINTS` shares the same IRI
//! (`https://uor.foundation/type/ConstrainedType`, the foundation's
//! ontology class for `ConstrainedTypeShape` instances). Instance
//! identity flows through `(SITE_COUNT, CONSTRAINTS)`, so distinct
//! site counts produce distinct content-addresses while same-shape
//! Rust types (e.g., `U32` and `I32`) produce **identical**
//! content-addresses by design — the Rust name is for the developer,
//! the IRI is for content-addressing.
//!
//! See [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
//! for the inclusion / exclusion criteria, the catalog growth tracks
//! (baseline vs. specialized), and the implementation pattern every
//! stdlib type follows.
//!
//! [08-closure]: https://github.com/UOR-Foundation/UOR-Framework/wiki/08-Concepts#closure-under-uor-foundation
//!
//! # See also
//!
//! - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
//! - [Wiki: 09 Architecture Decisions § ADR-017](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
//! - [Wiki: 12 Glossary § Term Definitions](https://github.com/UOR-Foundation/UOR-Framework/wiki/12-Glossary#term-definitions)
//!
//! # Constraints
//!
//! - **TC-02** — the morphism-kind traits are sealed by foundation; no
//!   downstream extension is permitted
//! - **TC-04** — the kind classification participates in compile-time
//!   UORassembly enforcement (a `Grounding` impl whose `Map` does not
//!   inhabit [`GroundingMapKind`] fails to compile)
//! - **ADR-017** — addresses are content-deterministic; the catalog is
//!   operational, not declarative
//!
//! # C4 placement
//!
//! Component `standard type library` (Level 3) inside container `prism`
//! (Level 2). It is consumed by application authors implementing
//! [`uor_foundation::enforcement::Grounding`] or
//! [`uor_foundation::enforcement::Sinking`].
//!
//! # Behavior
//!
//! ```rust
//! // Given: the ten morphism-kind marker types
//! // When:  each is used as a phantom type parameter
//! // Then:  the foundation's sealed trait family classifies them
//! //        identically to the foundation's own use sites
//! use prism::std_types::{
//!     BinaryGroundingMap, BinaryProjectionMap, DigestGroundingMap,
//!     DigestProjectionMap, IntegerGroundingMap, IntegerProjectionMap,
//!     JsonGroundingMap, JsonProjectionMap, Utf8GroundingMap,
//!     Utf8ProjectionMap,
//! };
//! fn _accepts_grounding<M: prism::std_types::GroundingMapKind>() {}
//! fn _accepts_projection<M: prism::std_types::ProjectionMapKind>() {}
//! _accepts_grounding::<BinaryGroundingMap>();
//! _accepts_grounding::<DigestGroundingMap>();
//! _accepts_grounding::<IntegerGroundingMap>();
//! _accepts_grounding::<JsonGroundingMap>();
//! _accepts_grounding::<Utf8GroundingMap>();
//! _accepts_projection::<BinaryProjectionMap>();
//! _accepts_projection::<DigestProjectionMap>();
//! _accepts_projection::<IntegerProjectionMap>();
//! _accepts_projection::<JsonProjectionMap>();
//! _accepts_projection::<Utf8ProjectionMap>();
//!
//! // And: the structural marker traits classify each kind as the
//! // ontology declares. `BinaryGroundingMap` is total and invertible;
//! // `IntegerGroundingMap` additionally preserves structure; the
//! // foundation rejects (at compile time) any attempt to claim a
//! // structural property a kind does not carry.
//! use prism::std_types::{Invertible, PreservesStructure, Total};
//! fn _total_invertible<M: prism::std_types::GroundingMapKind + Total + Invertible>() {}
//! fn _preserves_structure<M: prism::std_types::GroundingMapKind + PreservesStructure>() {}
//! _total_invertible::<BinaryGroundingMap>();
//! _total_invertible::<IntegerGroundingMap>();
//! _preserves_structure::<IntegerGroundingMap>();
//! _preserves_structure::<JsonGroundingMap>();
//! _preserves_structure::<Utf8GroundingMap>();
//! ```

pub use uor_foundation::enforcement::{
    BinaryGroundingMap, BinaryProjectionMap, DigestGroundingMap, DigestProjectionMap,
    GroundingMapKind, IntegerGroundingMap, IntegerProjectionMap, JsonGroundingMap,
    JsonProjectionMap, MorphismKind, ProjectionMapKind, Utf8GroundingMap, Utf8ProjectionMap,
};

// Sealed structural marker traits that classify morphism kinds. Authors
// use these in trait bounds to require, for example, an invertible
// grounding map without naming the concrete kind. The traits are
// foundation-sealed; downstream cannot add new structural classes.
pub use uor_foundation::enforcement::{Invertible, PreservesMetric, PreservesStructure, Total};

// The two sealed `GroundedValue` variants returned by `Grounding` impls,
// plus their sealed marker traits. `GroundedValue` is the closed set
// of permitted intermediates (`GroundedCoord` and `GroundedTuple<N>`);
// `GroundedShape` is the closed-set bound on the `T` of `Grounded<T>`.
pub use uor_foundation::enforcement::{GroundedCoord, GroundedShape, GroundedTuple, GroundedValue};

// `ConstrainedTypeInput` is the foundation's pre-declared canonical
// constrained-type shape: a built-in `ConstrainedTypeShape` impl that
// participates in the principal data path without the application
// author having to declare a fresh shape. It is the closest thing the
// standard type library has to a "prelude" type and is the canonical
// example used in the trace-replay round-trip scenario.
pub use uor_foundation::enforcement::ConstrainedTypeInput;

// `CartesianProductShape` is the foundation's canonical
// `ConstrainedTypeShape` for products of two component shapes (added in
// uor-foundation 0.3.1). It routes nerve-Betti computation through
// Künneth composition of component Betti profiles rather than flat
// pair-enumeration. Selecting it in a `result_type::<P>()` call admits
// a CartesianPartitionProduct unit through the principal data path.
pub use uor_foundation::pipeline::kunneth_compose;
pub use uor_foundation::pipeline::CartesianProductShape;

// Partition-algebra evidence, witness, and mint-input families. These
// are the cross-crate construction inputs and outputs for product,
// coproduct, and Cartesian-product partitions added by foundation
// 0.3.1's Product/Coproduct Completion Amendment. `PartitionResolver`,
// `PartitionRecord`, `PartitionHandle`, and `NullPartition` are the
// runtime-side carriers; `*Evidence`, `*Witness`, and `*MintInputs`
// classify the verified-mint bundles.
pub use uor_foundation::enforcement::{
    CartesianProductEvidence, CartesianProductMintInputs, CartesianProductWitness, NullPartition,
    PartitionCoproductEvidence, PartitionCoproductMintInputs, PartitionCoproductWitness,
    PartitionHandle, PartitionProductEvidence, PartitionProductMintInputs, PartitionProductWitness,
    PartitionRecord, PartitionResolver, VerifiedMint,
};

// `OntologyVerifiedMint` is the sealed mint trait introduced in 0.3.1
// for ontology-derived Path-2 witnesses. It carries a `HostTypes`-
// parameterized GAT `Inputs<H>` so witness inputs can hold
// host-decimal and handle fields without leaking concrete types.
pub use uor_foundation::OntologyVerifiedMint;

// ---- First-class stdlib types (§ 11 of AGENTS.md) ----

use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef};

/// `FixedSites<N>` — admit exactly `N` sites, unconstrained per-site.
///
/// The simplest non-trivial standard-type-library citizen: a generic
/// `ConstrainedTypeShape` that fixes a site count and imposes no
/// per-site constraint. It is the parametric building block under any
/// downstream shape that wants "this many sites, my own grounding
/// admission decides what each site contains" — for example, a 32-byte
/// hash output (32 sites at `WittLevel::W8`), an 80-byte Bitcoin block
/// header (80 sites at `WittLevel::W8`), or a 16-element
/// integer-vector at `WittLevel::W64`.
///
/// At any instantiation, `<FixedSites<N> as ConstrainedTypeShape>::SITE_COUNT == N`
/// and `<FixedSites<N> as ConstrainedTypeShape>::CONSTRAINTS` is the empty
/// slice (foundation reads "empty `CONSTRAINTS`" as "unconstrained" per
/// the trait's normative documentation). The IRI is the foundation's
/// `ConstrainedType` class IRI — shared across every empty-constraint
/// stdlib type per [ADR-017][09-adr-017] and the closure rule documented
/// in this module's header — so instance identity flows entirely
/// through `(SITE_COUNT, CONSTRAINTS)`.
///
/// [09-adr-017]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
///
/// # See also
///
/// - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
/// - [Wiki: 09 Architecture Decisions § ADR-017](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
///
/// # Constraints
///
/// - **TC-01** — admission is a compile-time activity; `SITE_COUNT` and
///   `CONSTRAINTS` are `const`-evaluable
/// - **TC-04** — bilateral compile-time enforcement: a downstream
///   author who consumes `FixedSites<N>` cannot violate the contract
///   without the toolchain rejecting their program
/// - **ADR-013** — closure under `uor-foundation`: the body uses only
///   foundation vocabulary (`ConstrainedTypeShape`, `ConstraintRef`)
/// - **ADR-017** — content-addressed identity: the
///   `(IRI, SITE_COUNT, CONSTRAINTS)` triple deterministically encodes
///   each instantiation
///
/// # Behavior
///
/// ```rust
/// // Given: a fixed-32-sites shape
/// // When:  its trait constants are read
/// // Then:  SITE_COUNT reflects N and CONSTRAINTS is empty
/// use prism::pipeline::ConstrainedTypeShape;
/// use prism::std_types::FixedSites;
/// assert_eq!(<FixedSites<32> as ConstrainedTypeShape>::SITE_COUNT, 32);
/// assert!(<FixedSites<32> as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
/// assert_eq!(
///     <FixedSites<32> as ConstrainedTypeShape>::IRI,
///     "https://uor.foundation/type/ConstrainedType",
/// );
/// // And: a different N produces a distinct content-address — same
/// // IRI, different SITE_COUNT — so the (IRI, SITE_COUNT, CONSTRAINTS)
/// // triple distinguishes the two instantiations.
/// assert_eq!(<FixedSites<80> as ConstrainedTypeShape>::SITE_COUNT, 80);
/// assert_eq!(
///     <FixedSites<80> as ConstrainedTypeShape>::IRI,
///     <FixedSites<32> as ConstrainedTypeShape>::IRI,
/// );
/// ```
pub struct FixedSites<const N: usize>;

impl<const N: usize> ConstrainedTypeShape for FixedSites<N> {
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = N;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
}

/// `Bytes<N>` — byte-buffer admission intent of width `N`.
///
/// Structurally identical to [`FixedSites<N>`] and content-address-
/// identical at equal `N` (closure rule: same constraint declaration ⇒
/// same IRI ⇒ same UOR address). Use `Bytes<N>` when the unit's intent
/// is "this is a byte sequence" and `FixedSites<N>` when the intent is
/// "this is a generic site container of width N"; the Rust type name
/// distinguishes intent at the call site, the IRI does not.
///
/// # See also
///
/// - [`crate::std_types`] — the family contract and IRI namespace
/// - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
/// - [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
///
/// # Constraints
///
/// - **TC-01** — admission is compile-time
/// - **TC-04** — bilateral compile-time enforcement
/// - **ADR-013** — closure under `uor-foundation`
/// - **ADR-017** — content-addressed identity via the IRI
///
/// # Behavior
///
/// ```rust
/// use prism::pipeline::ConstrainedTypeShape;
/// use prism::std_types::{Bytes, FixedSites};
/// // Same SITE_COUNT and same IRI as FixedSites<N> per closure.
/// assert_eq!(<Bytes<32> as ConstrainedTypeShape>::SITE_COUNT, 32);
/// assert_eq!(
///     <Bytes<32> as ConstrainedTypeShape>::IRI,
///     "https://uor.foundation/type/ConstrainedType",
/// );
/// assert_eq!(
///     <Bytes<32> as ConstrainedTypeShape>::IRI,
///     <FixedSites<32> as ConstrainedTypeShape>::IRI,
/// );
/// ```
pub struct Bytes<const N: usize>;

impl<const N: usize> ConstrainedTypeShape for Bytes<N> {
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = N;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
}

// ---- Typed primitives (baseline per AGENTS.md § 11.4) ----
//
// Each typed primitive is a unit struct that impls `ConstrainedTypeShape`
// with a stable IRI under `uor.foundation/prism/std_types/<TypeName>`,
// `SITE_COUNT` set to its byte width when used at `WittLevel::W8`, and
// empty `CONSTRAINTS`. Value-level invariants (IEEE 754 well-formedness,
// `Bool ∈ {0, 1}`, UTF-32 codepoint validity) are host-side decisions
// enforced by the application's `Grounding` impl per the family contract
// laid out in this module's docs and in AGENTS.md § 11.

macro_rules! typed_primitive {
    (
        $(#[$brief:meta])*
        $name:ident, $iri:literal, $sites:literal
    ) => {
        $(#[$brief])*
        ///
        /// # See also
        ///
        /// - [`crate::std_types`] for the family contract and IRI namespace.
        /// - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
        /// - [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
        ///
        /// # Constraints
        ///
        /// - **TC-01** — admission is compile-time
        /// - **TC-04** — bilateral compile-time enforcement
        /// - **ADR-013** — closure under `uor-foundation`
        /// - **ADR-017** — content-addressed identity via the IRI
        ///
        /// # Behavior
        ///
        /// ```rust
        /// use prism::pipeline::ConstrainedTypeShape;
        #[doc = concat!("use prism::std_types::", stringify!($name), ";")]
        #[doc = concat!(
            "assert_eq!(<", stringify!($name), " as ConstrainedTypeShape>::SITE_COUNT, ",
            stringify!($sites), ");"
        )]
        #[doc = concat!(
            "assert_eq!(<", stringify!($name), " as ConstrainedTypeShape>::IRI, \"", $iri, "\");"
        )]
        #[doc = concat!(
            "assert!(<", stringify!($name), " as ConstrainedTypeShape>::CONSTRAINTS.is_empty());"
        )]
        /// ```
        pub struct $name;

        impl ConstrainedTypeShape for $name {
            const IRI: &'static str = $iri;
            const SITE_COUNT: usize = $sites;
            const CONSTRAINTS: &'static [ConstraintRef] = &[];
        }
    };
}

// Unsigned integers — byte-aligned widths from 8 to 256 bits.
typed_primitive!(
    /// Unsigned 8-bit integer (1 byte at `WittLevel::W8`).
    U8, "https://uor.foundation/type/ConstrainedType", 1
);
typed_primitive!(
    /// Unsigned 16-bit integer (2 bytes at `WittLevel::W8`).
    U16, "https://uor.foundation/type/ConstrainedType", 2
);
typed_primitive!(
    /// Unsigned 32-bit integer (4 bytes at `WittLevel::W8`).
    /// Width of a Bitcoin block-header nonce.
    U32, "https://uor.foundation/type/ConstrainedType", 4
);
typed_primitive!(
    /// Unsigned 64-bit integer (8 bytes at `WittLevel::W8`).
    U64, "https://uor.foundation/type/ConstrainedType", 8
);
typed_primitive!(
    /// Unsigned 128-bit integer (16 bytes at `WittLevel::W8`).
    U128, "https://uor.foundation/type/ConstrainedType", 16
);
typed_primitive!(
    /// Unsigned 256-bit integer (32 bytes at `WittLevel::W8`).
    /// Width of a SHA-256 output and a Bitcoin difficulty target.
    U256, "https://uor.foundation/type/ConstrainedType", 32
);

// Signed integers — same byte widths, distinct IRIs to self-document
// signed admission intent.
typed_primitive!(
    /// Signed 8-bit integer (1 byte at `WittLevel::W8`).
    I8, "https://uor.foundation/type/ConstrainedType", 1
);
typed_primitive!(
    /// Signed 16-bit integer (2 bytes at `WittLevel::W8`).
    I16, "https://uor.foundation/type/ConstrainedType", 2
);
typed_primitive!(
    /// Signed 32-bit integer (4 bytes at `WittLevel::W8`).
    I32, "https://uor.foundation/type/ConstrainedType", 4
);
typed_primitive!(
    /// Signed 64-bit integer (8 bytes at `WittLevel::W8`).
    I64, "https://uor.foundation/type/ConstrainedType", 8
);
typed_primitive!(
    /// Signed 128-bit integer (16 bytes at `WittLevel::W8`).
    I128, "https://uor.foundation/type/ConstrainedType", 16
);
typed_primitive!(
    /// Signed 256-bit integer (32 bytes at `WittLevel::W8`).
    I256, "https://uor.foundation/type/ConstrainedType", 32
);

// IEEE 754 floating-point — IEEE well-formedness (NaN, subnormal
// handling) is the application's `Grounding` impl's responsibility.
typed_primitive!(
    /// IEEE 754 binary32 floating-point (4 bytes at `WittLevel::W8`).
    /// Well-formedness (NaN, subnormal, and infinity policy) is enforced
    /// host-side by the application's `Grounding` impl.
    F32, "https://uor.foundation/type/ConstrainedType", 4
);
typed_primitive!(
    /// IEEE 754 binary64 floating-point (8 bytes at `WittLevel::W8`).
    /// Well-formedness is enforced host-side.
    F64, "https://uor.foundation/type/ConstrainedType", 8
);

// Boolean — value-in-{0, 1} contract is enforced host-side; the
// distinct IRI separates `Bool` from `U8` at the content-address level.
typed_primitive!(
    /// Boolean (1 byte at `WittLevel::W8`). The value-in-{0, 1} contract
    /// is enforced host-side by the application's `Grounding` impl;
    /// the distinct IRI separates `Bool` from `U8` at the content-address
    /// level.
    Bool, "https://uor.foundation/type/ConstrainedType", 1
);

// Character — UTF-32 codepoint width; Unicode validity is host-side.
typed_primitive!(
    /// Unicode codepoint (4 bytes at `WittLevel::W8`, UTF-32 width).
    /// Unicode validity (codepoint range, surrogate exclusion) is
    /// enforced host-side by the application's `Grounding` impl.
    Char, "https://uor.foundation/type/ConstrainedType", 4
);
