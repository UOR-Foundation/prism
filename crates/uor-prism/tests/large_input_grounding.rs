//! Large-input grounding: content-addressing inputs whose byte length
//! exceeds the ADR-060 inline carrier width (`INLINE_BYTES`).
//!
//! # The constraint this test resolves
//!
//! The convenience path `prism_model!` → `forward()` →
//! `pipeline::run_route` serializes the model input through
//! `IntoBindingValue::into_binding_bytes` into a stack `[u8; INLINE_BYTES]`
//! buffer and **rejects** any input whose `MAX_BYTES` exceeds
//! `INLINE_BYTES = carrier_inline_bytes::<B>()` (≈71–97 bytes for
//! SHA-256-class bounds). That convenience cap is *not* a fundamental
//! limit of the architecture: per wiki ADR-060 the byte width of any
//! single value carrier is an application concern, and large structured
//! payloads (model-weight container formats, multi-GB tensor-data
//! sections, canonical-JSON documents) are content-addressed by their
//! hash, not by materializing them into a fixed stack buffer.
//!
//! # The uncapped path
//!
//! Foundation's public surface admits arbitrarily large inputs without
//! the `run_route` cap:
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
//! This test grounds an input two orders of magnitude larger than
//! `INLINE_BYTES`, asserts the result is `Grounded` (no rejection), and
//! verifies the QS-05 replay round-trip — proving the architecture
//! content-addresses large inputs end-to-end through prism's re-exported
//! foundation surface.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

mod common;

use prism::crypto::Sha256Hasher;
use prism::operation::Term;
use prism::pipeline::run;
use prism::replay::{certify_from_trace, Trace};
use prism::seal::Validated;
use prism::std_types::ConstrainedTypeInput;
use prism::vocabulary::{
    Binding, CompileUnitBuilder, Hasher, HostBounds, VerificationDomain, WittLevel,
};
use uor_foundation::pipeline::ConstrainedTypeShape;

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
        run::<ConstrainedTypeInput, _, H, CARRIER>(unit).expect("pipeline admits large input");

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
    // 100 KiB — ~1000× the SHA-256-class `INLINE_BYTES` ceiling (which
    // the module-level `const` assertion guards). The convenience
    // `run_route` path would reject this; the streaming-hash +
    // explicit-binding path grounds it.
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
