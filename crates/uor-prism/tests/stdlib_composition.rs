//! End-to-end tests for the five composition shapes in
//! `prism::std_types` — `G2ProductShape`, `F4QuotientShape`,
//! `E6FiltrationShape`, `E7AugmentationShape`, `E8EmbeddingShape` —
//! realizing the five categorical operations on the Atlas image
//! inside E₈ per wiki ADR-059 + ADR-061.
//!
//! Exercises the contract laid out in
//! [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
//! against `uor-foundation`'s constrained-type admission path:
//!
//! - Each shape's `SITE_COUNT` is `const`-evaluable and reflects the
//!   shape's natural arity and canonical-form structure per ADR-059's
//!   construction (binary product for G₂ → SITE_COUNT = 2N; unary
//!   operand-preserving for F₄/E₇/E₈ → SITE_COUNT = N; unary
//!   structure-preserving filtration for E₆ → SITE_COUNT = N + 1, the
//!   one-byte degree-partition tag prepended to operand bytes per
//!   wiki ADR-061 §(2)). TC-01.
//! - All five shapes share the closure-under-foundation class IRI
//!   per AGENTS.md § 11.3, distinguished by Rust type identity rather
//!   than by IRI namespace.
//! - `validate_constrained_type` admits every shape for a spread of
//!   σ-axis label widths via the const path, satisfying the
//!   empty-`CONSTRAINTS` "unconstrained" reading.
//! - The shapes are pairwise distinct as Rust types; admission of one
//!   does not imply admission of any other.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use prism::pipeline::{validate_constrained_type, ConstrainedTypeShape};
use prism::std_types::{
    E6FiltrationShape, E7AugmentationShape, E8EmbeddingShape, F4QuotientShape, G2ProductShape,
};

// σ-axis label widths admissible in prism 0.3.x (32-byte fingerprint
// floor; see `uor-addr::hash`):
//   sha256    = 71  ("sha256:"    +  1 + 64)
//   blake3    = 71  ("blake3:"    +  1 + 64)
//   sha3-256  = 73  ("sha3-256:"  +  1 + 64)
//   keccak256 = 74  ("keccak256:" +  1 + 64)
const SHA256_LABEL_BYTES: usize = 71;
const SHA3_256_LABEL_BYTES: usize = 73;
const KECCAK256_LABEL_BYTES: usize = 74;

#[test]
fn g2_product_shape_site_count_is_binary() {
    // Given: G₂'s categorical construction is Klein quartet × ℤ/3, a
    //        binary product of two algebraic objects.
    // When: G2ProductShape is specialized over a per-component byte width.
    // Then: SITE_COUNT equals 2 × COMPONENT_LABEL_BYTES — the shape's
    //       natural arity is 2.
    const G_SHA256: usize = <G2ProductShape<71> as ConstrainedTypeShape>::SITE_COUNT;
    const G_SHA3: usize = <G2ProductShape<73> as ConstrainedTypeShape>::SITE_COUNT;
    const G_KECCAK: usize = <G2ProductShape<74> as ConstrainedTypeShape>::SITE_COUNT;

    assert_eq!(G_SHA256, 2 * SHA256_LABEL_BYTES);
    assert_eq!(G_SHA3, 2 * SHA3_256_LABEL_BYTES);
    assert_eq!(G_KECCAK, 2 * KECCAK256_LABEL_BYTES);
    assert!(<G2ProductShape<71> as ConstrainedTypeShape>::CONSTRAINTS.is_empty(),);
}

#[test]
fn unary_shape_site_counts_per_adr_061_section_2() {
    // Given: F₄/E₇/E₈'s categorical constructions are unary
    //        (quotient/augmentation/embedding of one Atlas structure)
    //        with canonical-form width = operand width;
    //        E₆'s filtration is unary but structure-preserving,
    //        prepending a one-byte degree-partition tag to the operand
    //        bytes per wiki ADR-061 §(2).
    // When: each unary shape is specialized over a per-component byte width.
    // Then: F₄/E₇/E₈ SITE_COUNT equals COMPONENT_LABEL_BYTES (operand
    //       width preserved); E₆ SITE_COUNT equals
    //       COMPONENT_LABEL_BYTES + 1 (degree-partition tag + operand).
    const F4: usize = <F4QuotientShape<71> as ConstrainedTypeShape>::SITE_COUNT;
    const E6: usize = <E6FiltrationShape<71> as ConstrainedTypeShape>::SITE_COUNT;
    const E7: usize = <E7AugmentationShape<71> as ConstrainedTypeShape>::SITE_COUNT;
    const E8: usize = <E8EmbeddingShape<71> as ConstrainedTypeShape>::SITE_COUNT;

    assert_eq!(F4, SHA256_LABEL_BYTES);
    assert_eq!(E6, SHA256_LABEL_BYTES + 1);
    assert_eq!(E7, SHA256_LABEL_BYTES);
    assert_eq!(E8, SHA256_LABEL_BYTES);
}

#[test]
fn unary_shapes_widen_with_sigma_axis() {
    // Given: keccak256's κ-label is 74 bytes; sha256's is 71;
    //        sha3-256's is 73.
    // When: each unary shape is specialized over a wider σ-axis.
    // Then: F₄/E₇/E₈ SITE_COUNT scales linearly with operand width
    //       (preserves operand bytes); E₆ scales as 1 + operand width
    //       (prepends one-byte degree-partition tag per CA-6).
    assert_eq!(
        <F4QuotientShape<74> as ConstrainedTypeShape>::SITE_COUNT,
        KECCAK256_LABEL_BYTES,
    );
    assert_eq!(
        <E6FiltrationShape<74> as ConstrainedTypeShape>::SITE_COUNT,
        KECCAK256_LABEL_BYTES + 1,
    );
    assert_eq!(
        <E8EmbeddingShape<73> as ConstrainedTypeShape>::SITE_COUNT,
        SHA3_256_LABEL_BYTES,
    );
}

#[test]
fn all_composition_shapes_share_the_closure_iri() {
    // Given: the five composition shapes per AGENTS.md § 11.3.
    // When: their IRIs are compared.
    // Then: all agree on the closure-under-foundation class IRI,
    //       because the closure rule derives the IRI from the
    //       constraint declaration (empty in all cases), not from
    //       Rust type identity.
    const IRI: &str = "https://uor.foundation/type/ConstrainedType";
    assert_eq!(<G2ProductShape<71> as ConstrainedTypeShape>::IRI, IRI);
    assert_eq!(<F4QuotientShape<71> as ConstrainedTypeShape>::IRI, IRI);
    assert_eq!(<E6FiltrationShape<71> as ConstrainedTypeShape>::IRI, IRI);
    assert_eq!(<E7AugmentationShape<71> as ConstrainedTypeShape>::IRI, IRI);
    assert_eq!(<E8EmbeddingShape<71> as ConstrainedTypeShape>::IRI, IRI);
}

#[test]
fn composition_shapes_admit_under_compile_time_admission() {
    // Given: a spread of composition-shape specializations across σ-axes.
    // When: each is submitted to validate_constrained_type.
    // Then: admission succeeds via the const path (TC-01 + TC-04),
    //       with no runtime trait dispatch needed.
    validate_constrained_type(G2ProductShape::<71>).expect("G2ProductShape<71> admits");
    validate_constrained_type(G2ProductShape::<74>).expect("G2ProductShape<74> admits");

    validate_constrained_type(F4QuotientShape::<71>).expect("F4QuotientShape<71> admits");
    validate_constrained_type(F4QuotientShape::<73>).expect("F4QuotientShape<73> admits");

    validate_constrained_type(E6FiltrationShape::<71>).expect("E6FiltrationShape<71> admits");
    validate_constrained_type(E6FiltrationShape::<74>).expect("E6FiltrationShape<74> admits");

    validate_constrained_type(E7AugmentationShape::<71>).expect("E7AugmentationShape<71> admits");
    validate_constrained_type(E7AugmentationShape::<73>).expect("E7AugmentationShape<73> admits");

    validate_constrained_type(E8EmbeddingShape::<71>).expect("E8EmbeddingShape<71> admits");
    validate_constrained_type(E8EmbeddingShape::<74>).expect("E8EmbeddingShape<74> admits");
}

#[test]
fn composition_shapes_are_pairwise_distinct_types() {
    // Given: F₄, E₇, E₈ at sha256-width all carry SITE_COUNT = 71
    //        (operand-width-preserving); E₆ carries SITE_COUNT = 72
    //        (degree-partition tag + operand) per wiki ADR-061 §(2).
    // When: their numerical SITE_COUNTs are compared.
    // Then: F₄/E₇/E₈ agree numerically; E₆ is one byte wider than the
    //       other three. The Rust type system distinguishes all five
    //       shapes regardless — admission of one does not imply
    //       admission of another, and downstream realizations
    //       pattern-match on the specific shape, not on the SITE_COUNT.
    const F4: usize = <F4QuotientShape<71> as ConstrainedTypeShape>::SITE_COUNT;
    const E6: usize = <E6FiltrationShape<71> as ConstrainedTypeShape>::SITE_COUNT;
    const E7: usize = <E7AugmentationShape<71> as ConstrainedTypeShape>::SITE_COUNT;
    const E8: usize = <E8EmbeddingShape<71> as ConstrainedTypeShape>::SITE_COUNT;
    assert_eq!(F4, E7);
    assert_eq!(E7, E8);
    assert_eq!(
        E6,
        F4 + 1,
        "E₆ is one byte wider than the operand-preserving unaries"
    );

    // Validate each independently — the type system's distinction
    // means admission of one is not admission of another.
    validate_constrained_type(F4QuotientShape::<71>).expect("F₄ admits");
    validate_constrained_type(E6FiltrationShape::<71>).expect("E₆ admits");
    validate_constrained_type(E7AugmentationShape::<71>).expect("E₇ admits");
    validate_constrained_type(E8EmbeddingShape::<71>).expect("E₈ admits");
}

#[test]
fn g2_product_distinct_from_unary_shapes() {
    // Given: G₂ at sha256-width has SITE_COUNT = 142 (binary);
    //        the unary shapes at sha256-width have SITE_COUNT = 71.
    // When: G₂'s SITE_COUNT is compared to the unary shapes'.
    // Then: G₂ is strictly wider, reflecting its binary-product
    //       structure per ADR-059.
    const G2: usize = <G2ProductShape<71> as ConstrainedTypeShape>::SITE_COUNT;
    const F4: usize = <F4QuotientShape<71> as ConstrainedTypeShape>::SITE_COUNT;
    // `G2 > F4` is a compile-time fact (a bare `assert!` over it would be
    // optimized out per clippy::assertions_on_constants); pin it as a
    // const assertion so a regression fails the build, not the test run.
    const _: () = assert!(G2 > F4);
    assert_eq!(G2, 2 * F4);
}
