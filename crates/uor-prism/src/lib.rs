//! `prism` — the Prism **standard library** (wiki ADR-031).
//!
//! Per [Wiki ADR-031][09-adr-031], the `prism` crate is the standard
//! library: a **façade** re-exporting [`uor_foundation`]'s substrate
//! together with the built-in axes and built-in types the Layer-3
//! sub-crates of the standard library declare. The wiki's user-facing
//! promise is _"depend on `uor-prism`, write a `prism_model!`, done"_
//! — application authors do not need to add `uor-foundation`,
//! `uor-foundation-sdk`, or the individual `uor-prism-<domain>` crates
//! to their dependency list. Everything composes through
//! `use prism::*;` (or finer-grained imports like
//! `use prism::crypto::Sha256Hasher`).
//!
//! Per [ADR-024][09-adr-024], the standard library is **Layer 2** of
//! the three-layer algebraic closure spine: substrate (`uor-foundation`)
//! is Layer 1; verbs and axes declared by application crates are
//! Layer 3.
//!
//! # Standard-library Layer-3 sub-crate roster
//!
//! Per ADR-031's roster commitment, the standard library publishes
//! four canonical sub-crates from the Prism repository, each consumed
//! through the façade re-exports below. Every axis impl is parametric
//! in its natural axis (byte-width, Q-format split, hasher, dimension)
//! so application authors instantiate the impl their model needs
//! without re-rolling the kernel body; canonical type aliases (e.g.,
//! `Sha256Hasher`, `BigInt256Numeric`) name the most common
//! instantiations, while parametric impls (e.g.,
//! `CpuI8MatmulSquare<DIM>`) are instantiated at the application's
//! chosen dimension.
//!
//! - **[`crypto`]** — wiki: hashes, curves, signatures, commitments.
//!   `HashAxis` impls: `Sha256Hasher`, `Sha512Hasher`,
//!   `Sha3_256Hasher`, `Keccak256Hasher`, `Blake3Hasher`.
//!   `CommitmentAxis` impl: `MerkleRoot<H, LEAF_BYTES>` (parametric
//!   over `HashAxis`), default alias `MerkleRootCommitment` = SHA-256.
//!   Shapes: `Digest<N>`, `PublicKey<N>`, `Signature<N>`,
//!   `MerkleProofShape<MAX_DEPTH, LEAF_BYTES>`.
//! - **[`numerics`]** — wiki: integer, fixed-point, prime-field,
//!   GF(2) arithmetic. Parametric: `BigIntModularNumeric<BYTES>`
//!   (8..=512 bits), `FixedPointQNumeric<I, F>` (any Q-format split
//!   ≤ 64 bits total), `Gf2NumericAxisN<BYTES>` (1..=128 bytes).
//!   Concrete: `PrimeFieldNumericSecp256k1`. Shapes: `BigIntShape<N>`,
//!   `FixedPointShape<I, F>`, `FieldElementShape<N>`,
//!   `Gf2RingShape<N>`.
//! - **[`tensor`]** — wiki: tensor compute + activations. Parametric:
//!   `CpuI8MatmulSquare<DIM>` (1..=16 square `i8`→`i16` matmul),
//!   `CpuI8VectorActivation<N>` (1..=256-length `i8` vector ReLU /
//!   Q1.7 sigmoid). Shapes: `MatrixShape<R, C, ELEM_BYTES>`,
//!   `VectorShape<N, ELEM_BYTES>`.
//! - **[`fhe`]** — wiki: homomorphic encryption. Parametric reference
//!   impl: `OneTimePadFhe<BLOCK_BYTES>` (1..=256). Shape:
//!   `CiphertextShape<N>`.
//!
//! # SDK macros (re-exported)
//!
//! Per ADR-031's façade commitment, the SDK macros declared by
//! `uor-foundation-sdk` are re-exported through [`pipeline`] so a
//! single `use prism::pipeline::prism_model;` reaches the canonical
//! application-author surface:
//!
//! - `prism_model!` (ADR-020 / ADR-022) — declare a typed route
//! - `verb!` (ADR-024) — declare a Layer-3 verb (named composition)
//! - `axis!` (ADR-030) — declare a Layer-3 axis (substrate-extension
//!   vocabulary)
//! - `resolver!` (ADR-036) — declare a `ResolverTuple` value
//! - `output_shape!` (ADR-027) — declare a custom Output shape
//! - `use_verbs!` (ADR-024) — import verbs from another implementation
//! - `product_shape!`, `coproduct_shape!`, `cartesian_product_shape!`,
//!   `partition_product!`, `partition_coproduct!` (ADR-026 / ADR-033)
//!   — shape constructors
//!
//! # See also
//!
//! - [Wiki: 01 Introduction and Goals](https://github.com/UOR-Foundation/UOR-Framework/wiki/01-Introduction-and-Goals)
//! - [Wiki: 04 Solution Strategy](https://github.com/UOR-Foundation/UOR-Framework/wiki/04-Solution-Strategy)
//! - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
//! - [Wiki: 06 Runtime View § Scenario 1: Principal Data Path Execution](https://github.com/UOR-Foundation/UOR-Framework/wiki/06-Runtime-View#scenario-1-principal-data-path-execution)
//! - [Wiki: 09 Architecture Decisions § ADR-024 — Three-layer algebraic closure][09-adr-024]
//! - [Wiki: 09 Architecture Decisions § ADR-030 — `axis!` SDK macro][09-adr-030]
//! - [Wiki: 09 Architecture Decisions § ADR-031 — `prism` is the standard library][09-adr-031]
//! - [Wiki: 10 Quality Requirements § Quality Scenarios](https://github.com/UOR-Foundation/UOR-Framework/wiki/10-Quality-Requirements#quality-scenarios)
//! - [Wiki: 12 Glossary](https://github.com/UOR-Foundation/UOR-Framework/wiki/12-Glossary)
//! - [Wiki: Conceptual Model § SD](https://github.com/UOR-Foundation/UOR-Framework/wiki/Conceptual-Model#sd) — OPM (ISO 19450) overall system diagram naming the three actors and Prism as the system-of-interest
//! - [Wiki: Conceptual Model § SD1 Prism Structure](https://github.com/UOR-Foundation/UOR-Framework/wiki/Conceptual-Model#sd1-prism-structure) — OPM decomposition of Prism into Substrate, Runtime, and Replay Surface
//!
//! # Constraints
//!
//! This crate is normatively bound by:
//!
//! - **TC-01** — zero-cost runtime; no Prism interpreter layer at execution
//! - **TC-02** — sealing of `Validated`, `Grounded`, `Certified` via the Rust
//!   type system, enforced through `pub(crate)` constructors in the substrate
//! - **TC-03** — singular principal data path; exactly one constructor for
//!   `Grounded<T>`, reached only through [`pipeline::run`]
//! - **TC-04** — bilateral compile-time UORassembly enforcement
//! - **TC-05** — replayability without invoking author deciders or hash
//!   functions; surfaced through [`replay::certify_from_trace`]
//! - **TC-06** — no application-author infrastructure at runtime
//! - **ADR-006** — UORassembly is enforced bilaterally at compile time
//!   through the Rust type system; this is the architectural commitment
//!   that makes TC-04 enforceable rather than aspirational
//!
//! Substitution axes per ADR-007/030/036: `HostTypes`, `HostBounds`,
//! `AxisTuple`, `ResolverTuple`, `TypedCommitment`.
//!
//! Additionally:
//!
//! - **ADR-019** — `uor-foundation`'s vocabulary is the signature
//!   category, `Term` is its initial algebra, [`pipeline::run`] is the
//!   catamorphism.
//! - **ADR-020** — application authors declare a Prism application by
//!   implementing the sealed [`pipeline::PrismModel`] trait; the
//!   `prism_model!` macro derives `forward`'s body via initiality.
//! - **ADR-024** — three-layer algebraic closure (substrate, prism,
//!   implementation).
//! - **ADR-030** — the `axis!` SDK macro is the universal
//!   substrate-extension declaration mechanism.
//! - **ADR-031** — `prism` IS the standard library: a façade over
//!   `uor-foundation` plus Layer-3 sub-crates published from the
//!   Prism repository.
//! - **ADR-040** — closed `BoundShape` catalog (7 individuals) with
//!   `type:LexicographicLessEqBound` for byte-sequence-valued
//!   observables; 1:1 correspondence with the foundation-published
//!   `ObservablePredicate` impl surface per ADR-049.
//! - **ADR-048** — typed-commitment substrate: the `TypedCommitment`
//!   trait with three built-in impls (`EmptyCommitment`,
//!   `SingletonCommitment<P>`, `AndCommitment<A, B>`) and the canonical
//!   `TargetCommitment = SingletonCommitment<LexicographicLessEqThreshold>`
//!   alias. The 5th model-declaration parameter `C` on `PrismModel`.
//! - **ADR-049** — five foundation-published typed UOR observable
//!   primitives (`Stratum<P>`, `WalshHadamardParity`,
//!   `UltrametricCloseTo<P>`, `AffineParity`,
//!   `LexicographicLessEqThreshold`) realizing the four taxonomy
//!   subclasses of ADR-038's closed observable catalog. Each is
//!   `Copy + Sealed` and consumable as a `SingletonCommitment<P>`
//!   operand per ADR-048.
//! - **ADR-057** — bounded recursive structural typing via
//!   `ConstraintRef::Recurse { shape_iri, descent_bound }` plus the
//!   foundation shape-IRI registry (`RegisteredShape`,
//!   `ShapeRegistryProvider`, `EmptyShapeRegistry`, `lookup_shape`,
//!   `lookup_shape_in`). Apps emit a registry via the `register_shape!`
//!   SDK macro; `partition_product!` / `partition_coproduct!` operand
//!   grammar admits `recurse[(<bound>)]:T` to declare recursive
//!   references without const-eval cycles. The registry-aware
//!   nerve / Betti substrate primitives shipped in foundation 0.4.15 —
//!   `primitive_simplicial_nerve_betti_in::<T, R>`,
//!   `primitive_cartesian_nerve_betti_in::<S, R>`, and
//!   `expand_constraints_in::<R>` — walk Recurse entries through `R`'s
//!   registry plus foundation's built-in registry, giving the
//!   structurally-correct nerve / Betti reading of recursively-expanded
//!   constraint sets. Wire-format trace events gain a `Recurse`
//!   discriminant; `TRACE_REPLAY_FORMAT_VERSION` bumps to 10.
//! - **ADR-058** — κ-derivation (the eight-resolver ψ-pipeline composed
//!   with the ψ_9 σ-projection) **is** the framework's
//!   compression-to-canonical-form operator; the κ-label is the
//!   minimum-information canonical-form representation, with a three-tier
//!   closure-lossless taxonomy (T1 byte-identical ⇒ T2 κ-label-identical
//!   ⇒ T3 outcome-coarse-equivalent). A conceptual-reading commitment over
//!   existing constructs — no new substrate surface.
//! - **ADR-059** — the operator-geometry codomain of κ-derivation is the
//!   Atlas image inside E₈, coarsely stratified by the Hopf convergence
//!   tower (the foundation's [`kernel::convergence`] namespace, surfaced
//!   through [`convergence`]): four `ConvergenceLevel` instances R / C /
//!   H / O at division-algebra dimensions {1, 2, 4, 8}. A
//!   conceptual-reading commitment over the foundation's existing
//!   `kernel::convergence` substrate vocabulary — no new substrate
//!   surface.
//!
//! [`kernel::convergence`]: uor_foundation::kernel::convergence
//!
//! # C4 placement
//!
//! Container `prism` (Level 2) of the Prism system. The submodules
//! mirror the Level 2 components named in the wiki's
//! [Building Block View § Whitebox `prism`][05-prism], with the
//! ADR-031-introduced standard-library Layer-3 sub-crate re-exports
//! sitting beside the foundation runtime surface:
//!
//! - [`pipeline`] — the principal data path + SDK macro re-exports
//! - [`seal`] — the sealed Prism-mechanism types
//! - [`replay`] — trace-replay verification surface
//! - [`operation`] — operation declaration vocabulary
//! - [`std_types`] — standard type library (baseline primitives)
//! - [`vocabulary`] — foundation surface re-exports
//! - [`convergence`] — the Hopf convergence tower: κ-derivation's
//!   operator-geometry codomain (ADR-058 / ADR-059)
//! - [`crypto`] — standard-library cryptography axes (ADR-031)
//! - [`numerics`] — standard-library numerics axes (ADR-031)
//! - [`tensor`] — standard-library tensor-compute axes (ADR-031)
//! - [`fhe`] — standard-library homomorphic-encryption axes (ADR-031)
//!
//! # Behavior
//!
//! ```rust
//! // Given: the substrate dependency `uor-foundation` is in scope
//! // When:  the prism standard-library façade is loaded
//! // Then:  every wiki Level 2 module of `prism` AND every ADR-031
//! //        standard-library sub-crate is reachable through `use prism::*;`
//! use prism::{convergence as _, operation as _, pipeline as _, replay as _};
//! use prism::{seal as _, std_types as _, vocabulary as _};
//! use prism::{crypto as _, fhe as _, numerics as _, tensor as _};
//! use uor_foundation as _;
//! assert_eq!(prism::WIKI, "https://github.com/UOR-Foundation/UOR-Framework/wiki");
//! ```
//!
//! [wiki]: https://github.com/UOR-Foundation/UOR-Framework/wiki
//! [05-prism]: https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism
//! [09-adr-024]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-030]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use uor_foundation;

pub mod convergence;
pub mod operation;
pub mod pipeline;
pub mod replay;
pub mod seal;
pub mod std_types;
pub mod vocabulary;

// Wiki ADR-031: the Prism standard library's Layer-3 sub-crates.
// Re-exported as `prism::<domain>` so application authors reach them
// through the single `prism` dependency.
pub mod crypto {
    //! Cryptography axes per [Wiki ADR-031][09-adr-031]
    //! (re-export of `uor-prism-crypto`).
    //!
    //! Application authors compose `HashAxis`, `CurveAxis`,
    //! `SignatureAxis`, and `CommitmentAxis` through their model's
    //! `AxisTuple` per ADR-030. Five canonical `HashAxis` impls are
    //! provided as the standard-library reference: SHA-256, SHA-512,
    //! SHA3-256, Keccak-256, and BLAKE3.
    //!
    //! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
    pub use prism_crypto::*;
}

pub mod numerics {
    //! Numerics axes per [Wiki ADR-031][09-adr-031]
    //! (re-export of `uor-prism-numerics`).
    //!
    //! Provides `BigIntAxis`, `FixedPointAxis`, `FieldAxis`, and
    //! `RingAxis`. Reference impls cover 256-bit modular arithmetic,
    //! Q32.32 fixed-point, secp256k1 prime-field arithmetic, and
    //! GF(2) over 256-bit operands.
    //!
    //! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
    pub use prism_numerics::*;
}

pub mod tensor {
    //! Tensor-compute axes per [Wiki ADR-031][09-adr-031]
    //! (re-export of `uor-prism-tensor`).
    //!
    //! Provides `TensorAxis` and `ActivationAxis` with fixed-shape
    //! CPU integer-precision reference impls. Variable-rank tensor
    //! compute composes through verbs over
    //! `partition_product!`-declared shapes (ADR-033/044).
    //!
    //! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
    pub use prism_tensor::*;
}

pub mod fhe {
    //! Homomorphic-encryption axes per [Wiki ADR-031][09-adr-031]
    //! (re-export of `uor-prism-fhe`).
    //!
    //! Provides `FheAxis` with a reference one-time-pad impl. Production
    //! FHE schemes (TFHE, BGV, CKKS) are operational policy per ADR-031.
    //!
    //! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
    pub use prism_fhe::*;
}

/// Canonical URL of the UOR-Framework wiki, the normative source for the
/// Prism architecture realized by this crate.
///
/// Every public item in `prism` carries a backlink to a wiki section that
/// roots at this URL. Consumers may reference this constant when surfacing
/// the same origin programmatically — for example, in error messages that
/// direct users to the architectural section that defines a violated
/// invariant.
///
/// # See also
///
/// - [Wiki: Home](https://github.com/UOR-Foundation/UOR-Framework/wiki)
///
/// # Constraints
///
/// - **CV-02** — code identifiers appear in monospace without paraphrase;
///   this constant is the single source of truth for the wiki origin
///
/// # Behavior
///
/// ```rust
/// // Given: prism is loaded
/// // When:  the wiki URL constant is read
/// // Then:  it points at the UOR-Framework wiki landing page
/// assert!(prism::WIKI.starts_with("https://"));
/// assert!(prism::WIKI.ends_with("/UOR-Framework/wiki"));
/// ```
pub const WIKI: &str = "https://github.com/UOR-Foundation/UOR-Framework/wiki";

/// Minimum supported Rust version of this crate.
///
/// Pinned to track `uor-foundation`'s effective MSRV so the dependency
/// graph never imposes a tighter requirement on consumers than the
/// substrate itself. Bumping this constant requires bumping the workspace
/// `rust-version` and the `rust-toolchain.toml` channel in lockstep.
///
/// # See also
///
/// - [Wiki: 02 Architecture Constraints](https://github.com/UOR-Foundation/UOR-Framework/wiki/02-Architecture-Constraints)
///
/// # Constraints
///
/// - **TC-04** — bilateral compile-time enforcement assumes a single,
///   declared toolchain version on both sides of the contract
///
/// # Behavior
///
/// ```rust
/// // Given: the MSRV constant
/// // When:  parsed into its semver components
/// // Then:  it is at least 1.83 and uses the major.minor form
/// let parts: Vec<&str> = prism::MSRV.split('.').collect();
/// assert_eq!(parts.len(), 2);
/// let major: u32 = parts[0].parse().expect("major version is numeric");
/// let minor: u32 = parts[1].parse().expect("minor version is numeric");
/// assert!((major, minor) >= (1, 83));
/// ```
pub const MSRV: &str = "1.83";
