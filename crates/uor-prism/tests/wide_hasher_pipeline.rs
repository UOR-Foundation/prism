//! Regression: a 64-byte-fingerprint hasher (`Sha512Hasher`, i.e.
//! `Hasher<64>`) must flow through the principal data path.
//!
//! Foundation 0.5.1 pinned the entire resolver/pipeline tower to
//! `Hasher<FP_MAX = 32>` — the `AxisTuple` blanket impl and every ψ-stage
//! resolver trait required `FP_MAX = 32`, so `Sha512Hasher`
//! (`Hasher<64>`) could not be selected as the substrate hasher at all;
//! a downstream project pinned to prism 0.3.1 / foundation 0.5.1 was
//! blocked. Foundation 0.5.2 generalized the tower: the `AxisTuple`
//! blanket is `impl<INLINE_BYTES, FP_MAX, H: Hasher<FP_MAX>>`, the
//! resolver traits take an unbounded `H`, and `run` / `run_route` /
//! `Grounded` / `PrismModel` carry `FP_MAX` as a const parameter.
//!
//! This test grounds a unit through `run` with `Sha512Hasher` at
//! `FP_MAX = 64`, asserts the certificate's fingerprint width is the
//! full 64 bytes, and verifies the QS-05 replay round-trip — the
//! conformance witness that a wide hasher flows end-to-end.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use prism::crypto::Sha512Hasher;
use prism::operation::Term;
use prism::pipeline::run;
use prism::replay::certify_from_trace;
use prism::seal::Validated;
use prism::std_types::ConstrainedTypeInput;
use prism::vocabulary::{CompileUnitBuilder, Hasher, HostBounds, VerificationDomain, WittLevel};

/// An application `HostBounds` whose `FINGERPRINT_MAX_BYTES` is 64,
/// admitting a 64-byte (SHA-512-class) fingerprint. Per ADR-060 the
/// foundation ships no default bounds; this is the test's declaration.
/// Values match the canonical profile except the doubled fingerprint
/// ceiling.
struct Bounds64;
impl HostBounds for Bounds64 {
    const FINGERPRINT_MIN_BYTES: usize = 32;
    const FINGERPRINT_MAX_BYTES: usize = 64;
    const TRACE_MAX_EVENTS: usize = 256;
    const WITT_LEVEL_MAX_BITS: u32 = 64;
    const FOLD_UNROLL_THRESHOLD: usize = 8;
    const BETTI_DIMENSION_MAX: usize = 8;
    const NERVE_CONSTRAINTS_MAX: usize = 8;
    const NERVE_SITES_MAX: usize = 8;
    const JACOBIAN_SITES_MAX: usize = 8;
    const RECURSION_TRACE_DEPTH_MAX: usize = 16;
    const OP_CHAIN_DEPTH_MAX: usize = 8;
    const AFFINE_COEFFS_MAX: usize = 8;
    const CONJUNCTION_TERMS_MAX: usize = 8;
    const UNFOLD_ITERATIONS_MAX: usize = 256;
}

// The inline carrier width derived from the 64-byte-fingerprint bounds,
// and the matching `FP_MAX`.
const CARRIER: usize = uor_foundation::pipeline::carrier_inline_bytes::<Bounds64>();
const FP_MAX: usize = 64;

const ROOT_TERMS: &[Term<'static, CARRIER>] = &[Term::Literal {
    value: prism::operation::TermValue::from_u64_be(7, 1),
    level: WittLevel::W8,
}];
static DOMAINS: &[VerificationDomain] = &[VerificationDomain::Enumerative];

#[test]
fn sha512_hasher_flows_through_the_pipeline() {
    // `Sha512Hasher: Hasher<64>` — the case foundation 0.5.1 rejected.
    let unit: Validated<_> = CompileUnitBuilder::new()
        .root_term(ROOT_TERMS)
        .witt_level_ceiling(WittLevel::W32)
        .thermodynamic_budget(2048)
        .target_domains(DOMAINS)
        .result_type::<ConstrainedTypeInput>()
        .validate()
        .expect("unit well-formed");

    let grounded = run::<ConstrainedTypeInput, _, Sha512Hasher, CARRIER, FP_MAX>(unit)
        .expect("pipeline admits a 64-byte-fingerprint hasher (0.5.2 fix)");

    // The full SHA-512 width flows into the certificate fingerprint —
    // not truncated to the old FP_MAX = 32 ceiling.
    assert_eq!(
        usize::from(grounded.content_fingerprint().width_bytes()),
        <Sha512Hasher as Hasher<64>>::OUTPUT_BYTES,
        "fingerprint width must equal Sha512Hasher::OUTPUT_BYTES (64)",
    );
    assert_eq!(
        usize::from(grounded.content_fingerprint().width_bytes()),
        64
    );

    // QS-05 replay round-trip holds for the wide hasher. `replay`'s
    // `TR_MAX` is the trace-buffer bound (the bounds' `TRACE_MAX_EVENTS`);
    // its `FP_MAX = 64` flows from the `Derivation`, so
    // `certify_from_trace` re-derives a `ContentFingerprint<64>`.
    let trace = grounded
        .derivation()
        .replay::<{ <Bounds64 as HostBounds>::TRACE_MAX_EVENTS }>();
    assert!(usize::from(trace.len()) <= <Bounds64 as HostBounds>::TRACE_MAX_EVENTS);
    let recertified = certify_from_trace(&trace).expect("trace well-formed");
    assert_eq!(
        recertified.certificate().content_fingerprint(),
        grounded.content_fingerprint(),
        "QS-05: re-certified fingerprint must equal source for a 64-byte hasher",
    );
}
