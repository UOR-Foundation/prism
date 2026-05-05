//! Standard type library.
//!
//! `std_types` exposes the ten morphism kinds that the foundation
//! pre-declares: five [`grounding`](GroundingMapKind) maps (host bytes →
//! `Grounded`) and five [`projection`](ProjectionMapKind) maps
//! (`Grounded` → host bytes). Each kind is sealed: the trait family is
//! closed by foundation, downstream cannot add new map kinds, and each
//! concrete struct is a zero-size marker that compiles into the
//! constraint nerve.
//!
//! Per ADR-017 ("Canonical UOR-address surface for standard types"),
//! these types produce content-deterministic addresses; the catalog
//! evolves operationally, not in the wiki, and `std_types` is the
//! re-export surface that tracks it.
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
