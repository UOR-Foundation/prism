//! Conformance vectors for prism-fhe's `FheAxis` reference impl per ADR-031.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use prism_fhe::{FheAxis, OneTimePadFheAxis};

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
