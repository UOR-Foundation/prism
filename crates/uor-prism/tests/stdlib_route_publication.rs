//! End-to-end tests for `prism::std_types::RouteShape` and
//! `prism::std_types::RevocationShape` — the publication-graph shapes
//! decentralized-network applications use to publish and revoke routes
//! to UOR-addressed content over a `UorTime` validity discipline.
//!
//! Exercises the contract laid out in
//! [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
//! against `uor-foundation`'s constrained-type admission path:
//!
//! - The trait constants are `const`-evaluable and resolve to the
//!   parametric sums of the const-generic widths (TC-01).
//! - `validate_constrained_type` admits the shapes for a spread of
//!   per-component-width combinations without invoking author-side
//!   runtime logic, satisfying the empty-`CONSTRAINTS` "unconstrained"
//!   reading the foundation's `ConstrainedTypeShape` documentation
//!   specifies.
//! - Distinct width combinations share the IRI but resolve to distinct
//!   `SITE_COUNT` constants — the IRI-namespace rule from
//!   [AGENTS.md § 11.3](../../../AGENTS.md#113-iri-namespace).
//! - `RevocationShape`'s `SITE_COUNT` is exactly `RouteShape`'s
//!   `SITE_COUNT` plus the revoked-label width — the structural
//!   relationship between a route and its revocation.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use prism::pipeline::{validate_constrained_type, ConstrainedTypeShape};
use prism::std_types::{RevocationShape, RouteShape};

// σ-axis label widths admissible in prism 0.3.x (32-byte fingerprint
// floor; see `uor-addr::hash` and the wiki's ADR-047 σ-projection
// hardening axioms). The values below are the κ-label byte widths
// these σ-axes produce per `uor_addr::hash::label_bytes`:
//   sha256    = 71  ("sha256:"    +  1 + 64)
//   blake3    = 71  ("blake3:"    +  1 + 64)
//   sha3-256  = 73  ("sha3-256:"  +  1 + 64)
//   keccak256 = 74  ("keccak256:" +  1 + 64)
const SHA256_LABEL_BYTES: usize = 71;
const BLAKE3_LABEL_BYTES: usize = 71;
const SHA3_256_LABEL_BYTES: usize = 73;
const KECCAK256_LABEL_BYTES: usize = 74;

// A representative application-supplied encoding for the
// (valid-from, valid-until) UorTime pair: 16 bytes per UorTime value
// (8 bytes big-endian f64 Landauer-nats + 8 bytes big-endian u64
// rewrite-steps) yielding 32 bytes for the pair. The foundation
// exposes no fixed wire format; this is the application-architect's
// commitment for these tests.
const TIME_PAIR_BYTES: usize = 32;

// A representative application-supplied endpoint width: a 32-byte
// content-addressable peer identifier. The shape admits any width;
// 32 is illustrative.
const ENDPOINT_BYTES: usize = 32;

#[test]
fn route_shape_site_count_is_const_evaluable() {
    // Given: a route shape specialized over sha256 across all κ-labels,
    //        the test's chosen endpoint and time-pair widths.
    // When: SITE_COUNT is queried in a const context.
    // Then: it equals the sum of the five widths, computed at compile time.
    const SITES_SHA256: usize =
        <RouteShape<71, 32, 32, 71, 71> as ConstrainedTypeShape>::SITE_COUNT;
    const SITES_KECCAK: usize =
        <RouteShape<74, 32, 32, 74, 74> as ConstrainedTypeShape>::SITE_COUNT;
    const SITES_MIXED: usize = <RouteShape<71, 32, 32, 74, 73> as ConstrainedTypeShape>::SITE_COUNT;
    const CONSTRAINTS_LEN: usize =
        <RouteShape<71, 32, 32, 71, 71> as ConstrainedTypeShape>::CONSTRAINTS.len();

    assert_eq!(
        SITES_SHA256,
        SHA256_LABEL_BYTES
            + ENDPOINT_BYTES
            + TIME_PAIR_BYTES
            + SHA256_LABEL_BYTES
            + SHA256_LABEL_BYTES,
    );
    assert_eq!(
        SITES_KECCAK,
        KECCAK256_LABEL_BYTES
            + ENDPOINT_BYTES
            + TIME_PAIR_BYTES
            + KECCAK256_LABEL_BYTES
            + KECCAK256_LABEL_BYTES,
    );
    // Mixed-axis case: sha256-target + keccak256-sig + sha3-256-commit.
    // The shape admits the combination; cross-axis composition is the
    // application's realization-architect commitment.
    assert_eq!(
        SITES_MIXED,
        SHA256_LABEL_BYTES
            + ENDPOINT_BYTES
            + TIME_PAIR_BYTES
            + KECCAK256_LABEL_BYTES
            + SHA3_256_LABEL_BYTES,
    );
    assert_eq!(CONSTRAINTS_LEN, 0);
}

#[test]
fn route_shape_iri_is_shared_across_specializations() {
    // Given: two RouteShape specializations differing in every component width.
    // When: their IRIs are compared.
    // Then: they agree, because the closure-under-foundation rule derives
    //       the IRI from the constraint declaration (empty in both cases),
    //       not from the per-component widths.
    assert_eq!(
        <RouteShape<71, 32, 32, 71, 71> as ConstrainedTypeShape>::IRI,
        <RouteShape<74, 64, 16, 73, 74> as ConstrainedTypeShape>::IRI,
    );
    assert_eq!(
        <RouteShape<71, 32, 32, 71, 71> as ConstrainedTypeShape>::IRI,
        "https://uor.foundation/type/ConstrainedType",
    );
}

#[test]
fn route_shape_blake3_and_sha256_share_site_count() {
    // Given: BLAKE3 and SHA-256 have identical κ-label widths (71 each).
    // When: RouteShape is specialized over them with all other widths equal.
    // Then: the SITE_COUNT values agree — the shape doesn't distinguish
    //       σ-axes by anything other than label width. The σ-axis identity
    //       is carried by the κ-label's wire-form prefix bytes, not by
    //       the shape's SITE_COUNT.
    assert_eq!(
        <RouteShape<{ SHA256_LABEL_BYTES }, 32, 32, { SHA256_LABEL_BYTES }, { SHA256_LABEL_BYTES }>
            as ConstrainedTypeShape>::SITE_COUNT,
        <RouteShape<{ BLAKE3_LABEL_BYTES }, 32, 32, { BLAKE3_LABEL_BYTES }, { BLAKE3_LABEL_BYTES }>
            as ConstrainedTypeShape>::SITE_COUNT,
    );
}

#[test]
fn route_shape_validates_under_compile_time_admission() {
    // Given: a spread of RouteShape specializations.
    // When: each is submitted to validate_constrained_type.
    // Then: admission succeeds via the const path (TC-01 + TC-04),
    //       with no runtime trait dispatch needed.
    validate_constrained_type(RouteShape::<71, 32, 32, 71, 71>)
        .expect("RouteShape<71,32,32,71,71> admits");
    validate_constrained_type(RouteShape::<74, 64, 32, 74, 74>)
        .expect("RouteShape<74,64,32,74,74> admits");
    validate_constrained_type(RouteShape::<73, 16, 24, 71, 74>)
        .expect("mixed-axis RouteShape admits");
}

#[test]
fn revocation_shape_extends_route_shape_by_revoked_label_width() {
    // Given: a RevocationShape with the same first-five widths as a
    //        RouteShape, plus a revoked-label-width parameter.
    // When: SITE_COUNT is compared between the two.
    // Then: the revocation's SITE_COUNT equals the route's plus the
    //       revoked-label width.
    const ROUTE_SITES: usize = <RouteShape<71, 32, 32, 71, 71> as ConstrainedTypeShape>::SITE_COUNT;
    const REV_SITES: usize =
        <RevocationShape<71, 32, 32, 71, 71, 71> as ConstrainedTypeShape>::SITE_COUNT;
    // Cross-axis revocation: a keccak256 publisher revokes a sha256 route.
    // The revocation's signature/commit are keccak256-axis; the
    // REVOKED_LABEL_BYTES is sha256. Declared up-front (before the first
    // statement) per clippy::items_after_statements.
    const X_REV: usize =
        <RevocationShape<74, 32, 32, 74, 74, 71> as ConstrainedTypeShape>::SITE_COUNT;

    assert_eq!(REV_SITES, ROUTE_SITES + SHA256_LABEL_BYTES);
    assert_eq!(
        X_REV,
        KECCAK256_LABEL_BYTES
            + ENDPOINT_BYTES
            + TIME_PAIR_BYTES
            + KECCAK256_LABEL_BYTES
            + KECCAK256_LABEL_BYTES
            + SHA256_LABEL_BYTES,
    );
}

#[test]
fn revocation_shape_iri_matches_route_shape_iri() {
    // Given: RouteShape and RevocationShape over the same first-five widths.
    // When: their IRIs are compared.
    // Then: they agree on the closure-under-foundation class IRI.
    //       The Rust type system distinguishes them — IRI sameness is
    //       expected; instance identity flows through the type system
    //       and the (SITE_COUNT, CONSTRAINTS) triple.
    assert_eq!(
        <RouteShape<71, 32, 32, 71, 71> as ConstrainedTypeShape>::IRI,
        <RevocationShape<71, 32, 32, 71, 71, 71> as ConstrainedTypeShape>::IRI,
    );
}

#[test]
fn revocation_shape_validates_under_compile_time_admission() {
    // Given: a spread of RevocationShape specializations.
    // When: each is submitted to validate_constrained_type.
    // Then: admission succeeds via the const path.
    validate_constrained_type(RevocationShape::<71, 32, 32, 71, 71, 71>)
        .expect("same-axis RevocationShape admits");
    validate_constrained_type(RevocationShape::<74, 32, 32, 74, 74, 71>)
        .expect("cross-axis RevocationShape admits");
}

#[test]
fn route_and_revocation_shapes_remain_distinct_types() {
    // Given: a RouteShape and a RevocationShape whose SITE_COUNT values
    //        happen to agree (synthetic — a route with widened endpoint
    //        matches a revocation with the canonical endpoint width).
    // When: both are submitted to validate_constrained_type.
    // Then: both admit independently; one admission does not imply the
    //       other. The Rust type system distinguishes them even when
    //       byte-count agreement is contrived.
    const ROUTE: usize = <RouteShape<71, 103, 32, 71, 71> as ConstrainedTypeShape>::SITE_COUNT;
    const REV: usize =
        <RevocationShape<71, 32, 32, 71, 71, 71> as ConstrainedTypeShape>::SITE_COUNT;
    assert_eq!(ROUTE, REV);

    validate_constrained_type(RouteShape::<71, 103, 32, 71, 71>).expect("route admits");
    validate_constrained_type(RevocationShape::<71, 32, 32, 71, 71, 71>)
        .expect("revocation admits");
}
