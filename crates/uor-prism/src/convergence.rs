//! The Hopf convergence tower: the operator-geometry codomain of κ-derivation.
//!
//! `convergence` re-exports the foundation's ontology-derived
//! [`kernel::convergence`][05-kernel] namespace (`Space::Kernel`; IRI
//! `https://uor.foundation/convergence/`) so application authors reach the
//! convergence-tower substrate vocabulary through the single `prism`
//! dependency per wiki [ADR-031][09]. Per [ADR-059][09] this tower is the
//! **operator-geometry-coordinate codomain stratification** of κ-derivation —
//! the framework's compression-to-canonical-form operator per
//! [ADR-058][09]. A canonical form's tower level (R / C / H / O) is the
//! algebra dimension at which its operator-geometry coordinates carry a
//! normed-division-algebra structure; descent O → H → C → R under
//! compression pressure surrenders the characteristic identities
//! (self-reference → choice → feedback → existence) in reverse order of
//! acquisition.
//!
//! # The tower
//!
//! Four [`ConvergenceLevel`] instances sit at the division-algebra
//! dimensions {1, 2, 4, 8} of the four normed division algebras:
//!
//! | Level | Algebra | Dim | Characteristic identity | [`HopfFiber`] |
//! |-------|---------|-----|-------------------------|---------------|
//! | R | reals | 1 | existence | S⁰ |
//! | C | complexes | 2 | feedback | S¹ |
//! | H | quaternions | 4 | choice | S³ |
//! | O | octonions | 8 | self-reference | S⁷ |
//!
//! Each level carries a [`HopfFiber`] (the Hopf fibration fiber sphere), a
//! Betti signature, and a characteristic identity. [`ConvergenceResidual`]
//! carries the persistent β\_{2^k−1} = 1 Betti number that survives at each
//! level. [`CommutativeSubspace`] selects the U(1) ⊂ SU(2) subspace at
//! pairwise-interaction convergence; [`AssociativeSubalgebra`] selects the
//! H ⊂ O subalgebra at triple-interaction convergence. The per-level
//! constants are surfaced through the [`l0_state`], [`l1_memory`],
//! [`l2_agency`], [`l3_self`] submodules (the four convergence levels) and
//! the [`hopf_s0`], [`hopf_s1`], [`hopf_s3`], [`hopf_s7`] submodules (the
//! four Hopf fibers).
//!
//! # Codomain-typed admission predicates
//!
//! Per [ADR-059][09] point (5), the convergence tower supplies a
//! codomain-typed vocabulary for application-level typed-commitment
//! surfaces per [ADR-048][09]: a canonical form's
//! `(ConvergenceLevel, ConvergenceResidual)` signature is admissible as a
//! structural predicate composed through
//! [`pipeline::AndCommitment`][crate::pipeline::AndCommitment], and
//! Atlas-image proximity is realizable through
//! [`SingletonCommitment<UltrametricCloseTo<K>>`][crate::pipeline::SingletonCommitment].
//!
//! # Resolver families
//!
//! Each tower trait carries a resolver family mirroring the foundation's
//! orphan-closure discipline: a `*Resolver` trait, a `*Handle` /
//! `*Record` carrier pair, a `Resolved*` wrapper, and a `Null*`
//! resolver-absent baseline. The namespace is `@generated` from the UOR
//! ontology (Amendment 66 — 5 classes, 13 properties, 8 individuals); the
//! `Null*` baselines close the trait orphans for substrates that declare no
//! convergence resolvers.
//!
//! # See also
//!
//! - [Wiki: 05 Building Block View — the `kernel` component][05-kernel]
//! - [Wiki: 08 Concepts — compression-to-canonical-form and the codomain][08]
//! - [Wiki: 09 Architecture Decisions — ADR-058 / ADR-059][09]
//! - [Wiki: 12 Glossary — Hopf convergence tower][12]
//!
//! # Constraints
//!
//! - **ADR-031** — `prism` IS the standard library; this module re-exports
//!   the foundation `kernel::convergence` substrate vocabulary so it is
//!   reachable through `use prism::convergence::*;`
//! - **ADR-058** — κ-derivation is the framework's
//!   compression-to-canonical-form operator
//! - **ADR-059** — the Hopf convergence tower is the operator-geometry
//!   codomain stratification of κ-derivation
//!
//! [05-kernel]: https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View
//! [08]: https://github.com/UOR-Foundation/UOR-Framework/wiki/08-Concepts
//! [09]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [12]: https://github.com/UOR-Foundation/UOR-Framework/wiki/12-Glossary

// The four tower traits plus the residual / subspace / subalgebra traits.
// `ConvergenceLevel<H>` carries the algebra dimension, Betti signature,
// Hopf fiber, characteristic identity, and level name; `HopfFiber<H>`
// carries the fibration's dimensions and sphere designations;
// `ConvergenceResidual<H>` carries the persistent β\_{2^k−1} = 1 Betti
// number; `CommutativeSubspace<H>` / `AssociativeSubalgebra<H>` carry the
// pairwise- and triple-interaction subspace selections.
pub use uor_foundation::kernel::convergence::{
    AssociativeSubalgebra, CommutativeSubspace, ConvergenceLevel, ConvergenceResidual, HopfFiber,
};

// `Null*` resolver-absent baselines per the foundation's orphan-closure
// discipline — the conformant default for substrates that declare no
// convergence resolvers.
pub use uor_foundation::kernel::convergence::{
    NullAssociativeSubalgebra, NullCommutativeSubspace, NullConvergenceLevel,
    NullConvergenceResidual, NullHopfFiber,
};

// Resolver traits — the substrate hooks an application's resolver tuple
// implements to make the tower's structural readings available to the
// principal data path.
pub use uor_foundation::kernel::convergence::{
    AssociativeSubalgebraResolver, CommutativeSubspaceResolver, ConvergenceLevelResolver,
    ConvergenceResidualResolver, HopfFiberResolver,
};

// Handle / Record carrier pairs and `Resolved*` wrappers — the
// resolver-protocol carriers paralleling the ψ-resolver families of
// ADR-036.
pub use uor_foundation::kernel::convergence::{
    AssociativeSubalgebraHandle, CommutativeSubspaceHandle, ConvergenceLevelHandle,
    ConvergenceResidualHandle, HopfFiberHandle,
};
pub use uor_foundation::kernel::convergence::{
    AssociativeSubalgebraRecord, CommutativeSubspaceRecord, ConvergenceLevelRecord,
    ConvergenceResidualRecord, HopfFiberRecord,
};
pub use uor_foundation::kernel::convergence::{
    ResolvedAssociativeSubalgebra, ResolvedCommutativeSubspace, ResolvedConvergenceLevel,
    ResolvedConvergenceResidual, ResolvedHopfFiber,
};

// Per-level constant submodules: the four convergence levels carry
// `ALGEBRA_DIMENSION`, `BETTI_SIGNATURE`, `CHARACTERISTIC_IDENTITY`,
// `FIBER_TYPE`, `LEVEL_NAME`; the four Hopf fibers carry `BASE_SPACE`,
// `FIBER_DIMENSION`, `FIBER_SPHERE`, `TOTAL_SPACE`.
pub use uor_foundation::kernel::convergence::{hopf_s0, hopf_s1, hopf_s3, hopf_s7};
pub use uor_foundation::kernel::convergence::{l0_state, l1_memory, l2_agency, l3_self};
