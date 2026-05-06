//! End-to-end coverage for every baseline primitive in `prism::std_types`,
//! per [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy).
//!
//! For each baseline primitive the test suite asserts:
//!
//! 1. The trait constants are `const`-evaluable (TC-01).
//! 2. The IRI lives under the catalog's namespace
//!    (`uor.foundation/prism/std_types/<TypeName>`).
//! 3. `validate_constrained_type` admits the shape via foundation's
//!    preflight gate (feasibility + package coherence) without invoking
//!    any author-side logic.
//! 4. The (IRI, SITE_COUNT, CONSTRAINTS) triple of distinct shapes
//!    diverges where the catalog says it should: `U32` ≠ `I32` by IRI
//!    (paired-pair distinction), `Bool` ≠ `U8` by IRI (semantic
//!    distinction at equal byte width), `Bytes<32>` ≠ `FixedSites<32>`
//!    by IRI (intent distinction at equal structure).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use prism::pipeline::{validate_constrained_type, ConstrainedTypeShape};
use prism::std_types::{
    Bool, Bytes, Char, FixedSites, F32, F64, I128, I16, I256, I32, I64, I8, U128, U16, U256, U32,
    U64, U8,
};

const NS: &str = "uor.foundation/prism/std_types/";

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
fn iris_live_in_the_catalog_namespace() {
    let assert_ns = |iri: &str, suffix: &str| {
        assert_eq!(
            iri,
            format!("{NS}{suffix}"),
            "IRI {iri} not in catalog namespace under {suffix}",
        );
    };
    assert_ns(<U8 as ConstrainedTypeShape>::IRI, "U8");
    assert_ns(<U16 as ConstrainedTypeShape>::IRI, "U16");
    assert_ns(<U32 as ConstrainedTypeShape>::IRI, "U32");
    assert_ns(<U64 as ConstrainedTypeShape>::IRI, "U64");
    assert_ns(<U128 as ConstrainedTypeShape>::IRI, "U128");
    assert_ns(<U256 as ConstrainedTypeShape>::IRI, "U256");
    assert_ns(<I8 as ConstrainedTypeShape>::IRI, "I8");
    assert_ns(<I16 as ConstrainedTypeShape>::IRI, "I16");
    assert_ns(<I32 as ConstrainedTypeShape>::IRI, "I32");
    assert_ns(<I64 as ConstrainedTypeShape>::IRI, "I64");
    assert_ns(<I128 as ConstrainedTypeShape>::IRI, "I128");
    assert_ns(<I256 as ConstrainedTypeShape>::IRI, "I256");
    assert_ns(<F32 as ConstrainedTypeShape>::IRI, "F32");
    assert_ns(<F64 as ConstrainedTypeShape>::IRI, "F64");
    assert_ns(<Bool as ConstrainedTypeShape>::IRI, "Bool");
    assert_ns(<Char as ConstrainedTypeShape>::IRI, "Char");
    assert_ns(<Bytes<32> as ConstrainedTypeShape>::IRI, "Bytes");
    assert_ns(<FixedSites<32> as ConstrainedTypeShape>::IRI, "FixedSites");
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
fn paired_signed_and_unsigned_are_distinct_by_iri() {
    // U32 and I32 share SITE_COUNT but must have distinct content-addresses.
    assert_eq!(
        <U32 as ConstrainedTypeShape>::SITE_COUNT,
        <I32 as ConstrainedTypeShape>::SITE_COUNT,
    );
    assert_ne!(
        <U32 as ConstrainedTypeShape>::IRI,
        <I32 as ConstrainedTypeShape>::IRI,
    );
}

#[test]
fn semantic_aliases_preserve_iri_distinction() {
    // `Bool`, `U8`, `I8`, `Char`'s structural footprint at given widths
    // overlaps with structural shapes — but the catalog's IRI rules
    // keep them content-addressed apart.
    assert_eq!(<Bool as ConstrainedTypeShape>::SITE_COUNT, 1);
    assert_eq!(<U8 as ConstrainedTypeShape>::SITE_COUNT, 1);
    assert_eq!(<I8 as ConstrainedTypeShape>::SITE_COUNT, 1);
    assert_eq!(<FixedSites<1> as ConstrainedTypeShape>::SITE_COUNT, 1);
    assert_ne!(
        <Bool as ConstrainedTypeShape>::IRI,
        <U8 as ConstrainedTypeShape>::IRI,
    );
    assert_ne!(
        <U8 as ConstrainedTypeShape>::IRI,
        <I8 as ConstrainedTypeShape>::IRI,
    );
    assert_ne!(
        <Bool as ConstrainedTypeShape>::IRI,
        <FixedSites<1> as ConstrainedTypeShape>::IRI,
    );
}

#[test]
fn bytes_and_fixed_sites_are_iri_distinct_at_equal_width() {
    assert_eq!(
        <Bytes<32> as ConstrainedTypeShape>::SITE_COUNT,
        <FixedSites<32> as ConstrainedTypeShape>::SITE_COUNT,
    );
    assert_ne!(
        <Bytes<32> as ConstrainedTypeShape>::IRI,
        <FixedSites<32> as ConstrainedTypeShape>::IRI,
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
