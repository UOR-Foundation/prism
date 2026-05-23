//! Surface checks for the developer's contract introduced by
//! [ADR-020][09-adr-020] and realized in `uor-foundation` 0.3.2 as
//! [`prism::pipeline::PrismModel`].
//!
//! `PrismModel` is sealed by `__sdk_seal::Sealed` — only the
//! `prism_model!` macro from `uor-foundation-sdk` can mint an impl, so
//! these tests intentionally do not implement it. Two layers of
//! coverage instead:
//!
//! 1. **Compile-time path resolution.** The `use` statements at the
//!    top of this file plus the `_accepts_prism_model` and
//!    `_associated_types` helpers below fail to compile if the
//!    re-exports in `prism::pipeline` regress, if the supertrait /
//!    associated-type bounds disagree with the wiki spec
//!    (`Input: ConstrainedTypeShape + IntoBindingValue`,
//!    `Output: ConstrainedTypeShape + GroundedShape`,
//!    `Route: FoundationClosed`), or if `run_route`'s signature does
//!    not match ADR-022 D5.
//! 2. **Foundation-supplied impls for `ConstrainedTypeInput`.** The
//!    foundation provides `FoundationClosed` and `IntoBindingValue`
//!    impls for the identity input shape; we exercise both, locking in
//!    the contract foundation 0.3.2 commits to.
//!
//! [09-adr-020]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(clippy::unwrap_used, clippy::expect_used)]

use prism::pipeline::{
    primitive_cartesian_nerve_betti, primitive_cartesian_nerve_betti_in,
    primitive_simplicial_nerve_betti, primitive_simplicial_nerve_betti_in, AffineParity,
    AndCommitment, ConstrainedTypeShape, ConstraintRef, EmptyCommitment, EmptyShapeRegistry,
    FoundationClosed, GenericImpossibilityWitness, HasChainComplexResolver,
    HasCochainComplexResolver, HasCohomologyGroupResolver, HasHomologyGroupResolver,
    HasHomotopyGroupResolver, HasKInvariantResolver, HasNerveResolver, HasPostnikovResolver,
    IntoBindingValue, LeafConstraintRef, LexicographicLessEqThreshold, NullResolverTuple,
    ObservablePredicate, PipelineFailure, PrismModel, RegisteredShape, ResolverTuple,
    ShapeRegistryProvider, SingletonCommitment, Stratum, TargetCommitment, TypedCommitment,
    UltrametricCloseTo, WalshHadamardParity, MAX_BETTI_DIMENSION, NERVE_CONSTRAINTS_CAP,
};
use prism::seal::Grounded;
use prism::std_types::CartesianProductShape;
use prism::std_types::{ConstrainedTypeInput, GroundedShape};
use prism::vocabulary::{DefaultHostTypes, Hasher};

// Per ADR-060 the foundation ships no `DefaultHostBounds`; the test
// suite (the "application" here) declares its own. `TestHostBounds`
// lives in `tests/common/mod.rs`.
mod common;
use common::TestHostBounds;

const CARRIER: usize = uor_foundation::pipeline::carrier_inline_bytes::<TestHostBounds>();

// ---- Compile-time bound resolution ----
//
// These generic helpers are never invoked. Their `where` clauses are
// resolved at function-definition time; if any bound regresses, the
// crate fails to compile and the test binary fails to build.

#[allow(dead_code)]
fn _accepts_prism_model<'a, H, M, const FP_MAX: usize>()
where
    H: Hasher<FP_MAX>,
    M: PrismModel<'a, DefaultHostTypes, TestHostBounds, H, CARRIER, FP_MAX>,
{
    // `PrismModel`'s fourth generic `R` defaults to `NullResolverTuple`
    // per ADR-035/036; the 3-param form below uses that default. Foundation
    // 0.4.3 ships a blanket `impl<H: Hasher> AxisTuple for H`, so the
    // `A: AxisTuple + Hasher` bound on the trait is satisfied transitively
    // from `H: Hasher`.
}

#[allow(dead_code)]
fn _associated_type_bounds<'a, H, M, const FP_MAX: usize>()
where
    H: Hasher<FP_MAX>,
    M: PrismModel<'a, DefaultHostTypes, TestHostBounds, H, CARRIER, FP_MAX>,
    // ADR-060: `IntoBindingValue<'a>` now returns a source-polymorphic
    // `TermValue` carrier (Inline/Borrowed/Stream) rather than
    // serializing into a fixed buffer; the `'a` is the borrowed-input
    // lifetime the carrier (and resulting `Grounded<'a>`) propagates.
    M::Input: ConstrainedTypeShape + IntoBindingValue<'a>,
    M::Output: ConstrainedTypeShape + GroundedShape + IntoBindingValue<'a>,
    M::Route: FoundationClosed<CARRIER>,
{
}

#[allow(dead_code)]
fn _run_route_signature<'a, H, M, R, C, const FP_MAX: usize>(
    input: M::Input,
    resolvers: &R,
    commitment: &C,
) -> Result<Grounded<'a, M::Output, CARRIER, FP_MAX>, PipelineFailure>
where
    H: Hasher<FP_MAX> + 'a,
    M: PrismModel<'a, DefaultHostTypes, TestHostBounds, H, CARRIER, FP_MAX, R, C>,
    // ADR-035/036: `R: ResolverTuple` is the substrate parameter for
    // the eight categorical-machinery resolvers (Nerve, ChainComplex,
    // HomologyGroup, CochainComplex, CohomologyGroup, Postnikov,
    // HomotopyGroup, KInvariant). `NullResolverTuple` satisfies the
    // arity (=0) but each `Has*Resolver` bound delegates to a null
    // implementation that raises `RESOLVER_ABSENT` when invoked —
    // the default mode for applications that don't supply real resolvers.
    R: ResolverTuple
        + HasNerveResolver<CARRIER, H>
        + HasChainComplexResolver<CARRIER, H>
        + HasHomologyGroupResolver<CARRIER, H>
        + HasCochainComplexResolver<CARRIER, H>
        + HasCohomologyGroupResolver<CARRIER, H>
        + HasPostnikovResolver<CARRIER, H>
        + HasHomotopyGroupResolver<CARRIER, H>
        + HasKInvariantResolver<CARRIER, H>,
    // ADR-048: `C: TypedCommitment` is the 5th model-declaration
    // parameter — the cost-model commitment surface. The catamorphism
    // evaluates `commitment.evaluate(kappa_label)` after the
    // resolver-bound κ-label is emitted. `EmptyCommitment` is the
    // default and satisfies the bound trivially (it commits to nothing).
    C: TypedCommitment,
{
    // Body is the canonical ADR-022 D5 form; the macro-emitted
    // `PrismModel::forward` expands to exactly this call with R / C
    // defaulting to `NullResolverTuple` / `EmptyCommitment` when the
    // model declares neither resolver use nor a typed commitment.
    prism::pipeline::run_route::<DefaultHostTypes, TestHostBounds, H, M, R, C, CARRIER, FP_MAX>(
        input, resolvers, commitment,
    )
}

/// Compile-time witness that `NullResolverTuple` impls `ResolverTuple`
/// and `EmptyCommitment` impls `TypedCommitment` — the defaults for
/// `PrismModel`/`run_route`'s 4th and 5th parameters. Declaring the
/// functions with these bounds resolves the impls at definition time.
#[allow(dead_code)]
fn accepts_resolver_tuple<R: ResolverTuple>() {}

#[allow(dead_code)]
fn accepts_typed_commitment<C: TypedCommitment>() {}

#[allow(dead_code)]
const NULL_RESOLVER_TUPLE_IS_REACHABLE: fn() = accepts_resolver_tuple::<NullResolverTuple>;

#[allow(dead_code)]
const EMPTY_COMMITMENT_IS_REACHABLE: fn() = accepts_typed_commitment::<EmptyCommitment>;

// ADR-048: the other two foundation-published `TypedCommitment` impls
// (`SingletonCommitment<P>`, `AndCommitment<A, B>`) and the canonical
// `TargetCommitment = SingletonCommitment<LexicographicLessEqThreshold>`
// alias all resolve through the prism façade re-exports.
#[allow(dead_code)]
const SINGLETON_COMMITMENT_IS_REACHABLE: fn() =
    accepts_typed_commitment::<SingletonCommitment<LexicographicLessEqThreshold>>;

#[allow(dead_code)]
const AND_COMMITMENT_IS_REACHABLE: fn() =
    accepts_typed_commitment::<AndCommitment<EmptyCommitment, EmptyCommitment>>;

#[allow(dead_code)]
const TARGET_COMMITMENT_IS_REACHABLE: fn() = accepts_typed_commitment::<TargetCommitment>;

// ADR-049: the foundation-published five `ObservablePredicate` impls
// resolve through the prism façade re-exports.
#[allow(dead_code)]
fn accepts_observable_predicate<P: ObservablePredicate>() {}

#[allow(dead_code)]
const STRATUM_2_IS_OBSERVABLE_PREDICATE: fn() = accepts_observable_predicate::<Stratum<2>>;
#[allow(dead_code)]
const WALSH_HADAMARD_PARITY_IS_OBSERVABLE_PREDICATE: fn() =
    accepts_observable_predicate::<WalshHadamardParity>;
#[allow(dead_code)]
const ULTRAMETRIC_CLOSE_TO_2_IS_OBSERVABLE_PREDICATE: fn() =
    accepts_observable_predicate::<UltrametricCloseTo<2>>;
#[allow(dead_code)]
const AFFINE_PARITY_IS_OBSERVABLE_PREDICATE: fn() = accepts_observable_predicate::<AffineParity>;
#[allow(dead_code)]
const LEXICOGRAPHIC_LESS_EQ_THRESHOLD_IS_OBSERVABLE_PREDICATE: fn() =
    accepts_observable_predicate::<LexicographicLessEqThreshold>;

// ADR-057: the foundation-published shape-IRI registry surface
// (`RegisteredShape`, `ShapeRegistryProvider`, `EmptyShapeRegistry`,
// `lookup_shape`, `lookup_shape_in`) plus the
// `ConstraintRef::Recurse` / `LeafConstraintRef::Recurse` variants
// resolve through the prism façade re-exports.
#[allow(dead_code)]
fn accepts_shape_registry_provider<R: ShapeRegistryProvider>() {}

#[allow(dead_code)]
const EMPTY_SHAPE_REGISTRY_IS_PROVIDER: fn() =
    accepts_shape_registry_provider::<EmptyShapeRegistry>;

// `RegisteredShape` is constructed at link time by the
// `register_shape!` SDK macro; the foundation-owned `lookup_shape`
// path is reserved for future foundation-curated shapes (currently
// returns `None` for any IRI). The const here pins the function
// pointer types so a signature regression breaks the build.
#[allow(dead_code)]
const LOOKUP_SHAPE_SIGNATURE: fn(&str) -> Option<&'static RegisteredShape> =
    prism::pipeline::lookup_shape;

#[allow(dead_code)]
const LOOKUP_SHAPE_IN_EMPTY_SIGNATURE: fn(&str) -> Option<&'static RegisteredShape> =
    prism::pipeline::lookup_shape_in::<EmptyShapeRegistry>;

/// Compile-time witness that the `Recurse` variant is reachable on
/// both `ConstraintRef` and `LeafConstraintRef`. Each constant pins
/// a const value of the variant; if the wiki-spec field shape
/// (`shape_iri: &'static str, descent_bound: u32`) regresses the
/// build fails.
#[allow(dead_code)]
const CONSTRAINT_REF_RECURSE_REACHABLE: ConstraintRef = ConstraintRef::Recurse {
    shape_iri: "https://uor.foundation/test/recurse",
    descent_bound: 0,
};

#[allow(dead_code)]
const LEAF_CONSTRAINT_REF_RECURSE_REACHABLE: LeafConstraintRef = LeafConstraintRef::Recurse {
    shape_iri: "https://uor.foundation/test/recurse",
    descent_bound: 0,
};

// ADR-057 nerve / Betti substrate primitives (foundation 0.4.15
// completes the registry-aware `_in` variants). The function pointer
// constants below pin each primitive's signature so a signature
// regression in foundation breaks our build.
type BettiArray = [u32; MAX_BETTI_DIMENSION];
type BettiResult = Result<BettiArray, GenericImpossibilityWitness>;

#[allow(dead_code)]
const PRIMITIVE_SIMPLICIAL_NERVE_BETTI_SIGNATURE: fn() -> BettiResult =
    primitive_simplicial_nerve_betti::<ConstrainedTypeInput>;

#[allow(dead_code)]
const PRIMITIVE_SIMPLICIAL_NERVE_BETTI_IN_SIGNATURE: fn() -> BettiResult =
    primitive_simplicial_nerve_betti_in::<ConstrainedTypeInput, EmptyShapeRegistry>;

#[allow(dead_code)]
fn accepts_cartesian_betti<S: CartesianProductShape>(_f: fn() -> BettiResult) {}

// `primitive_cartesian_nerve_betti<S>` / `_in<S, R>` resolve at any
// `S: CartesianProductShape`. We bind the function-pointer signature
// in a generic harness so the bounds are checked at definition time
// without minting a concrete cartesian-product shape in this test.
#[allow(dead_code)]
fn bind_cartesian_betti_signatures<S: CartesianProductShape>() {
    accepts_cartesian_betti::<S>(primitive_cartesian_nerve_betti::<S>);
    accepts_cartesian_betti::<S>(primitive_cartesian_nerve_betti_in::<S, EmptyShapeRegistry>);
}

type ExpandConstraintsInFn = fn(
    &[ConstraintRef],
    u32,
    &mut [ConstraintRef; NERVE_CONSTRAINTS_CAP],
    &mut usize,
) -> Result<(), GenericImpossibilityWitness>;

#[allow(dead_code)]
const EXPAND_CONSTRAINTS_IN_SIGNATURE: ExpandConstraintsInFn =
    prism::pipeline::expand_constraints_in::<EmptyShapeRegistry>;

// ---- Runtime checks against foundation-supplied impls ----

#[test]
fn prism_model_surface_compiles() {
    // The fact that this test compiles is the assertion: every helper
    // above resolved its `where` clause against the re-exports in
    // `prism::pipeline`, which means the trait paths and signatures
    // match the wiki spec for ADR-020 + ADR-022.
}

#[test]
fn foundation_closed_resolves_for_constrained_type_input() {
    // `FoundationClosed::arena_slice() -> &'static [Term]` is the
    // route's term-tree witness. Foundation's `ConstrainedTypeInput`
    // impl is the identity model — empty arena.
    let arena = <ConstrainedTypeInput as FoundationClosed<CARRIER>>::arena_slice();
    assert!(arena.is_empty(), "identity model carries no terms");
}

// ADR-060 (foundation 0.5.1): `IntoBindingValue` no longer carries a
// `MAX_BYTES` const + `into_binding_bytes` writer; it returns a
// source-polymorphic `TermValue<'a, INLINE_BYTES>` carrier from
// `as_binding_value`. The identity input shape still satisfies the
// contract — witnessed at definition time by the bound below (we cannot
// construct `ConstrainedTypeInput` from outside foundation to call the
// method, so the trait-impl resolution is the assertion).
#[allow(dead_code)]
fn _accepts_into_binding_value<'a, T: IntoBindingValue<'a>>() {}

#[allow(dead_code)]
const CONSTRAINED_TYPE_INPUT_IS_INTO_BINDING_VALUE: fn() =
    _accepts_into_binding_value::<'static, ConstrainedTypeInput>;

#[test]
fn nerve_betti_primitives_resolve_for_identity_input() {
    // ADR-057 / foundation 0.4.15: the simplicial-nerve Betti primitive
    // and its registry-aware companion both run end-to-end on the
    // identity input shape. `ConstrainedTypeInput` carries an empty
    // constraint set, so the simplicial nerve is the empty complex —
    // Betti numbers (1, 0, 0, …) (one 0-cell connected component, no
    // higher cells). Both call paths through the prism façade must
    // produce the same byte-identical result per ADR-031.
    let plain = primitive_simplicial_nerve_betti::<ConstrainedTypeInput>()
        .expect("identity input fits within nerve caps");
    let registry_aware =
        primitive_simplicial_nerve_betti_in::<ConstrainedTypeInput, EmptyShapeRegistry>()
            .expect("identity input fits within nerve caps even when registry-aware");
    assert_eq!(
        plain, registry_aware,
        "registry-aware variant must agree with plain variant on shapes with no Recurse entries"
    );
    assert_eq!(plain[0], 1, "identity shape has one connected component");
}

#[test]
fn expand_constraints_in_passes_through_non_recurse() {
    // ADR-057 / foundation 0.4.15: `expand_constraints_in::<R>` is the
    // workhorse helper that walks a constraint slice and expands
    // `ConstraintRef::Recurse` entries through `R`'s registry. On an
    // input free of Recurse the output is a verbatim copy.
    let input = [
        ConstraintRef::Site { position: 0 },
        ConstraintRef::Site { position: 1 },
    ];
    let mut out_arr = [ConstraintRef::Site { position: 0 }; NERVE_CONSTRAINTS_CAP];
    let mut out_n: usize = 0;
    prism::pipeline::expand_constraints_in::<EmptyShapeRegistry>(
        &input,
        u32::MAX,
        &mut out_arr,
        &mut out_n,
    )
    .expect("non-Recurse expansion never fails");
    assert_eq!(out_n, 2, "two non-Recurse entries pass through unchanged");
    assert!(matches!(out_arr[0], ConstraintRef::Site { position: 0 }));
    assert!(matches!(out_arr[1], ConstraintRef::Site { position: 1 }));
}
