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
    ConstrainedTypeShape, FoundationClosed, IntoBindingValue, PipelineFailure, PrismModel,
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
}

#[allow(dead_code)]
fn _associated_type_bounds<H, M>()
where
    H: Hasher,
    M: PrismModel<DefaultHostTypes, DefaultHostBounds, H>,
    M::Input: ConstrainedTypeShape + IntoBindingValue,
    M::Output: ConstrainedTypeShape + GroundedShape,
    M::Route: FoundationClosed,
{
}

#[allow(dead_code)]
fn _run_route_signature<H, M>(input: M::Input) -> Result<Grounded<M::Output>, PipelineFailure>
where
    H: Hasher,
    M: PrismModel<DefaultHostTypes, DefaultHostBounds, H>,
{
    // Body is the canonical ADR-022 D5 form; the macro-emitted
    // `PrismModel::forward` expands to exactly this call.
    prism::pipeline::run_route::<DefaultHostTypes, DefaultHostBounds, H, M>(input)
}

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
