//! Scaling tests for the wiki's substitution axes.
//!
//! Per the wiki's [Architecture Constraints § Substitution Axes][02-axes]
//! and [Quality Requirements][10-qs], a conformant Prism implementation
//! must admit any well-formed `(HostTypes, HostBounds, Hasher, T)` tuple
//! and produce a [round-trip][06-scenario-2] whose certificate is
//! bit-identical to the source `Grounded<T>`'s certificate. These tests
//! falsify any regression of that property by running the principal
//! data path → replay → `certify_from_trace` round-trip across a
//! representative spread of:
//!
//! - **`Hasher::OUTPUT_BYTES`** widths within `DefaultHostBounds`'s
//!   `[FINGERPRINT_MIN_BYTES, FINGERPRINT_MAX_BYTES]` range:
//!   16 (minimum), 24 (intermediate), 32 (maximum).
//! - **Witt-level ceilings** spanning the named family `W8`/`W16`/`W32`
//!   and the boundary level `W64` (the default `HostBounds`'s
//!   `WITT_LEVEL_MAX_BITS`).
//!
//! The third axis named in the wiki — `HostTypes` — is held at
//! `DefaultHostTypes` because foundation 0.3.1's `pipeline::run` is
//! parameterized over `<T, P, H>` and inherits `HostTypes` through the
//! application's own crate-level type alias rather than the call site.
//! `HostBounds` is similarly held at `DefaultHostBounds` for the same
//! reason: `Hasher`, `Trace`, and `ContentFingerprint` resolve their
//! const generics to the default profile when called through the
//! foundation-supplied `pipeline::run` entry point.
//!
//! Per the boundary established with `prism-btc`: this test does not
//! ship reference Hasher implementations of any cryptographic substrate
//! (BLAKE3, SHA-256, etc.); per ADR-014 prism does not ship operation
//! libraries. The local `Fnv1a*` impls below are *test fuel only* —
//! their structural form follows the Hasher trait's normative example
//! and exists solely to vary `OUTPUT_BYTES` along the scaling axis.
//!
//! Per [TR-05][11-tr-05] (hasher selection mismatch produces verification
//! failure indistinguishable from data corruption), the spread also
//! exercises the foundation's normative width-tag invariant on
//! `ContentFingerprint`: differing `OUTPUT_BYTES` widths must yield
//! distinguishable certificates even when the leading bytes coincide.
//!
//! [02-axes]: https://github.com/UOR-Foundation/UOR-Framework/wiki/02-Architecture-Constraints
//! [10-qs]: https://github.com/UOR-Foundation/UOR-Framework/wiki/10-Quality-Requirements#quality-scenarios
//! [06-scenario-2]: https://github.com/UOR-Foundation/UOR-Framework/wiki/06-Runtime-View#scenario-2-trace-replay-verification
//! [11-tr-05]: https://github.com/UOR-Foundation/UOR-Framework/wiki/11-Technical-Risks#tr-05--hasher-selection-mismatch-produces-verification-failure-indistinguishable-from-data-corruption

#![allow(clippy::unwrap_used, clippy::expect_used)]

mod common;

use common::{Fnv16, Fnv24, Fnv32};
use prism::operation::Term;
use prism::pipeline::run;
use prism::replay::{certify_from_trace, Trace};
use prism::seal::Validated;
use prism::std_types::ConstrainedTypeInput;
use prism::vocabulary::{
    CompileUnitBuilder, DefaultHostBounds, Hasher, HostBounds, VerificationDomain, WittLevel,
};

// ---- Generic round-trip property ----

static ROOT_TERMS: &[Term] = &[Term::Literal {
    value: 7,
    level: WittLevel::W8,
}];
static DOMAINS: &[VerificationDomain] = &[VerificationDomain::Enumerative];

/// Run the principal data path with hasher `H` and Witt-level ceiling
/// `witt_ceiling`, then assert all four wiki-named round-trip
/// invariants:
///
/// 1. The pipeline admits the unit (TC-03 singular path).
/// 2. The certificate's content fingerprint width equals
///    `H::OUTPUT_BYTES` (Hasher contract).
/// 3. The trace fits within `<DefaultHostBounds as HostBounds>::TRACE_MAX_EVENTS`
///    (HostBounds capacity contract).
/// 4. `certify_from_trace`'s certificate is bit-identical to the source
///    grounded value's certificate (QS-05).
fn assert_roundtrip<H: Hasher>(witt_ceiling: WittLevel) {
    // (1) Build → validate → run.
    let builder = CompileUnitBuilder::new()
        .root_term(ROOT_TERMS)
        .witt_level_ceiling(witt_ceiling)
        .thermodynamic_budget(2048)
        .target_domains(DOMAINS)
        .result_type::<ConstrainedTypeInput>();
    let unit: Validated<_> = builder.validate().expect("unit well-formed");
    let grounded = run::<ConstrainedTypeInput, _, H>(unit).expect("pipeline admits");

    // (2) Hasher contract: width recorded on the fingerprint matches the
    // hasher's declared `OUTPUT_BYTES`.
    assert_eq!(
        usize::from(grounded.content_fingerprint().width_bytes()),
        H::OUTPUT_BYTES,
        "fingerprint width must equal Hasher::OUTPUT_BYTES",
    );

    // (3) HostBounds capacity contract: the trace cannot exceed
    // `TRACE_MAX_EVENTS`. We assert against the typed bound rather than
    // the literal 256 so this test would fail loudly if a future
    // `HostBounds` impl reduces the cap.
    let trace: Trace = grounded.derivation().replay();
    assert!(
        usize::from(trace.len()) <= <DefaultHostBounds as HostBounds>::TRACE_MAX_EVENTS,
        "trace length exceeds HostBounds::TRACE_MAX_EVENTS",
    );

    // (4) Replay equivalence (QS-05): structural validation alone re-derives
    // a certificate bit-identical to the pipeline's.
    let recertified = certify_from_trace(&trace).expect("trace well-formed");
    assert_eq!(
        recertified.certificate().content_fingerprint(),
        grounded.content_fingerprint(),
        "QS-05: re-certified fingerprint must equal source",
    );
    assert_eq!(
        recertified.certificate().witt_bits(),
        grounded.witt_level_bits(),
        "QS-05: re-certified witt_bits must equal source",
    );
}

// ---- Matrix: 3 Hasher widths × 4 Witt-level ceilings = 12 cases ----
//
// Each row pins one `Hasher::OUTPUT_BYTES` value and walks the Witt-level
// axis; each column does the dual. Together they cover every cell of the
// (Hasher × WittLevel) sub-matrix that the wiki's substitution-axis
// contract names.

#[test]
fn fnv16_w8() {
    assert_roundtrip::<Fnv16>(WittLevel::W8);
}

#[test]
fn fnv16_w16() {
    assert_roundtrip::<Fnv16>(WittLevel::W16);
}

#[test]
fn fnv16_w32() {
    assert_roundtrip::<Fnv16>(WittLevel::W32);
}

#[test]
fn fnv16_w64_boundary() {
    // `W64` equals `<DefaultHostBounds as HostBounds>::WITT_LEVEL_MAX_BITS`
    // — the boundary the substitution-axis contract names as the cap of
    // the default capacity profile.
    assert_roundtrip::<Fnv16>(WittLevel::new(64));
}

#[test]
fn fnv24_w8() {
    assert_roundtrip::<Fnv24>(WittLevel::W8);
}

#[test]
fn fnv24_w16() {
    assert_roundtrip::<Fnv24>(WittLevel::W16);
}

#[test]
fn fnv24_w32() {
    assert_roundtrip::<Fnv24>(WittLevel::W32);
}

#[test]
fn fnv24_w64_boundary() {
    assert_roundtrip::<Fnv24>(WittLevel::new(64));
}

#[test]
fn fnv32_w8() {
    assert_roundtrip::<Fnv32>(WittLevel::W8);
}

#[test]
fn fnv32_w16() {
    assert_roundtrip::<Fnv32>(WittLevel::W16);
}

#[test]
fn fnv32_w32() {
    assert_roundtrip::<Fnv32>(WittLevel::W32);
}

#[test]
fn fnv32_w64_boundary() {
    assert_roundtrip::<Fnv32>(WittLevel::new(64));
}

// ---- Cross-axis invariant: different OUTPUT_BYTES at the same Witt
// level produce *different* fingerprints, so the width tag is part of
// the certificate's identity (foundation contract — see
// `ContentFingerprint`'s width-tag discussion).

#[test]
fn fingerprints_at_different_widths_are_distinguishable() {
    fn fresh_unit() -> Validated<prism::vocabulary::CompileUnit<'static>> {
        CompileUnitBuilder::new()
            .root_term(ROOT_TERMS)
            .witt_level_ceiling(WittLevel::W32)
            .thermodynamic_budget(2048)
            .target_domains(DOMAINS)
            .result_type::<ConstrainedTypeInput>()
            .validate()
            .expect("unit well-formed")
    }

    let g16 = run::<ConstrainedTypeInput, _, Fnv16>(fresh_unit()).expect("admits");
    let g24 = run::<ConstrainedTypeInput, _, Fnv24>(fresh_unit()).expect("admits");
    let g32 = run::<ConstrainedTypeInput, _, Fnv32>(fresh_unit()).expect("admits");

    let f16 = g16.content_fingerprint();
    let f24 = g24.content_fingerprint();
    let f32 = g32.content_fingerprint();

    // Width-tag invariant from the foundation: two certificates with
    // different `OUTPUT_BYTES` widths are NEVER equal, even if their
    // leading bytes coincide.
    assert_ne!(f16, f24);
    assert_ne!(f24, f32);
    assert_ne!(f16, f32);
}
