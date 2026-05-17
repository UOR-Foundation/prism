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
    AffineParity, AndCommitment, ConstrainedTypeShape, EmptyCommitment, FoundationClosed,
    HasChainComplexResolver, HasCochainComplexResolver, HasCohomologyGroupResolver,
    HasHomologyGroupResolver, HasHomotopyGroupResolver, HasKInvariantResolver, HasNerveResolver,
    HasPostnikovResolver, IntoBindingValue, LexicographicLessEqThreshold, NullResolverTuple,
    ObservablePredicate, PipelineFailure, PrismModel, ResolverTuple, SingletonCommitment, Stratum,
    TargetCommitment, TypedCommitment, UltrametricCloseTo, WalshHadamardParity,
};
use prism::seal::Grounded;
use prism::std_types::{ConstrainedTypeInput, GroundedShape};
use prism::vocabulary::{DefaultHostBounds, DefaultHostTypes, Hasher};

// ---- Compile-time bound resolution ----
//
// These generic helpers are never invoked. Their `where` clauses are
// resolved at function-definition time; if any bound regresses, the
// crate fails to compile and the test binary fails to build.

#[allow(dead_code)]
fn _accepts_prism_model<H, M>()
where
    H: Hasher,
    M: PrismModel<DefaultHostTypes, DefaultHostBounds, H>,
{
    // `PrismModel`'s fourth generic `R` defaults to `NullResolverTuple`
    // per ADR-035/036; the 3-param form below uses that default. Foundation
    // 0.4.3 ships a blanket `impl<H: Hasher> AxisTuple for H`, so the
    // `A: AxisTuple + Hasher` bound on the trait is satisfied transitively
    // from `H: Hasher`.
}

#[allow(dead_code)]
fn _associated_type_bounds<H, M>()
where
    H: Hasher,
    M: PrismModel<DefaultHostTypes, DefaultHostBounds, H>,
    M::Input: ConstrainedTypeShape + IntoBindingValue,
    // ADR-035: `Output` now additionally requires `IntoBindingValue` so
    // the runtime can lower the grounded output back into a binding
    // value for downstream composition.
    M::Output: ConstrainedTypeShape + GroundedShape + IntoBindingValue,
    M::Route: FoundationClosed,
{
}

#[allow(dead_code)]
fn _run_route_signature<H, M, R, C>(
    input: M::Input,
    resolvers: &R,
    commitment: &C,
) -> Result<Grounded<M::Output>, PipelineFailure>
where
    H: Hasher,
    M: PrismModel<DefaultHostTypes, DefaultHostBounds, H, R, C>,
    // ADR-035/036: `R: ResolverTuple` is the substrate parameter for
    // the eight categorical-machinery resolvers (Nerve, ChainComplex,
    // HomologyGroup, CochainComplex, CohomologyGroup, Postnikov,
    // HomotopyGroup, KInvariant). `NullResolverTuple` satisfies the
    // arity (=0) but each `Has*Resolver` bound delegates to a null
    // implementation that raises `RESOLVER_ABSENT` when invoked —
    // the default mode for applications that don't supply real resolvers.
    R: ResolverTuple
        + HasNerveResolver<H>
        + HasChainComplexResolver<H>
        + HasHomologyGroupResolver<H>
        + HasCochainComplexResolver<H>
        + HasCohomologyGroupResolver<H>
        + HasPostnikovResolver<H>
        + HasHomotopyGroupResolver<H>
        + HasKInvariantResolver<H>,
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
    prism::pipeline::run_route::<DefaultHostTypes, DefaultHostBounds, H, M, R, C>(
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
    let arena = <ConstrainedTypeInput as FoundationClosed>::arena_slice();
    assert!(arena.is_empty(), "identity model carries no terms");
}

#[test]
fn into_binding_value_resolves_for_constrained_type_input() {
    // `IntoBindingValue::MAX_BYTES` is the on-stack capacity hint per
    // ADR-023; foundation's identity-input impl reports zero bytes.
    // We can't construct `ConstrainedTypeInput` from outside foundation
    // (its single field is private), so we verify the const-evaluable
    // contract — the MAX_BYTES value the trait promises — without
    // calling the method that would require a `&self` we can't mint.
    const MAX: usize = <ConstrainedTypeInput as IntoBindingValue>::MAX_BYTES;
    assert_eq!(
        MAX, 0,
        "identity input has zero MAX_BYTES per foundation 0.3.2"
    );
}
