//! Conformance vectors for prism-fhe's `FheAxis` reference impl per ADR-031.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::cast_possible_truncation,
    clippy::needless_range_loop
)]

use prism_fhe::{
    CiphertextShape, FheAxis, OneTimePadFhe, OneTimePadFhe128, OneTimePadFhe16, OneTimePadFhe64,
    OneTimePadFheAxis,
};
use uor_foundation::pipeline::ConstrainedTypeShape;

#[test]
fn one_time_pad_adds_zero_yields_left() {
    // a XOR 0 = a (zero ciphertext is the additive identity).
    let mut input = [0u8; 64];
    for (i, slot) in input.iter_mut().enumerate().take(32) {
        *slot = u8::try_from(i).expect("loop index fits u8");
    }
    let mut out = [0u8; 32];
    OneTimePadFheAxis::add_ciphertexts(&input, &mut out).expect("add_ciphertexts ok");
    for (i, byte) in out.iter().enumerate() {
        assert_eq!(*byte, u8::try_from(i).expect("loop index fits u8"));
    }
}

#[test]
fn one_time_pad_self_xor_yields_zero() {
    // a XOR a = 0 (every ciphertext is its own additive inverse in GF(2)).
    let mut input = [0u8; 64];
    for i in 0..32 {
        input[i] = 0xaa;
        input[32 + i] = 0xaa;
    }
    let mut out = [0u8; 32];
    OneTimePadFheAxis::add_ciphertexts(&input, &mut out).expect("add_ciphertexts ok");
    for byte in out {
        assert_eq!(byte, 0);
    }
}

#[test]
fn one_time_pad_rejects_wrong_input_arity() {
    let input = [0u8; 32]; // expected 64
    let mut out = [0u8; 32];
    let err = OneTimePadFheAxis::add_ciphertexts(&input, &mut out).unwrap_err();
    assert_eq!(
        err.constraint_iri,
        "https://uor.foundation/axis/FheAxis/inputBlockPair"
    );
}

// ---- Parametricity: alternate block widths ----

#[test]
fn one_time_pad_16byte_blocks() {
    let mut input = [0u8; 32];
    for i in 0..16 {
        input[i] = 0xaa;
        input[16 + i] = 0x55;
    }
    let mut out = [0u8; 16];
    OneTimePadFhe16::add_ciphertexts(&input, &mut out).expect("xor ok");
    for &b in &out {
        assert_eq!(b, 0xff);
    }
}

#[test]
fn one_time_pad_64byte_blocks() {
    let mut input = [0u8; 128];
    for i in 0..64 {
        input[i] = 0x12;
        input[64 + i] = 0x34;
    }
    let mut out = [0u8; 64];
    OneTimePadFhe64::add_ciphertexts(&input, &mut out).expect("xor ok");
    for &b in &out {
        assert_eq!(b, 0x12 ^ 0x34);
    }
}

#[test]
fn one_time_pad_128byte_blocks() {
    type Fhe128 = OneTimePadFhe<128>;
    let mut input = [0u8; 256];
    for i in 0..128 {
        input[i] = (i as u8).wrapping_mul(7);
        input[128 + i] = (i as u8).wrapping_mul(11);
    }
    let mut out = [0u8; 128];
    Fhe128::add_ciphertexts(&input, &mut out).expect("xor ok");
    for i in 0..128 {
        let expected = ((i as u8).wrapping_mul(7)) ^ ((i as u8).wrapping_mul(11));
        assert_eq!(out[i], expected);
    }
    // Suppress unused-import lint on OneTimePadFhe128.
    let _: OneTimePadFhe128 = OneTimePadFhe128::default();
}

// ---- Parametric shape introspection ----

#[test]
fn ciphertext_shape_site_counts() {
    assert_eq!(
        <CiphertextShape<16> as ConstrainedTypeShape>::SITE_COUNT,
        16
    );
    assert_eq!(
        <CiphertextShape<32> as ConstrainedTypeShape>::SITE_COUNT,
        32
    );
    assert_eq!(
        <CiphertextShape<64> as ConstrainedTypeShape>::SITE_COUNT,
        64
    );
}

#[test]
fn ciphertext_shape_iri_closure_rule() {
    assert_eq!(
        <CiphertextShape<32> as ConstrainedTypeShape>::IRI,
        "https://uor.foundation/type/ConstrainedType"
    );
}
