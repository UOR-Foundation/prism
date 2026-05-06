//! End-to-end coverage for every baseline primitive in `prism::std_types`,
//! per [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
//! and the closure rule from
//! [Concepts § Closure Under uor-foundation][08-closure].
//!
//! For each baseline primitive the test suite asserts:
//!
//! 1. The trait constants are `const`-evaluable (TC-01).
//! 2. The IRI is the foundation's `ConstrainedType` class IRI — shared
//!    across every empty-constraint stdlib type, **derived from the
//!    constraint declaration, not from the Rust type name** per the
//!    closure rule of ADR-017.
//! 3. `validate_constrained_type` admits the shape via foundation's
//!    preflight gate (feasibility + package coherence) without invoking
//!    any author-side logic.
//! 4. Closure semantics: structurally-identical Rust types
//!    (e.g. `U32` and `I32`, `Bool` and `U8`, `Bytes<32>` and
//!    `FixedSites<32>`) produce **identical** content-addresses through
//!    their shared IRI and equal `(SITE_COUNT, CONSTRAINTS)`. The Rust
//!    type name is for the developer; the IRI is for content-addressing.
//!
//! [08-closure]: https://github.com/UOR-Foundation/UOR-Framework/wiki/08-Concepts#closure-under-uor-foundation

#![allow(clippy::unwrap_used, clippy::expect_used)]

use prism::pipeline::{validate_constrained_type, ConstrainedTypeShape};
use prism::std_types::{
    Bool, Bytes, Char, FixedSites, F32, F64, I128, I16, I256, I32, I64, I8, U128, U16, U256, U32,
    U64, U8,
};

/// The single IRI shared by every empty-constraint stdlib type, per
/// ADR-017 closure: derived from the constraint declaration, not from
/// the Rust type name.
const SHARED_IRI: &str = "https://uor.foundation/type/ConstrainedType";

#[test]
fn integer_byte_widths_match_catalog() {
    // Unsigned family
    assert_eq!(<U8 as ConstrainedTypeShape>::SITE_COUNT, 1);
    assert_eq!(<U16 as ConstrainedTypeShape>::SITE_COUNT, 2);
    assert_eq!(<U32 as ConstrainedTypeShape>::SITE_COUNT, 4);
    assert_eq!(<U64 as ConstrainedTypeShape>::SITE_COUNT, 8);
    assert_eq!(<U128 as ConstrainedTypeShape>::SITE_COUNT, 16);
    assert_eq!(<U256 as ConstrainedTypeShape>::SITE_COUNT, 32);
    // Signed family — same widths, distinct IRIs.
    assert_eq!(<I8 as ConstrainedTypeShape>::SITE_COUNT, 1);
    assert_eq!(<I16 as ConstrainedTypeShape>::SITE_COUNT, 2);
    assert_eq!(<I32 as ConstrainedTypeShape>::SITE_COUNT, 4);
    assert_eq!(<I64 as ConstrainedTypeShape>::SITE_COUNT, 8);
    assert_eq!(<I128 as ConstrainedTypeShape>::SITE_COUNT, 16);
    assert_eq!(<I256 as ConstrainedTypeShape>::SITE_COUNT, 32);
}

#[test]
fn float_byte_widths_match_catalog() {
    assert_eq!(<F32 as ConstrainedTypeShape>::SITE_COUNT, 4);
    assert_eq!(<F64 as ConstrainedTypeShape>::SITE_COUNT, 8);
}

#[test]
fn other_baseline_widths_match_catalog() {
    assert_eq!(<Bool as ConstrainedTypeShape>::SITE_COUNT, 1);
    assert_eq!(<Char as ConstrainedTypeShape>::SITE_COUNT, 4);
    assert_eq!(<Bytes<7> as ConstrainedTypeShape>::SITE_COUNT, 7);
    assert_eq!(<FixedSites<7> as ConstrainedTypeShape>::SITE_COUNT, 7);
}

#[test]
fn every_baseline_iri_is_the_foundation_class_iri() {
    let assert_shared = |iri: &str, name: &str| {
        assert_eq!(
            iri, SHARED_IRI,
            "{name}'s IRI must equal foundation's ConstrainedType class IRI",
        );
    };
    assert_shared(<U8 as ConstrainedTypeShape>::IRI, "U8");
    assert_shared(<U16 as ConstrainedTypeShape>::IRI, "U16");
    assert_shared(<U32 as ConstrainedTypeShape>::IRI, "U32");
    assert_shared(<U64 as ConstrainedTypeShape>::IRI, "U64");
    assert_shared(<U128 as ConstrainedTypeShape>::IRI, "U128");
    assert_shared(<U256 as ConstrainedTypeShape>::IRI, "U256");
    assert_shared(<I8 as ConstrainedTypeShape>::IRI, "I8");
    assert_shared(<I16 as ConstrainedTypeShape>::IRI, "I16");
    assert_shared(<I32 as ConstrainedTypeShape>::IRI, "I32");
    assert_shared(<I64 as ConstrainedTypeShape>::IRI, "I64");
    assert_shared(<I128 as ConstrainedTypeShape>::IRI, "I128");
    assert_shared(<I256 as ConstrainedTypeShape>::IRI, "I256");
    assert_shared(<F32 as ConstrainedTypeShape>::IRI, "F32");
    assert_shared(<F64 as ConstrainedTypeShape>::IRI, "F64");
    assert_shared(<Bool as ConstrainedTypeShape>::IRI, "Bool");
    assert_shared(<Char as ConstrainedTypeShape>::IRI, "Char");
    assert_shared(<Bytes<32> as ConstrainedTypeShape>::IRI, "Bytes");
    assert_shared(<FixedSites<32> as ConstrainedTypeShape>::IRI, "FixedSites");
}

#[test]
fn admission_succeeds_for_every_baseline_type() {
    // Foundation's `validate_constrained_type` runs preflight feasibility
    // and package coherence; an empty `CONSTRAINTS` slice is trivially
    // feasible, so every baseline primitive must pass.
    validate_constrained_type(U8).expect("U8 admissible");
    validate_constrained_type(U16).expect("U16 admissible");
    validate_constrained_type(U32).expect("U32 admissible");
    validate_constrained_type(U64).expect("U64 admissible");
    validate_constrained_type(U128).expect("U128 admissible");
    validate_constrained_type(U256).expect("U256 admissible");
    validate_constrained_type(I8).expect("I8 admissible");
    validate_constrained_type(I16).expect("I16 admissible");
    validate_constrained_type(I32).expect("I32 admissible");
    validate_constrained_type(I64).expect("I64 admissible");
    validate_constrained_type(I128).expect("I128 admissible");
    validate_constrained_type(I256).expect("I256 admissible");
    validate_constrained_type(F32).expect("F32 admissible");
    validate_constrained_type(F64).expect("F64 admissible");
    validate_constrained_type(Bool).expect("Bool admissible");
    validate_constrained_type(Char).expect("Char admissible");
    validate_constrained_type(Bytes::<32>).expect("Bytes<32> admissible");
    validate_constrained_type(FixedSites::<32>).expect("FixedSites<32> admissible");
}

#[test]
fn paired_signed_and_unsigned_share_iri_per_closure() {
    // U32 and I32 share constraint declaration ⇒ same IRI ⇒ same UOR
    // content-address. The Rust name distinguishes intent at the call
    // site; the IRI does not.
    assert_eq!(
        <U32 as ConstrainedTypeShape>::SITE_COUNT,
        <I32 as ConstrainedTypeShape>::SITE_COUNT,
    );
    assert_eq!(
        <U32 as ConstrainedTypeShape>::IRI,
        <I32 as ConstrainedTypeShape>::IRI,
    );
}

#[test]
fn semantic_aliases_share_iri_at_equal_constraints() {
    // `Bool`, `U8`, `I8`, `FixedSites<1>` all have SITE_COUNT=1 and
    // empty CONSTRAINTS — closure says they share an IRI. The Rust
    // type name self-documents intent; the IRI is for addressing.
    assert_eq!(<Bool as ConstrainedTypeShape>::SITE_COUNT, 1);
    assert_eq!(<U8 as ConstrainedTypeShape>::SITE_COUNT, 1);
    assert_eq!(<I8 as ConstrainedTypeShape>::SITE_COUNT, 1);
    assert_eq!(<FixedSites<1> as ConstrainedTypeShape>::SITE_COUNT, 1);
    assert_eq!(
        <Bool as ConstrainedTypeShape>::IRI,
        <U8 as ConstrainedTypeShape>::IRI,
    );
    assert_eq!(
        <U8 as ConstrainedTypeShape>::IRI,
        <I8 as ConstrainedTypeShape>::IRI,
    );
    assert_eq!(
        <Bool as ConstrainedTypeShape>::IRI,
        <FixedSites<1> as ConstrainedTypeShape>::IRI,
    );
}

#[test]
fn bytes_and_fixed_sites_share_iri_at_equal_width() {
    // Per closure: same constraint declaration ⇒ same IRI.
    assert_eq!(
        <Bytes<32> as ConstrainedTypeShape>::SITE_COUNT,
        <FixedSites<32> as ConstrainedTypeShape>::SITE_COUNT,
    );
    assert_eq!(
        <Bytes<32> as ConstrainedTypeShape>::IRI,
        <FixedSites<32> as ConstrainedTypeShape>::IRI,
    );
}

#[test]
fn distinct_site_counts_distinguish_via_constraint_declaration() {
    // Closure says shapes with DIFFERENT constraint declarations have
    // distinguishable content-addresses, even when sharing IRI. The
    // (IRI, SITE_COUNT, CONSTRAINTS) triple — not IRI alone — is what
    // determines the UOR address.
    assert_eq!(
        <U8 as ConstrainedTypeShape>::IRI,
        <U32 as ConstrainedTypeShape>::IRI,
    );
    assert_ne!(
        <U8 as ConstrainedTypeShape>::SITE_COUNT,
        <U32 as ConstrainedTypeShape>::SITE_COUNT,
    );
}

#[test]
fn all_baseline_primitives_have_empty_constraints() {
    // Empty CONSTRAINTS is the catalog rule: value-level invariants
    // (IEEE 754, Bool ∈ {0, 1}, Unicode validity) live host-side.
    assert!(<U8 as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
    assert!(<U256 as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
    assert!(<I256 as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
    assert!(<F32 as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
    assert!(<F64 as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
    assert!(<Bool as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
    assert!(<Char as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
    assert!(<Bytes<32> as ConstrainedTypeShape>::CONSTRAINTS.is_empty());
}
