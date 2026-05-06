//! End-to-end exercise of the principal data path and its trace-replay
//! round-trip, going through `prism`'s public namespace exclusively.
//!
//! The test mirrors the wiki's
//! [Runtime View § Scenario 1: Principal Data Path Execution][06-scenario-1]
//! and [Scenario 2: Trace-Replay Verification][06-scenario-2]. It proves
//! the round-trip equivalence required by QS-05: replay → certify_from_trace
//! produces a `Certified` whose certificate is bit-identical to the one
//! emitted by `pipeline::run`.
//!
//! [06-scenario-1]: https://github.com/UOR-Foundation/UOR-Framework/wiki/06-Runtime-View#scenario-1-principal-data-path-execution
//! [06-scenario-2]: https://github.com/UOR-Foundation/UOR-Framework/wiki/06-Runtime-View#scenario-2-trace-replay-verification

#![allow(clippy::unwrap_used, clippy::expect_used)]

use prism::operation::Term;
use prism::pipeline::run;
use prism::replay::certify_from_trace;
use prism::seal::Validated;
use prism::std_types::ConstrainedTypeInput;
use prism::vocabulary::{CompileUnitBuilder, Hasher, VerificationDomain, WittLevel};

/// A minimal 16-byte FNV-1a substrate hasher: two 64-bit lanes.
///
/// This is the canonical reference implementation surfaced in the
/// `uor_foundation::enforcement::Hasher` trait documentation. It is
/// deterministic, side-effect-free, and exposes the full `Hasher`
/// contract without relying on any external crate.
#[derive(Clone, Copy)]
struct Fnv1a16 {
    a: u64,
    b: u64,
}

impl Hasher for Fnv1a16 {
    const OUTPUT_BYTES: usize = 16;

    fn initial() -> Self {
        Self {
            a: 0xcbf2_9ce4_8422_2325,
            b: 0x8422_2325_cbf2_9ce4,
        }
    }

    fn fold_byte(mut self, x: u8) -> Self {
        self.a ^= u64::from(x);
        self.a = self.a.wrapping_mul(0x100_0000_01b3);
        self.b ^= u64::from(x).rotate_left(8);
        self.b = self.b.wrapping_mul(0x100_0000_01b3);
        self
    }

    // `Hasher` is `Hasher<const FP_MAX: usize = 32>` in foundation 0.3.1;
    // the default `FP_MAX = 32` matches `<DefaultHostBounds as
    // HostBounds>::FINGERPRINT_MAX_BYTES`.
    fn finalize(self) -> [u8; 32] {
        let mut buf = [0u8; 32];
        buf[..8].copy_from_slice(&self.a.to_be_bytes());
        buf[8..16].copy_from_slice(&self.b.to_be_bytes());
        buf
    }
}

static ROOT_TERMS: &[Term] = &[Term::Literal {
    value: 7,
    level: WittLevel::W8,
}];
static DOMAINS: &[VerificationDomain] = &[VerificationDomain::Enumerative];

#[test]
fn pipeline_run_then_replay_roundtrip() {
    // Given: a well-formed `CompileUnit` built through the foundation
    // surface as the application author would.
    let builder = CompileUnitBuilder::new()
        .root_term(ROOT_TERMS)
        .witt_level_ceiling(WittLevel::W32)
        .thermodynamic_budget(2048)
        .target_domains(DOMAINS)
        .result_type::<ConstrainedTypeInput>();
    let unit: Validated<_> = builder.validate().expect("unit well-formed");

    // When: `prism::pipeline::run` consumes the unit with the FNV-1a
    // substrate, producing a sealed `Grounded<T>`.
    let grounded = run::<ConstrainedTypeInput, _, Fnv1a16>(unit).expect("pipeline admits");

    // And: the grounded value's derivation is replayed into a `Trace`
    // at the foundation's default `HostBounds` capacity
    // (`<DefaultHostBounds as HostBounds>::TRACE_MAX_EVENTS == 256`),
    // and the trace alone is fed through `prism::replay::certify_from_trace`.
    let trace: prism::replay::Trace = grounded.derivation().replay();
    let recertified = certify_from_trace(&trace).expect("trace is well-formed");

    // Then: the re-certified fingerprint matches the source grounded
    // value's fingerprint bit-for-bit, satisfying QS-05 (replay
    // equivalence) without invoking any hash function during replay.
    assert_eq!(
        recertified.certificate().content_fingerprint(),
        grounded.content_fingerprint(),
        "QS-05: re-certified fingerprint must equal the source fingerprint",
    );
    assert_eq!(
        recertified.certificate().witt_bits(),
        grounded.witt_level_bits(),
        "QS-05: re-certified witt_bits must equal the source witt_level_bits",
    );
    assert!(
        !trace.is_empty(),
        "Scenario 1 emits at least one event when the pipeline admits",
    );
}

#[test]
fn empty_trace_is_rejected_with_typed_error() {
    use prism::replay::{ReplayError, Trace};

    // Given: the simplest deterministic input — an empty trace at the
    // foundation's default `HostBounds` capacity (`Trace<256>`).
    let trace: Trace = Trace::empty();

    // When: certify_from_trace runs structural validation.
    let outcome = certify_from_trace(&trace);

    // Then: it returns the canonical typed error, satisfying TC-05's
    // "verifier never invokes deciders or hash functions" property —
    // structural validation alone produces the rejection.
    assert!(matches!(outcome, Err(ReplayError::EmptyTrace)));
}
