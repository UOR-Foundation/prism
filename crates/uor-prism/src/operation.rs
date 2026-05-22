//! Operation-declaration vocabulary.
//!
//! `operation` exposes the closed primitive vocabulary the application
//! author composes when declaring a constrained type: the [`Term`] AST
//! and its arena ([`TermArena`]), the [`TermList`] container, and the
//! eighteen-element closed set of [`PrimitiveOp`] discriminants (the
//! original fifteen plus `Div`/`Mod`/`Pow` per ADR-053). The
//! [`TermValue`] carrier on `Term::Literal` per ADR-051 — refined by
//! ADR-060 into the source-polymorphic enum
//! `TermValue<'a, INLINE_BYTES> { Inline, Borrowed, Stream }` — lets a
//! wide-Witt-level literal sit inline in the AST, a larger in-memory
//! value borrow zero-copy, and an unbounded payload stream via a
//! `ChunkSource`. The inline width derives from the application's
//! `HostBounds` via `carrier_inline_bytes::<B>()`; there is no
//! foundation byte-width cap (ADR-060 removed `TERM_VALUE_MAX_BYTES`).
//!
//! Per ADR-014, `prism`'s `operation` module surfaces the *primitive
//! operation vocabulary* — the closed `PrimitiveOp` set that the
//! catamorphism's per-variant fold-rules evaluate. Application-author
//! operation *libraries* are declared at Layer 3 via the SDK macros
//! `verb!` (ADR-024 — named compositions of prism operators) and
//! `axis!` (ADR-030 — substrate-extension vocabularies); the
//! standard-library Layer-3 sub-crates published from this repository
//! (`prism::{crypto, numerics, tensor, fhe}` per ADR-031) ship the
//! canonical reference impls. The foundation guarantees the closed
//! `PrimitiveOp` set is exhaustive — kind-typed discriminants with no
//! proc-macro back-doors per the substrate's W4 conformance check.
//!
//! # See also
//!
//! - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
//! - [Wiki: 08 Concepts § Operation Declaration](https://github.com/UOR-Foundation/UOR-Framework/wiki/08-Concepts#operation-declaration)
//! - [Wiki: 09 Architecture Decisions § ADR-014](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
//!
//! # Constraints
//!
//! - **TC-01** — operation declaration is a compile-time activity; no
//!   runtime dispatch is generated for the declared primitives
//! - **TC-04** — the closed set of primitives is part of the
//!   bilateral compile-time UORassembly contract
//! - **ADR-014** — `prism`'s `operation` module declares the
//!   `PrimitiveOp` vocabulary; operation libraries (verb declarations
//!   per ADR-024 + axis declarations per ADR-030) are Layer-3 surfaces
//!   the standard-library sub-crates per ADR-031 supply canonical
//!   impls for
//!
//! # C4 placement
//!
//! Component `operation declaration` (Level 3) inside container `prism`
//! (Level 2). The vocabulary is consumed by [`crate::pipeline`] and by
//! application-author code that constructs [`Term`] expressions.
//!
//! # Behavior
//!
//! ```rust
//! // Given: the closed set of primitive operations
//! // When:  matched exhaustively
//! // Then:  every variant is named exactly once and the match compiles
//! use prism::operation::PrimitiveOp;
//! fn _arity_class(op: PrimitiveOp) -> &'static str {
//!     match op {
//!         PrimitiveOp::Neg | PrimitiveOp::Bnot | PrimitiveOp::Succ | PrimitiveOp::Pred => "unary",
//!         PrimitiveOp::Add
//!         | PrimitiveOp::Sub
//!         | PrimitiveOp::Mul
//!         | PrimitiveOp::Xor
//!         | PrimitiveOp::And
//!         | PrimitiveOp::Or
//!         | PrimitiveOp::Le
//!         | PrimitiveOp::Lt
//!         | PrimitiveOp::Ge
//!         | PrimitiveOp::Gt
//!         | PrimitiveOp::Concat
//!         | PrimitiveOp::Div
//!         | PrimitiveOp::Mod
//!         | PrimitiveOp::Pow => "binary",
//!     }
//! }
//! assert_eq!(_arity_class(PrimitiveOp::Add), "binary");
//! assert_eq!(_arity_class(PrimitiveOp::Neg), "unary");
//!
//! // And: the author-implemented admission and projection traits are
//! // reachable as trait bounds. The generic functions compile only
//! // because `Grounding` and `Sinking` are public traits at this path.
//! use prism::operation::{Grounding, Sinking};
//! fn _accepts_grounding<G: Grounding>() {}
//! // Per ADR-060 `Sinking` is generic over the inline carrier width;
//! // staying generic here proves the trait path resolves without
//! // pinning a width.
//! fn _accepts_sinking<const INLINE_BYTES: usize, S: Sinking<INLINE_BYTES>>() {}
//! ```

pub use uor_foundation::pipeline::TermValue;
pub use uor_foundation::PrimitiveOp;
pub use uor_foundation::{Term, TermArena, TermList};

// Author-implemented admission and projection traits. Per ADR-014 these
// are vocabulary the application author composes to declare how host
// bytes are admitted into the principal data path (`Grounding`) and how
// `Grounded<T>` values are projected back to host carriers (`Sinking`).
// `GroundingExt` is the foundation-supplied sealed extension trait that
// drives admission via `ground(host_bytes) -> Option<Self::Output>`.
pub use uor_foundation::enforcement::{Grounding, GroundingExt, Sinking};

// `GroundingProgram` is the combinator-builder for `Grounding::program()`,
// the kind-typed program a `Grounding` impl returns. Re-exported so
// authors can name the type without depending on `uor-foundation`'s
// `enforcement` module path directly.
pub use uor_foundation::enforcement::GroundingProgram;
