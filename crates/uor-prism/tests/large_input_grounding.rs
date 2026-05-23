//! Large-input grounding: content-addressing inputs whose byte length
//! exceeds the ADR-060 inline carrier width (`INLINE_BYTES`).
//!
//! # Background
//!
//! Per wiki ADR-060 the byte width of a value carrier is an application
//! concern: large structured payloads (model-weight container formats,
//! multi-GB tensor-data sections, canonical-JSON documents) are
//! content-addressed by their hash, not materialized into a fixed
//! buffer. Foundation 0.5.1 completed the input side of ADR-060 —
//! `IntoBindingValue::as_binding_value` now returns the source-
//! polymorphic `TermValue<'a, INLINE_BYTES>` carrier (`Inline` for
//! values within the derived inline width, `Borrowed` for larger
//! in-memory values, `Stream` for unbounded sources), and `run_route`
//! consumes that carrier directly with **no `INLINE_BYTES` cap** (the
//! pre-0.5.1 `MAX_BYTES`-overflow rejection is gone): an input shape
//! whose `as_binding_value` returns `Borrowed`/`Stream` flows through
//! the convenience `prism_model!` path unbounded.
//!
//! # The explicit-binding path this test exercises
//!
//! This test exercises the lower-level path — content-addressing a
//! large input by hash and binding it directly — which works
//! independently of the model surface and is the most direct
//! demonstration that the κ-derivation admits arbitrarily large inputs:
//!
//! 1. **Stream-hash** the full input through the application's [`Hasher`]
//!    via [`Hasher::fold_bytes`] — chunk-by-chunk, never materializing
//!    more than the hasher's own state. The input may be any size.
//! 2. Take the leading 8 bytes of the digest as the binding's
//!    `content_address: u64` (the same truncation `run_route` applies
//!    internally).
//! 3. Construct a [`Binding`] for the route's input slot
//!    (`name_index = 0` per ADR-022 D3 G2) carrying that content address
//!    and the input shape's IRI.
//! 4. Build the unit with [`CompileUnitBuilder::bindings`], validate, and
//!    [`run`] it. `run` folds the *unit structure* — including the
//!    binding's content address — into the `Grounded`'s certificate, so
//!    the large input's identity flows into the κ-derivation without the
//!    raw bytes ever sitting in a fixed buffer.
//!
//! This test grounds an input three orders of magnitude larger than the
//! inline carrier width, asserts the result is `Grounded` (no
//! rejection), and verifies the QS-05 replay round-trip — proving the
//! architecture content-addresses large inputs end-to-end through
//! prism's re-exported foundation surface.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

mod common;

use prism::crypto::Sha256Hasher;
use prism::operation::{Term, TermValue};
use prism::pipeline::run;
use prism::replay::{certify_from_trace, Trace};
use prism::seal::Validated;
use prism::std_types::ConstrainedTypeInput;
use prism::vocabulary::{
    Binding, CompileUnitBuilder, Hasher, HostBounds, VerificationDomain, WittLevel,
};
use uor_foundation::enforcement::GroundedShape;
use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef, IntoBindingValue};

const CARRIER: usize = uor_foundation::pipeline::carrier_inline_bytes::<common::TestHostBounds>();

static DOMAINS: &[VerificationDomain] = &[VerificationDomain::Enumerative];

// The identity route: `Term::Variable { name_index: 0 }` returns the
// route-input slot (ADR-022 D3 G2), whose binding carries the large
// input's streamed content address.
const ROUTE: &[Term<'static, CARRIER>] = &[Term::Variable { name_index: 0 }];

/// The test inputs (≥64 KiB) must exceed the inline carrier width for
/// the demonstration to be meaningful — asserted at compile time.
const _: () = assert!(
    64 * 1024 > CARRIER,
    "test inputs must exceed the inline carrier width",
);

/// Stream-hash an arbitrarily large input through the application's
/// `Hasher` and take the leading-8-byte big-endian `u64` content
/// address — exactly the truncation `run_route` applies internally,
/// but with the full input folded chunk-by-chunk (never materialized).
fn content_address_of<H: Hasher>(input: &[u8]) -> u64 {
    let digest = H::initial().fold_bytes(input).finalize();
    u64::from_be_bytes([
        digest[0], digest[1], digest[2], digest[3], digest[4], digest[5], digest[6], digest[7],
    ])
}

/// Ground a `large_input` of arbitrary size and assert the QS-05
/// round-trip holds.
fn ground_large_input<H: Hasher>(large_input: &[u8]) {
    // (1)+(2) Stream-hash the full input → content address. No fixed
    // buffer; `large_input` may be any length.
    let content_address = content_address_of::<H>(large_input);

    // (3) Bind it to the route's input slot. `Binding` is a
    // public-fielded foundation type; the content address is the
    // application's streaming hash of the full input.
    let input_binding = [Binding {
        name_index: 0,
        type_index: 0,
        value_index: 0,
        surface: <ConstrainedTypeInput as ConstrainedTypeShape>::IRI,
        content_address,
    }];

    // (4) Build → validate → run. The identity route carries no terms;
    // the binding carries the large input's identity.
    let builder = CompileUnitBuilder::new()
        .root_term(ROUTE)
        .bindings(&input_binding)
        .witt_level_ceiling(WittLevel::W32)
        .thermodynamic_budget(4096)
        .target_domains(DOMAINS)
        .result_type::<ConstrainedTypeInput>();
    let unit: Validated<_> = builder
        .validate()
        .expect("unit well-formed for large input");
    let grounded =
        run::<ConstrainedTypeInput, _, H, CARRIER, 32>(unit).expect("pipeline admits large input");

    // The grounded fingerprint width equals the hasher's output width.
    assert_eq!(
        usize::from(grounded.content_fingerprint().width_bytes()),
        H::OUTPUT_BYTES,
    );

    // QS-05 replay equivalence: structural validation re-derives a
    // bit-identical certificate.
    let trace: Trace = grounded.derivation().replay();
    assert!(usize::from(trace.len()) <= <common::TestHostBounds as HostBounds>::TRACE_MAX_EVENTS);
    let recertified = certify_from_trace(&trace).expect("trace well-formed");
    assert_eq!(
        recertified.certificate().content_fingerprint(),
        grounded.content_fingerprint(),
        "QS-05: re-certified fingerprint must equal source for a large input",
    );
}

#[test]
fn grounds_input_far_larger_than_inline_carrier() {
    // 100 KiB — ~1000× the inline carrier width (which the module-level
    // `const` assertion guards). The streaming-hash + explicit-binding
    // path content-addresses it; per ADR-060 / foundation 0.5.1 the
    // `run_route` convenience path also handles inputs of this size via
    // an `as_binding_value` returning `Borrowed`/`Stream`.
    let large_input: Vec<u8> = (0..100 * 1024).map(|i| (i % 251) as u8).collect();
    ground_large_input::<Sha256Hasher>(&large_input);
}

#[test]
fn distinct_large_inputs_ground_to_distinct_addresses() {
    // Content-addressing soundness: two different large inputs must
    // produce different binding content addresses (hence distinct units).
    let a: Vec<u8> = (0..64 * 1024).map(|i| (i % 251) as u8).collect();
    let mut b = a.clone();
    *b.last_mut().unwrap() ^= 0xff; // flip one byte in the last block
    assert_ne!(
        content_address_of::<Sha256Hasher>(&a),
        content_address_of::<Sha256Hasher>(&b),
        "distinct large inputs must content-address distinctly",
    );
    // And both ground successfully.
    ground_large_input::<Sha256Hasher>(&a);
    ground_large_input::<Sha256Hasher>(&b);
}

// ---- The canonical 0.5.1 carrier path: as_binding_value -> Borrowed ----
//
// An application input shape that borrows an arbitrarily large byte
// region and returns it as a `TermValue::Borrowed` carrier from
// `as_binding_value`. This is the mechanism foundation 0.5.1 added to
// complete the ADR-060 input path: the carrier admits the full input
// with no inline-width truncation, and `run_route` consumes it directly
// (no `INLINE_BYTES` cap). The shape is a foundation-vocabulary
// `ConstrainedTypeShape` with the generic content-addressed IRI per
// ADR-017.
struct BorrowedInput<'a> {
    data: &'a [u8],
}

impl ConstrainedTypeShape for BorrowedInput<'_> {
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = 0;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    const CYCLE_SIZE: u64 = 1;
}
impl uor_foundation::pipeline::__sdk_seal::Sealed for BorrowedInput<'_> {}
impl GroundedShape for BorrowedInput<'_> {}
impl<'a> IntoBindingValue<'a> for BorrowedInput<'a> {
    fn as_binding_value<const INLINE_BYTES: usize>(&self) -> TermValue<'a, INLINE_BYTES> {
        // The full input borrows zero-copy into the carrier — no
        // materialization, no inline-width ceiling.
        TermValue::borrowed(self.data)
    }
}

#[test]
fn as_binding_value_borrows_arbitrarily_large_input_without_truncation() {
    // 1 MiB — far beyond any inline carrier width. The `Borrowed`
    // carrier holds the full slice; nothing is truncated to
    // `INLINE_BYTES`.
    let big: Vec<u8> = (0..1024 * 1024).map(|i| (i % 251) as u8).collect();
    let input = BorrowedInput { data: &big };

    let carrier = input.as_binding_value::<CARRIER>();
    assert!(
        matches!(carrier, TermValue::Borrowed(_)),
        "a large input must produce the Borrowed carrier, not Inline",
    );
    // `bytes()` exposes the full borrowed slice — the whole 1 MiB,
    // independent of the CARRIER inline width.
    assert_eq!(
        carrier.bytes().len(),
        big.len(),
        "the Borrowed carrier must expose the full input with no INLINE_BYTES truncation",
    );
    assert_eq!(carrier.bytes(), big.as_slice());
}
