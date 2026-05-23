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
//! - **Decentralized publication-graph shapes.** [`RouteShape`] and
//!   [`RevocationShape`] — the typed-distinction surfaces for publishing
//!   and revoking routes to UOR-addressed content over a `UorTime`
//!   validity window.
//! - **Composition shapes (ADR-061).** [`G2ProductShape<N>`][G2ProductShape]
//!   (binary product, `SITE_COUNT = 2N`), the operand-preserving unary
//!   [`F4QuotientShape<N>`][F4QuotientShape] /
//!   [`E7AugmentationShape<N>`][E7AugmentationShape] /
//!   [`E8EmbeddingShape<N>`][E8EmbeddingShape] (`SITE_COUNT = N`), and the
//!   structure-preserving [`E6FiltrationShape<N>`][E6FiltrationShape]
//!   (`SITE_COUNT = N + 1`) — each of ADR-059's five categorical
//!   operations on the Atlas image inside E₈, as a `ConstrainedTypeShape`.
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
    // ADR-032: cardinality of the value-set under the discrete-clock
    // model. Empty-constraint shapes at W8 semantics carry 256 values
    // per site; `cartesian_product_shape` (homogeneous power) raises
    // the per-site cycle to `SITE_COUNT` saturating. `FixedSites<0>`
    // collapses to the identity (`CYCLE_SIZE = 1`), matching
    // `ConstrainedTypeInput` and the foundation convention. The
    // truncation of `N: usize` to `u32` is harmless: SITE_COUNT values
    // that approach `u32::MAX` would already overflow `u64` and
    // saturate to `u64::MAX` long before the cast loses information.
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(N as u32);
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
    // ADR-032: per closure, identical to `FixedSites<N>`. Truncation
    // bounded as in `FixedSites<N>` above.
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(N as u32);
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
            // ADR-032: 256-per-site at W8 raised to SITE_COUNT, saturating.
            const CYCLE_SIZE: u64 = 256u64.saturating_pow($sites as u32);
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

// ---- Decentralized publication-graph shapes ----
//
// The Prism standard library's catalog gains structural shapes for
// decentralized content-addressing networks built atop UOR-native
// primitives. Per the framework's commitments:
//
// - `UorTime` (foundation, re-exported as `prism::vocabulary::UorTime`)
//   supplies substrate-independent temporal ordering via the joint
//   Landauer-budget and rewrite-step partial order, with
//   `UorTime::min_wall_clock` under a `Calibration` deriving the
//   provable physical lower-bound wall-clock duration.
// - `SignatureAxis` (re-exported as `prism::crypto::SignatureAxis`)
//   supplies signing/verification per wiki ADR-031.
// - `CommitmentAxis` (re-exported as `prism::crypto::CommitmentAxis`)
//   supplies commitment surfaces (Merkle, Pedersen, KZG) per
//   wiki ADR-031.
// - `FheAxis` (re-exported as `prism::fhe::FheAxis`) supplies
//   homomorphic-encryption surface per wiki ADR-031.
//
// `RouteShape` and `RevocationShape` are the structural shape
// identities applications use to publish route declarations on a
// decentralized data bus and to revoke previously-published routes.
// Per `AGENTS.md § 11.3`'s closure rule, the shapes carry no
// semantic content beyond their `(IRI, SITE_COUNT, CONSTRAINTS)`
// triple; the Rust type-name distinction is the developer-facing
// surface. Applications give the sites meaning through their
// realization's canonicalize discipline. The const-generic
// parameters fix the per-component byte widths the application's
// realization commits to, so the type-system distinguishes
// instances whose component widths differ.

/// A publication-graph **route declaration** shape — the structural
/// type identity an application uses to publish a route from a
/// content κ-label to a service endpoint over a `UorTime` validity
/// window, witnessed by a signature κ-label and bound to a
/// commitment-root κ-label.
///
/// Per `AGENTS.md § 11.1`, the shape carries no operation logic and
/// no resolvers — it is the typed-distinction surface only. The
/// application's realization supplies the operational semantics:
/// the canonicalize function that serializes the five components
/// into the shape's `SITE_COUNT` sites, the `SignatureAxis` impl
/// that authenticates the signature κ-label, the `CommitmentAxis`
/// impl that verifies the commitment-root κ-label, and the
/// `Calibration` that interprets the time-pair's physical bounds.
///
/// The five const-generic parameters fix the per-component byte
/// widths:
///
/// - `TARGET_LABEL_BYTES` — the κ-label being routed
///   (71 for sha256/blake3, 73 for sha3-256, 74 for keccak256;
///   see `uor_addr::hash::label_bytes`).
/// - `ENDPOINT_BYTES` — application-encoded service endpoint width;
///   application-policy (e.g., 32 for a fixed-width peer identifier,
///   wider for a multiaddr).
/// - `TIME_PAIR_BYTES` — width of the realization's encoding of the
///   `(valid-from, valid-until)` `UorTime` pair. The foundation
///   exposes no fixed wire format for `UorTime`; the realization
///   architect commits the encoding (e.g., big-endian f64 + big-endian
///   u64 per value, yielding 32 bytes; or a compact varint encoding
///   yielding less).
/// - `SIG_LABEL_BYTES` — the signature κ-label width (per chosen σ-axis).
/// - `COMMIT_LABEL_BYTES` — the commitment-root κ-label width
///   (per chosen σ-axis).
///
/// `SITE_COUNT` is the sum of all five widths. The Rust type system
/// distinguishes `RouteShape<A,B,C,D,E>` from any
/// `RouteShape<A',B',C',D',E'>` with different widths, even when
/// `SITE_COUNT` is numerically equal — this is the
/// type-system-level naming that gives applications a typed handle
/// distinguishing route declarations from other shapes carrying the
/// same byte count.
///
/// The shape's IRI is `https://uor.foundation/type/ConstrainedType`
/// per the closure rule (`AGENTS.md § 11.3`) — empty-`CONSTRAINTS`
/// shapes share the foundation's class IRI.
///
/// # See also
///
/// - [`crate::std_types`] for the family contract and IRI namespace.
/// - [`crate::vocabulary::UorTime`] for the temporal-ordering primitive.
/// - [`crate::crypto::SignatureAxis`] for signature verification.
/// - [`crate::crypto::CommitmentAxis`] for commitment proofs.
/// - [`crate::fhe::FheAxis`] for content-ciphertext composition.
/// - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
/// - [Wiki: 09 Architecture Decisions § ADR-031](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
/// - [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
///
/// # Constraints
///
/// - **TC-01** — admission is compile-time.
/// - **TC-04** — bilateral compile-time enforcement.
/// - **ADR-013** — closure under `uor-foundation`.
/// - **ADR-017** — content-addressed identity via the IRI.
/// - **ADR-031** — composes with `prism-crypto` + `prism-fhe` Layer-3 axes.
///
/// # Behavior
///
/// ```rust
/// use prism::pipeline::ConstrainedTypeShape;
/// use prism::std_types::RouteShape;
///
/// // Given an application committing the sha256 σ-axis across all three
/// // κ-label positions, a 32-byte peer-id endpoint, and a 32-byte
/// // time-pair encoding (big-endian f64 + big-endian u64 per value):
/// type R = RouteShape<71, 32, 32, 71, 71>;
/// // When SITE_COUNT is queried,
/// // Then it equals the sum of the five widths.
/// assert_eq!(<R as ConstrainedTypeShape>::SITE_COUNT, 71 + 32 + 32 + 71 + 71);
/// // And the shape shares the closure-under-foundation class IRI.
/// assert_eq!(
///     <R as ConstrainedTypeShape>::IRI,
///     "https://uor.foundation/type/ConstrainedType",
/// );
/// // And carries no embedded constraints.
/// assert!(<R as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
///
/// // Given a different application choosing keccak256 across the κ-labels:
/// type RK = RouteShape<74, 32, 32, 74, 74>;
/// // When SITE_COUNT is queried,
/// // Then the shape widens to reflect the wider keccak256 labels.
/// assert_eq!(<RK as ConstrainedTypeShape>::SITE_COUNT, 74 + 32 + 32 + 74 + 74);
/// ```
pub struct RouteShape<
    const TARGET_LABEL_BYTES: usize,
    const ENDPOINT_BYTES: usize,
    const TIME_PAIR_BYTES: usize,
    const SIG_LABEL_BYTES: usize,
    const COMMIT_LABEL_BYTES: usize,
>;

impl<
        const TARGET_LABEL_BYTES: usize,
        const ENDPOINT_BYTES: usize,
        const TIME_PAIR_BYTES: usize,
        const SIG_LABEL_BYTES: usize,
        const COMMIT_LABEL_BYTES: usize,
    > ConstrainedTypeShape
    for RouteShape<
        TARGET_LABEL_BYTES,
        ENDPOINT_BYTES,
        TIME_PAIR_BYTES,
        SIG_LABEL_BYTES,
        COMMIT_LABEL_BYTES,
    >
{
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = TARGET_LABEL_BYTES
        + ENDPOINT_BYTES
        + TIME_PAIR_BYTES
        + SIG_LABEL_BYTES
        + COMMIT_LABEL_BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    // ADR-032: empty-constraint shape at W8 semantics — 256 values per site,
    // raised to SITE_COUNT saturating. Same path as `Bytes<N>`/`FixedSites<N>`.
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(Self::SITE_COUNT as u32);
}

/// A publication-graph **revocation declaration** shape — the
/// structural type identity an application uses to revoke a
/// previously-published [`RouteShape`].
///
/// A revocation carries the same five-component surface as the route
/// it revokes, plus one additional κ-label position: the κ-label of
/// the route being revoked. The revocation's own `(valid-from,
/// valid-until)` `UorTime` pair determines from when the revocation
/// is in effect; the revocation's signature κ-label authenticates
/// the revocation per the publishing node's identity discipline.
///
/// A revocation supersedes the route it references when, under the
/// application's `Calibration`, the revocation's `valid-from`
/// `UorTime` is `>=` the targeted route's `valid-from` per the
/// partial-order of `UorTime`. Two parties evaluating the same
/// (route, revocation) pair under the same `Calibration` reach the
/// same supersession decision — substrate-independent per the
/// `UorTime` discipline.
///
/// The first five const-generic parameters mirror [`RouteShape`]'s
/// per-component widths; `REVOKED_LABEL_BYTES` is the κ-label width
/// of the route being revoked (which may differ from
/// `TARGET_LABEL_BYTES` if the revoking publisher uses a different
/// σ-axis than the original publisher).
///
/// `SITE_COUNT` is the sum of the six widths. As with [`RouteShape`],
/// the Rust type system distinguishes a `RevocationShape` from any
/// `RouteShape` even when their `SITE_COUNT` values are numerically
/// equal.
///
/// # See also
///
/// - [`RouteShape`] for the route declaration this revokes.
/// - [`crate::std_types`] for the family contract and IRI namespace.
/// - [`crate::vocabulary::UorTime`] for the temporal-ordering primitive.
/// - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
/// - [Wiki: 09 Architecture Decisions § ADR-031](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
/// - [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
///
/// # Constraints
///
/// - **TC-01** — admission is compile-time.
/// - **TC-04** — bilateral compile-time enforcement.
/// - **ADR-013** — closure under `uor-foundation`.
/// - **ADR-017** — content-addressed identity via the IRI.
/// - **ADR-031** — composes with `prism-crypto` Layer-3 axes.
///
/// # Behavior
///
/// ```rust
/// use prism::pipeline::ConstrainedTypeShape;
/// use prism::std_types::{RevocationShape, RouteShape};
///
/// // Given a route shape and a same-axis revocation shape over it:
/// type ROUTE = RouteShape<71, 32, 32, 71, 71>;
/// type REV   = RevocationShape<71, 32, 32, 71, 71, 71>;
/// // When the revocation's SITE_COUNT is queried,
/// // Then it equals the route's SITE_COUNT plus the revoked-label width.
/// assert_eq!(
///     <REV as ConstrainedTypeShape>::SITE_COUNT,
///     <ROUTE as ConstrainedTypeShape>::SITE_COUNT + 71,
/// );
/// // And the revocation shares the closure-under-foundation class IRI.
/// assert_eq!(
///     <REV as ConstrainedTypeShape>::IRI,
///     "https://uor.foundation/type/ConstrainedType",
/// );
/// // And carries no embedded constraints.
/// assert!(<REV as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
/// ```
pub struct RevocationShape<
    const TARGET_LABEL_BYTES: usize,
    const ENDPOINT_BYTES: usize,
    const TIME_PAIR_BYTES: usize,
    const SIG_LABEL_BYTES: usize,
    const COMMIT_LABEL_BYTES: usize,
    const REVOKED_LABEL_BYTES: usize,
>;

impl<
        const TARGET_LABEL_BYTES: usize,
        const ENDPOINT_BYTES: usize,
        const TIME_PAIR_BYTES: usize,
        const SIG_LABEL_BYTES: usize,
        const COMMIT_LABEL_BYTES: usize,
        const REVOKED_LABEL_BYTES: usize,
    > ConstrainedTypeShape
    for RevocationShape<
        TARGET_LABEL_BYTES,
        ENDPOINT_BYTES,
        TIME_PAIR_BYTES,
        SIG_LABEL_BYTES,
        COMMIT_LABEL_BYTES,
        REVOKED_LABEL_BYTES,
    >
{
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = TARGET_LABEL_BYTES
        + ENDPOINT_BYTES
        + TIME_PAIR_BYTES
        + SIG_LABEL_BYTES
        + COMMIT_LABEL_BYTES
        + REVOKED_LABEL_BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(Self::SITE_COUNT as u32);
}

// ---- Composition shapes (ADR-061) ----
//
// The Prism standard library's catalog gains five `ConstrainedTypeShape`
// impls realizing the categorical operations on the Atlas image inside
// E₈ per wiki ADR-059's codomain-structure commitment. Each shape is
// the typed-input identity for a composition realization in uor-addr
// (the 10th realization, joining json, sexp, xml, asn1, ring,
// codemodule, gguf, onnx, cbor).
//
// Per wiki ADR-061's structural commitment, each categorical operation
// on the Atlas IS a composition shape — not a marker type parameterizing
// a separate shape. The Rust type system distinguishes the five shapes;
// each shape's `SITE_COUNT` formula reflects its natural arity per
// ADR-059's construction:
//
// - G₂ via product (Klein quartet × ℤ/3 → 12 roots, rank 2): binary
//   product. `G2ProductShape::SITE_COUNT = 2 × N`.
// - F₄ via quotient (96 / ± mirror symmetry → 48 sign classes, rank 4):
//   unary quotient. `F4QuotientShape::SITE_COUNT = N`.
// - E₆ via filtration (64 degree-5 + 8 degree-6 vertices → 72 roots,
//   rank 6): unary filtration, structure-preserving — the canonical
//   form retains the operand identity (N bytes) annotated with a one-
//   byte degree-partition tag identifying which of the two filtration
//   groups (degree-5 or degree-6) the operand belongs to.
//   `E6FiltrationShape::SITE_COUNT = N + 1` per wiki ADR-061 §(2).
// - E₇ via augmentation (96 vertices + 30 S₄ orbits → 126 roots,
//   rank 7): unary augmentation, canonical-form-internal — the
//   operand is normalized to its S₄-orbit canonical representative;
//   no additional bytes prepended. `E7AugmentationShape::SITE_COUNT = N`.
// - E₈ via direct embedding (φ: Atlas ↪ E₈ injective → 240 roots,
//   rank 8): unary direct embedding, identity on canonical-form
//   bytes — the composed κ-label is distinguished from the operand's
//   κ-label by realization-IRI provenance.
//   `E8EmbeddingShape::SITE_COUNT = N`.
//
// Multi-operand compositions of arity > 2 iterate via
// `ConstraintRef::Recurse` per ADR-057 — a three-operand product
// decomposes into two iterated `G2ProductShape` applications. The
// per-shape canonical ordering is the realization architect's
// commitment (the uor-addr composition realization), grounded in the
// operation's algebraic structure; the framework commits the
// constraint, the realization commits the exact rule.

/// `G₂-via-product` composition shape — the binary product
/// construction on the Atlas image inside E₈ per wiki [ADR-059]'s
/// categorical-operation vocabulary.
///
/// G₂ is the rank-2 exceptional Lie algebra reached from the Atlas by
/// the Klein quartet × ℤ/3 construction (12 roots). As a categorical
/// operation, G₂ is a *product* of two algebraic objects — the shape
/// is therefore binary by structural necessity:
/// `SITE_COUNT = 2 × COMPONENT_LABEL_BYTES`. Two operand κ-labels at
/// the chosen σ-axis byte width concatenate in the canonical order
/// the realization architect commits to (typically lexicographic for
/// symmetric operands).
///
/// The composed κ-label produced by the corresponding uor-addr
/// composition realization addresses the operand pair under the G₂
/// product's algebraic identity. Cross-substrate convergence holds
/// per ADR-058 + ADR-059 + ADR-061: two substrates composing the same
/// two operands under the same σ-axis emit byte-identical composed
/// κ-labels.
///
/// # See also
///
/// - [`crate::std_types`] for the family contract and IRI namespace.
/// - [`F4QuotientShape`] / [`E6FiltrationShape`] /
///   [`E7AugmentationShape`] / [`E8EmbeddingShape`] for the other
///   four categorical operations on the Atlas.
/// - [`crate::convergence`] for the convergence-tower vocabulary
///   ADR-059 commits.
/// - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
/// - [Wiki: 09 Architecture Decisions § ADR-059](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
/// - [Wiki: 09 Architecture Decisions § ADR-061](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
/// - [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
///
/// # Constraints
///
/// - **TC-01** — admission is compile-time.
/// - **TC-04** — bilateral compile-time enforcement.
/// - **ADR-013** — closure under `uor-foundation`.
/// - **ADR-017** — content-addressed identity via the IRI.
/// - **ADR-058** — κ-derivation produces the composed κ-label.
/// - **ADR-059** — codomain factors through the Atlas image inside E₈.
/// - **ADR-061** — operational composition surface for κ-labels.
///
/// # Behavior
///
/// ```rust
/// use prism::pipeline::ConstrainedTypeShape;
/// use prism::std_types::G2ProductShape;
///
/// // Given a binary G₂ product over two sha256 κ-labels (71 bytes each),
/// type G = G2ProductShape<71>;
/// // When SITE_COUNT is queried,
/// // Then it equals 2 × 71 — the binary product's structural width.
/// assert_eq!(<G as ConstrainedTypeShape>::SITE_COUNT, 2 * 71);
/// assert_eq!(
///     <G as ConstrainedTypeShape>::IRI,
///     "https://uor.foundation/type/ConstrainedType",
/// );
/// assert!(<G as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
/// ```
pub struct G2ProductShape<const COMPONENT_LABEL_BYTES: usize>;

impl<const COMPONENT_LABEL_BYTES: usize> ConstrainedTypeShape
    for G2ProductShape<COMPONENT_LABEL_BYTES>
{
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = 2 * COMPONENT_LABEL_BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(Self::SITE_COUNT as u32);
}

/// `F₄-via-quotient` composition shape — the unary quotient
/// construction on the Atlas image inside E₈ per wiki [ADR-059]'s
/// categorical-operation vocabulary.
///
/// F₄ is the rank-4 exceptional Lie algebra reached from the Atlas by
/// the 96 / ± mirror-symmetry quotient (48 sign classes). As a
/// categorical operation, F₄ is a *quotient* of one Atlas structure
/// under mirror symmetry — the shape is therefore unary by structural
/// necessity: `SITE_COUNT = COMPONENT_LABEL_BYTES`. A single operand
/// κ-label is canonicalized into its mirror-symmetry equivalence
/// class before σ-projection.
///
/// The composed κ-label addresses the operand's equivalence class
/// under the quotient, not the operand directly. Two operands that
/// are mirror-symmetric per the realization's commitment compose to
/// byte-identical composed κ-labels.
///
/// # See also
///
/// - [`crate::std_types`] for the family contract and IRI namespace.
/// - [`G2ProductShape`] / [`E6FiltrationShape`] /
///   [`E7AugmentationShape`] / [`E8EmbeddingShape`] for the other
///   four categorical operations on the Atlas.
/// - [`crate::convergence`] for the convergence-tower vocabulary
///   ADR-059 commits.
/// - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
/// - [Wiki: 09 Architecture Decisions § ADR-059](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
/// - [Wiki: 09 Architecture Decisions § ADR-061](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
/// - [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
///
/// # Constraints
///
/// - **TC-01** — admission is compile-time.
/// - **TC-04** — bilateral compile-time enforcement.
/// - **ADR-013** — closure under `uor-foundation`.
/// - **ADR-017** — content-addressed identity via the IRI.
/// - **ADR-058** — κ-derivation produces the composed κ-label.
/// - **ADR-059** — codomain factors through the Atlas image inside E₈.
/// - **ADR-061** — operational composition surface for κ-labels.
///
/// # Behavior
///
/// ```rust
/// use prism::pipeline::ConstrainedTypeShape;
/// use prism::std_types::F4QuotientShape;
///
/// // Given a unary F₄ quotient over one sha256 κ-label (71 bytes),
/// type F = F4QuotientShape<71>;
/// // When SITE_COUNT is queried,
/// // Then it equals the operand width — F₄'s quotient is unary.
/// assert_eq!(<F as ConstrainedTypeShape>::SITE_COUNT, 71);
/// assert_eq!(
///     <F as ConstrainedTypeShape>::IRI,
///     "https://uor.foundation/type/ConstrainedType",
/// );
/// assert!(<F as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
/// ```
pub struct F4QuotientShape<const COMPONENT_LABEL_BYTES: usize>;

impl<const COMPONENT_LABEL_BYTES: usize> ConstrainedTypeShape
    for F4QuotientShape<COMPONENT_LABEL_BYTES>
{
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = COMPONENT_LABEL_BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(Self::SITE_COUNT as u32);
}

/// `E₆-via-filtration` composition shape — the unary filtration
/// construction on the Atlas image inside E₈ per wiki [ADR-059]'s
/// categorical-operation vocabulary.
///
/// E₆ is the rank-6 exceptional Lie algebra reached from the Atlas by
/// the degree-partition filtration (64 degree-5 vertices + 8 degree-6
/// vertices → 72 roots). As a categorical operation, E₆ is a
/// *filtration* of one Atlas structure by vertex degree — the shape
/// is unary, with `SITE_COUNT = COMPONENT_LABEL_BYTES + 1` per wiki
/// [ADR-061] §(2). The filtration is structure-preserving (not
/// quotient-like): the canonical form retains the operand's identity
/// (`COMPONENT_LABEL_BYTES` bytes) annotated with a one-byte degree-
/// partition tag identifying which of the two filtration groups
/// (degree-5 or degree-6) the operand belongs to. The composed
/// κ-label respects the filtration's degree-partition by structural
/// concatenation of the tag and the operand bytes.
///
/// # See also
///
/// - [`crate::std_types`] for the family contract and IRI namespace.
/// - [`G2ProductShape`] / [`F4QuotientShape`] /
///   [`E7AugmentationShape`] / [`E8EmbeddingShape`] for the other
///   four categorical operations on the Atlas.
/// - [`crate::convergence`] for the convergence-tower vocabulary
///   ADR-059 commits.
/// - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
/// - [Wiki: 09 Architecture Decisions § ADR-059](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
/// - [Wiki: 09 Architecture Decisions § ADR-061](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
/// - [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
///
/// # Constraints
///
/// - **TC-01** — admission is compile-time.
/// - **TC-04** — bilateral compile-time enforcement.
/// - **ADR-013** — closure under `uor-foundation`.
/// - **ADR-017** — content-addressed identity via the IRI.
/// - **ADR-058** — κ-derivation produces the composed κ-label.
/// - **ADR-059** — codomain factors through the Atlas image inside E₈.
/// - **ADR-061** — operational composition surface for κ-labels.
///
/// # Behavior
///
/// ```rust
/// use prism::pipeline::ConstrainedTypeShape;
/// use prism::std_types::E6FiltrationShape;
///
/// // Given a unary E₆ filtration over one sha256 κ-label (71 bytes),
/// type E6 = E6FiltrationShape<71>;
/// // When SITE_COUNT is queried,
/// // Then it equals the operand width + 1 — the structure-preserving
/// // filtration prepends a one-byte degree-partition tag.
/// assert_eq!(<E6 as ConstrainedTypeShape>::SITE_COUNT, 72);
/// assert_eq!(
///     <E6 as ConstrainedTypeShape>::IRI,
///     "https://uor.foundation/type/ConstrainedType",
/// );
/// assert!(<E6 as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
/// ```
pub struct E6FiltrationShape<const COMPONENT_LABEL_BYTES: usize>;

impl<const COMPONENT_LABEL_BYTES: usize> ConstrainedTypeShape
    for E6FiltrationShape<COMPONENT_LABEL_BYTES>
{
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = COMPONENT_LABEL_BYTES + 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(Self::SITE_COUNT as u32);
}

/// `E₇-via-augmentation` composition shape — the unary augmentation
/// construction on the Atlas image inside E₈ per wiki [ADR-059]'s
/// categorical-operation vocabulary.
///
/// E₇ is the rank-7 exceptional Lie algebra reached from the Atlas by
/// the S₄-orbit augmentation (96 vertices + 30 S₄ orbits → 126 roots).
/// As a categorical operation, E₇ is an *augmentation* of one Atlas
/// structure with S₄-orbit data — the shape is unary, with
/// `SITE_COUNT = COMPONENT_LABEL_BYTES` per wiki [ADR-061] §(2). The
/// S₄ augmentation is canonical-form-internal: the operand is
/// normalized to its S₄-orbit canonical representative; no additional
/// bytes are prepended to the canonical form. The augmentation data
/// is part of the realization's canonicalize verb, internal to the
/// canonicalize function, not an additional operand position.
///
/// The composed κ-label respects the augmentation's S₄-orbit structure.
///
/// # See also
///
/// - [`crate::std_types`] for the family contract and IRI namespace.
/// - [`G2ProductShape`] / [`F4QuotientShape`] /
///   [`E6FiltrationShape`] / [`E8EmbeddingShape`] for the other
///   four categorical operations on the Atlas.
/// - [`crate::convergence`] for the convergence-tower vocabulary
///   ADR-059 commits.
/// - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
/// - [Wiki: 09 Architecture Decisions § ADR-059](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
/// - [Wiki: 09 Architecture Decisions § ADR-061](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
/// - [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
///
/// # Constraints
///
/// - **TC-01** — admission is compile-time.
/// - **TC-04** — bilateral compile-time enforcement.
/// - **ADR-013** — closure under `uor-foundation`.
/// - **ADR-017** — content-addressed identity via the IRI.
/// - **ADR-058** — κ-derivation produces the composed κ-label.
/// - **ADR-059** — codomain factors through the Atlas image inside E₈.
/// - **ADR-061** — operational composition surface for κ-labels.
///
/// # Behavior
///
/// ```rust
/// use prism::pipeline::ConstrainedTypeShape;
/// use prism::std_types::E7AugmentationShape;
///
/// // Given a unary E₇ augmentation over one sha256 κ-label (71 bytes),
/// type E7 = E7AugmentationShape<71>;
/// // When SITE_COUNT is queried,
/// // Then it equals the operand width — E₇'s augmentation is unary.
/// assert_eq!(<E7 as ConstrainedTypeShape>::SITE_COUNT, 71);
/// assert_eq!(
///     <E7 as ConstrainedTypeShape>::IRI,
///     "https://uor.foundation/type/ConstrainedType",
/// );
/// assert!(<E7 as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
/// ```
pub struct E7AugmentationShape<const COMPONENT_LABEL_BYTES: usize>;

impl<const COMPONENT_LABEL_BYTES: usize> ConstrainedTypeShape
    for E7AugmentationShape<COMPONENT_LABEL_BYTES>
{
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = COMPONENT_LABEL_BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(Self::SITE_COUNT as u32);
}

/// `E₈-via-direct-embedding` composition shape — the universal
/// embedding construction on the Atlas image inside E₈ per wiki
/// [ADR-059]'s categorical-operation vocabulary.
///
/// E₈ is the rank-8 exceptional Lie algebra reached from the Atlas by
/// the direct embedding φ: Atlas ↪ E₈ (injective, adjacency-preserving,
/// 240 roots). As a categorical operation, E₈ is the *direct embedding*
/// of one Atlas structure into the full E₈ root system — the shape is
/// unary, with `SITE_COUNT = COMPONENT_LABEL_BYTES` per wiki [ADR-061]
/// §(2). The universal target — any single operand factors through E₈
/// without further algebraic constraint per ADR-059's
/// Atlas-as-initial-object commitment. The embedding is the identity
/// on canonical-form bytes; the composed κ-label is distinguished from
/// the operand's κ-label by realization-IRI provenance, not by digest
/// bytes.
///
/// The composed κ-label addresses the operand's E₈ image directly.
/// Two operands at the same Atlas-image position modulo E₈ Weyl-orbit
/// equivalence compose to byte-identical composed κ-labels under
/// fixed σ-axis selection per ADR-047.
///
/// # See also
///
/// - [`crate::std_types`] for the family contract and IRI namespace.
/// - [`G2ProductShape`] / [`F4QuotientShape`] /
///   [`E6FiltrationShape`] / [`E7AugmentationShape`] for the other
///   four categorical operations on the Atlas.
/// - [`crate::convergence`] for the convergence-tower vocabulary
///   ADR-059 commits.
/// - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
/// - [Wiki: 09 Architecture Decisions § ADR-059](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
/// - [Wiki: 09 Architecture Decisions § ADR-061](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
/// - [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
///
/// # Constraints
///
/// - **TC-01** — admission is compile-time.
/// - **TC-04** — bilateral compile-time enforcement.
/// - **ADR-013** — closure under `uor-foundation`.
/// - **ADR-017** — content-addressed identity via the IRI.
/// - **ADR-058** — κ-derivation produces the composed κ-label.
/// - **ADR-059** — codomain factors through the Atlas image inside E₈.
/// - **ADR-061** — operational composition surface for κ-labels.
///
/// # Behavior
///
/// ```rust
/// use prism::pipeline::ConstrainedTypeShape;
/// use prism::std_types::E8EmbeddingShape;
///
/// // Given a unary E₈ direct embedding over one sha256 κ-label (71 bytes),
/// type E8 = E8EmbeddingShape<71>;
/// // When SITE_COUNT is queried,
/// // Then it equals the operand width — E₈'s direct embedding is unary.
/// assert_eq!(<E8 as ConstrainedTypeShape>::SITE_COUNT, 71);
/// assert_eq!(
///     <E8 as ConstrainedTypeShape>::IRI,
///     "https://uor.foundation/type/ConstrainedType",
/// );
/// assert!(<E8 as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
/// ```
pub struct E8EmbeddingShape<const COMPONENT_LABEL_BYTES: usize>;

impl<const COMPONENT_LABEL_BYTES: usize> ConstrainedTypeShape
    for E8EmbeddingShape<COMPONENT_LABEL_BYTES>
{
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = COMPONENT_LABEL_BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(Self::SITE_COUNT as u32);
}
