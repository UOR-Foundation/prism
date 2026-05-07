//! End-to-end tests for `prism::std_types::FixedSites<N>` — the structural
//! parametric building block under every other empty-constraint stdlib
//! shape (Bytes<N>, U8 … I256, F32, F64, Bool, Char). The broader
//! baseline catalog is covered by `tests/stdlib_primitives.rs`; this
//! file pins the FixedSites-specific contract.
//!
//! Exercises the contract laid out in
//! [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
//! against `uor-foundation`'s constrained-type admission path:
//!
//! - The trait constants are `const`-evaluable and resolve to the
//!   parametric values (TC-01).
//! - `validate_constrained_type` admits `FixedSites<N>` for a spread of
//!   `N` values without invoking author-side runtime logic, satisfying
//!   the empty-`CONSTRAINTS` "unconstrained" reading the foundation's
//!   `ConstrainedTypeShape` documentation specifies.
//! - Distinct `N` instantiations share the IRI but resolve to distinct
//!   `SITE_COUNT` constants, exercising the IRI-namespace rule from
//!   [AGENTS.md § 11.3](../../../AGENTS.md#113-iri-namespace).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use prism::pipeline::{validate_constrained_type, ConstrainedTypeShape};
use prism::std_types::FixedSites;

#[test]
fn site_count_is_const_evaluable() {
    // Trait constants must be evaluable in const context — TC-01 zero-cost
    // runtime is realized by every admission decision happening at compile
    // time.
    const N32: usize = <FixedSites<32> as ConstrainedTypeShape>::SITE_COUNT;
    const N80: usize = <FixedSites<80> as ConstrainedTypeShape>::SITE_COUNT;
    const N1: usize = <FixedSites<1> as ConstrainedTypeShape>::SITE_COUNT;
    const EMPTY_CONSTRAINTS: usize = <FixedSites<32> as ConstrainedTypeShape>::CONSTRAINTS.len();

    assert_eq!(N32, 32);
    assert_eq!(N80, 80);
    assert_eq!(N1, 1);
    assert_eq!(EMPTY_CONSTRAINTS, 0);
}

#[test]
fn iri_is_shared_across_instantiations() {
    assert_eq!(
        <FixedSites<1> as ConstrainedTypeShape>::IRI,
        <FixedSites<1024> as ConstrainedTypeShape>::IRI,
        "IRI must identify the shape family, not the instance",
    );
    assert_eq!(
        <FixedSites<32> as ConstrainedTypeShape>::IRI,
        "https://uor.foundation/type/ConstrainedType",
        "per ADR-017 closure: every empty-constraint stdlib type shares \
         the foundation's `ConstrainedType` class IRI",
    );
}

#[test]
fn validate_admits_a_spread_of_n() {
    // Each instantiation must pass foundation's admission — preflight
    // feasibility + package coherence — with no runtime side-effects.
    // Spread covers a typical scalar (32), a Bitcoin block-header width
    // (80), and a single-site degenerate case.
    validate_constrained_type(FixedSites::<32>).expect("FixedSites<32> admissible");
    validate_constrained_type(FixedSites::<80>).expect("FixedSites<80> admissible");
    validate_constrained_type(FixedSites::<1>).expect("FixedSites<1> admissible");
    validate_constrained_type(FixedSites::<256>).expect("FixedSites<256> admissible");
}

#[test]
fn distinct_n_resolve_to_distinct_site_counts() {
    // Identity-via-(IRI, SITE_COUNT, CONSTRAINTS): same IRI, different
    // SITE_COUNT ⇒ distinct shape per ADR-017.
    let n_a = <FixedSites<32> as ConstrainedTypeShape>::SITE_COUNT;
    let n_b = <FixedSites<33> as ConstrainedTypeShape>::SITE_COUNT;
    assert_ne!(n_a, n_b);
    assert_eq!(
        <FixedSites<32> as ConstrainedTypeShape>::IRI,
        <FixedSites<33> as ConstrainedTypeShape>::IRI,
    );
}
