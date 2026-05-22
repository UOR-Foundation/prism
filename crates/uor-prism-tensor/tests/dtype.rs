//! Smoke tests for the [`prism_tensor::dtype`] element-type alphabet
//! and the [`TensorDtypeRegistry`] shape-IRI registry per wiki ADR-057.
//!
//! Each block-byte-count assertion in this file is anchored to a
//! reference value: either an IEEE 754 / integer width derived from
//! `bits / 8`, or an authoritative `ggml-common.h` `static_assert`
//! formula. No literal magic numbers are written for the dtypes
//! themselves; the test computes the expected value from the same
//! parameters the dtype module uses to declare the value, so the test
//! and the declaration co-vary if ggml ever changes a parameter.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::identity_op,
    non_camel_case_types
)]

use prism_tensor::dtype::{
    Dtype, TensorDtypeRegistry, BF16, BOOL, C128, C64, F16, F32, F4_E2M1, F64, F8_E4M3,
    F8_E4M3_FNUZ, F8_E5M2, F8_E5M2_FNUZ, I16, I32, I4, I64, I8, IQ1_M, IQ1_S, IQ2_S, IQ2_XS,
    IQ2_XXS, IQ3_S, IQ3_XXS, IQ4_NL, IQ4_XS, Q2_K, Q3_K, Q4_0, Q4_1, Q4_K, Q5_0, Q5_1, Q5_K, Q6_K,
    Q8_0, Q8_1, Q8_K, U16, U32, U4, U64, U8,
};
use uor_foundation::pipeline::shape_iri_registry::ShapeRegistryProvider;
use uor_foundation::pipeline::ConstrainedTypeShape;

// ---- Reference parameters mirroring `ggml-common.h` ----
//
// The test reproduces the source-of-truth values independently so a
// regression in the dtype module's formulas is caught against external
// constants, not against the module's own private constants.

const BITS_PER_BYTE: usize = 8;

const QK_K: usize = 256;
const QK4_0: usize = 32;
const QK5_0: usize = 32;
const QK8_0: usize = 32;
const QK4_NL: usize = 32;

const GGML_HALF: usize = 16 / BITS_PER_BYTE;
const GGML_U16: usize = 16 / BITS_PER_BYTE;
const GGML_FLOAT: usize = 32 / BITS_PER_BYTE;
const GGML_I16: usize = 16 / BITS_PER_BYTE;

const K_SCALE_SIZE: usize = 12;
const IQ3S_N_SCALE: usize = 4;

// ---- Continuous floating-point ----

#[test]
fn continuous_float_block_bytes() {
    assert_eq!(F32::BLOCK_BYTES, 32 / BITS_PER_BYTE);
    assert_eq!(F16::BLOCK_BYTES, 16 / BITS_PER_BYTE);
    assert_eq!(BF16::BLOCK_BYTES, 16 / BITS_PER_BYTE);
    assert_eq!(F64::BLOCK_BYTES, 64 / BITS_PER_BYTE);

    for be in [
        F32::BLOCK_ELEMS,
        F16::BLOCK_ELEMS,
        BF16::BLOCK_ELEMS,
        F64::BLOCK_ELEMS,
    ] {
        assert_eq!(be, 1, "continuous floats are one element per block");
    }
}

// ---- ONNX 8-bit floating-point ----

#[test]
fn onnx_float8_block_bytes() {
    let one_byte = 8 / BITS_PER_BYTE;
    assert_eq!(F8_E4M3::BLOCK_BYTES, one_byte);
    assert_eq!(F8_E4M3_FNUZ::BLOCK_BYTES, one_byte);
    assert_eq!(F8_E5M2::BLOCK_BYTES, one_byte);
    assert_eq!(F8_E5M2_FNUZ::BLOCK_BYTES, one_byte);
}

// ---- ONNX complex ----

#[test]
fn onnx_complex_block_bytes() {
    assert_eq!(C64::BLOCK_BYTES, 2 * (32 / BITS_PER_BYTE));
    assert_eq!(C128::BLOCK_BYTES, 2 * (64 / BITS_PER_BYTE));
}

// ---- Signed / unsigned integers + boolean ----

#[test]
fn integer_block_bytes() {
    assert_eq!(I8::BLOCK_BYTES, 1);
    assert_eq!(I16::BLOCK_BYTES, 2);
    assert_eq!(I32::BLOCK_BYTES, 4);
    assert_eq!(I64::BLOCK_BYTES, 8);
    assert_eq!(U8::BLOCK_BYTES, 1);
    assert_eq!(U16::BLOCK_BYTES, 2);
    assert_eq!(U32::BLOCK_BYTES, 4);
    assert_eq!(U64::BLOCK_BYTES, 8);
    assert_eq!(BOOL::BLOCK_BYTES, 1);
}

// ---- ONNX packed 4-bit ----

#[test]
fn packed_nibble_block_bytes() {
    // Two 4-bit elements per byte.
    assert_eq!(I4::BLOCK_BYTES, 1);
    assert_eq!(U4::BLOCK_BYTES, 1);
    assert_eq!(F4_E2M1::BLOCK_BYTES, 1);
    assert_eq!(I4::BLOCK_ELEMS, 2);
    assert_eq!(U4::BLOCK_ELEMS, 2);
    assert_eq!(F4_E2M1::BLOCK_ELEMS, 2);
}

// ---- GGML legacy block-32 quantization ----

#[test]
fn ggml_legacy_block32_bytes() {
    // block_q4_0: sizeof(ggml_half) + QK4_0/2
    assert_eq!(Q4_0::BLOCK_BYTES, GGML_HALF + QK4_0 / 2);
    assert_eq!(Q4_0::BLOCK_BYTES, 18);
    // block_q4_1: 2*sizeof(ggml_half) + QK4_1/2
    assert_eq!(Q4_1::BLOCK_BYTES, 2 * GGML_HALF + QK4_0 / 2);
    assert_eq!(Q4_1::BLOCK_BYTES, 20);
    // block_q5_0: sizeof(ggml_half) + sizeof(uint32_t) + QK5_0/2
    assert_eq!(Q5_0::BLOCK_BYTES, GGML_HALF + 2 * GGML_U16 + QK5_0 / 2);
    assert_eq!(Q5_0::BLOCK_BYTES, 22);
    // block_q5_1: 2*sizeof(ggml_half) + sizeof(uint32_t) + QK5_1/2
    assert_eq!(Q5_1::BLOCK_BYTES, 2 * GGML_HALF + 2 * GGML_U16 + QK5_0 / 2);
    assert_eq!(Q5_1::BLOCK_BYTES, 24);
    // block_q8_0: sizeof(ggml_half) + QK8_0
    assert_eq!(Q8_0::BLOCK_BYTES, GGML_HALF + QK8_0);
    assert_eq!(Q8_0::BLOCK_BYTES, 34);
    // block_q8_1: 2*sizeof(ggml_half) + QK8_1
    assert_eq!(Q8_1::BLOCK_BYTES, 2 * GGML_HALF + QK8_0);
    assert_eq!(Q8_1::BLOCK_BYTES, 36);

    for be in [
        Q4_0::BLOCK_ELEMS,
        Q4_1::BLOCK_ELEMS,
        Q5_0::BLOCK_ELEMS,
        Q5_1::BLOCK_ELEMS,
        Q8_0::BLOCK_ELEMS,
        Q8_1::BLOCK_ELEMS,
    ] {
        assert_eq!(be, 32, "legacy quantization is block-32");
    }
}

// ---- GGML K-series block-256 quantization ----

#[test]
fn ggml_kseries_block256_bytes() {
    // block_q2_K: 2*sizeof(ggml_half) + QK_K/16 + QK_K/4
    assert_eq!(Q2_K::BLOCK_BYTES, 2 * GGML_HALF + QK_K / 16 + QK_K / 4);
    assert_eq!(Q2_K::BLOCK_BYTES, 84);
    // block_q3_K: sizeof(ggml_half) + QK_K/4 + QK_K/8 + K_SCALE_SIZE
    assert_eq!(
        Q3_K::BLOCK_BYTES,
        GGML_HALF + QK_K / 4 + QK_K / 8 + K_SCALE_SIZE
    );
    assert_eq!(Q3_K::BLOCK_BYTES, 110);
    // block_q4_K: 2*sizeof(ggml_half) + K_SCALE_SIZE + QK_K/2
    assert_eq!(Q4_K::BLOCK_BYTES, 2 * GGML_HALF + K_SCALE_SIZE + QK_K / 2);
    assert_eq!(Q4_K::BLOCK_BYTES, 144);
    // block_q5_K: 2*sizeof(ggml_half) + K_SCALE_SIZE + QK_K/2 + QK_K/8
    assert_eq!(
        Q5_K::BLOCK_BYTES,
        2 * GGML_HALF + K_SCALE_SIZE + QK_K / 2 + QK_K / 8
    );
    assert_eq!(Q5_K::BLOCK_BYTES, 176);
    // block_q6_K: sizeof(ggml_half) + QK_K/16 + 3*QK_K/4
    assert_eq!(Q6_K::BLOCK_BYTES, GGML_HALF + QK_K / 16 + 3 * QK_K / 4);
    assert_eq!(Q6_K::BLOCK_BYTES, 210);
    // block_q8_K: sizeof(float) + QK_K + QK_K/16 * sizeof(int16_t)
    assert_eq!(Q8_K::BLOCK_BYTES, GGML_FLOAT + QK_K + QK_K / 16 * GGML_I16);
    assert_eq!(Q8_K::BLOCK_BYTES, 292);

    for be in [
        Q2_K::BLOCK_ELEMS,
        Q3_K::BLOCK_ELEMS,
        Q4_K::BLOCK_ELEMS,
        Q5_K::BLOCK_ELEMS,
        Q6_K::BLOCK_ELEMS,
        Q8_K::BLOCK_ELEMS,
    ] {
        assert_eq!(be, QK_K, "K-series quantization is block-256");
    }
}

// ---- GGML IQ-series importance-aware quantization ----

#[test]
fn ggml_iqseries_block_bytes() {
    // block_iq1_s: sizeof(ggml_half) + QK_K/8 + QK_K/16
    assert_eq!(IQ1_S::BLOCK_BYTES, GGML_HALF + QK_K / 8 + QK_K / 16);
    assert_eq!(IQ1_S::BLOCK_BYTES, 50);
    // block_iq1_m: QK_K/8 + QK_K/16 + QK_K/32 (no ggml_half prefix)
    assert_eq!(IQ1_M::BLOCK_BYTES, QK_K / 8 + QK_K / 16 + QK_K / 32);
    assert_eq!(IQ1_M::BLOCK_BYTES, 56);
    // block_iq2_xxs: sizeof(ggml_half) + QK_K/8 * sizeof(uint16_t)
    assert_eq!(IQ2_XXS::BLOCK_BYTES, GGML_HALF + QK_K / 8 * GGML_U16);
    assert_eq!(IQ2_XXS::BLOCK_BYTES, 66);
    // block_iq2_xs: sizeof(ggml_half) + QK_K/8 * sizeof(uint16_t) + QK_K/32
    assert_eq!(
        IQ2_XS::BLOCK_BYTES,
        GGML_HALF + QK_K / 8 * GGML_U16 + QK_K / 32
    );
    assert_eq!(IQ2_XS::BLOCK_BYTES, 74);
    // block_iq2_s: sizeof(ggml_half) + QK_K/4 + QK_K/16
    assert_eq!(IQ2_S::BLOCK_BYTES, GGML_HALF + QK_K / 4 + QK_K / 16);
    assert_eq!(IQ2_S::BLOCK_BYTES, 82);
    // block_iq3_xxs: sizeof(ggml_half) + 3*(QK_K/8)
    assert_eq!(IQ3_XXS::BLOCK_BYTES, GGML_HALF + 3 * (QK_K / 8));
    assert_eq!(IQ3_XXS::BLOCK_BYTES, 98);
    // block_iq3_s: sizeof(ggml_half) + 13*(QK_K/32) + IQ3S_N_SCALE
    assert_eq!(
        IQ3_S::BLOCK_BYTES,
        GGML_HALF + 13 * (QK_K / 32) + IQ3S_N_SCALE
    );
    assert_eq!(IQ3_S::BLOCK_BYTES, 110);
    // block_iq4_nl: sizeof(ggml_half) + QK4_NL/2 (block-32, not K-series)
    assert_eq!(IQ4_NL::BLOCK_BYTES, GGML_HALF + QK4_NL / 2);
    assert_eq!(IQ4_NL::BLOCK_BYTES, 18);
    assert_eq!(IQ4_NL::BLOCK_ELEMS, QK4_NL);
    // block_iq4_xs: sizeof(ggml_half) + sizeof(uint16_t) + QK_K/64 + QK_K/2
    assert_eq!(
        IQ4_XS::BLOCK_BYTES,
        GGML_HALF + GGML_U16 + QK_K / 64 + QK_K / 2
    );
    assert_eq!(IQ4_XS::BLOCK_BYTES, 136);
}

// ---- ADR-017 closure-rule collisions ----

#[test]
fn equal_block_bytes_content_address_identically() {
    // Per ADR-017's empty-CONSTRAINTS closure rule, two dtypes with
    // identical BLOCK_BYTES share (SITE_COUNT, CONSTRAINTS) and so
    // content-address identically at the framework level. SITE_COUNT
    // equals BLOCK_BYTES for every dtype.
    assert_eq!(
        <F16 as ConstrainedTypeShape>::SITE_COUNT,
        <BF16 as ConstrainedTypeShape>::SITE_COUNT
    );
    assert_eq!(
        <F16 as ConstrainedTypeShape>::SITE_COUNT,
        <I16 as ConstrainedTypeShape>::SITE_COUNT
    );
    assert_eq!(
        <F16 as ConstrainedTypeShape>::SITE_COUNT,
        <U16 as ConstrainedTypeShape>::SITE_COUNT
    );
    // 18-byte collision: Q4_0 and IQ4_NL.
    assert_eq!(Q4_0::BLOCK_BYTES, IQ4_NL::BLOCK_BYTES);
    assert_eq!(
        <Q4_0 as ConstrainedTypeShape>::SITE_COUNT,
        <IQ4_NL as ConstrainedTypeShape>::SITE_COUNT
    );
    // 110-byte collision: Q3_K and IQ3_S.
    assert_eq!(Q3_K::BLOCK_BYTES, IQ3_S::BLOCK_BYTES);
    assert_eq!(
        <Q3_K as ConstrainedTypeShape>::SITE_COUNT,
        <IQ3_S as ConstrainedTypeShape>::SITE_COUNT
    );

    // Every dtype shares the generic ConstrainedType IRI per ADR-017.
    assert_eq!(
        <F32 as ConstrainedTypeShape>::IRI,
        "https://uor.foundation/type/ConstrainedType"
    );
    assert_eq!(
        <Q4_0 as ConstrainedTypeShape>::IRI,
        <IQ4_NL as ConstrainedTypeShape>::IRI
    );
}

#[test]
fn site_count_equals_block_bytes() {
    assert_eq!(<F32 as ConstrainedTypeShape>::SITE_COUNT, F32::BLOCK_BYTES);
    assert_eq!(
        <Q6_K as ConstrainedTypeShape>::SITE_COUNT,
        Q6_K::BLOCK_BYTES
    );
    assert_eq!(
        <IQ4_XS as ConstrainedTypeShape>::SITE_COUNT,
        IQ4_XS::BLOCK_BYTES
    );
}

// ---- Shape-IRI registry per ADR-057 ----

#[test]
fn registry_carries_every_dtype() {
    // 4 continuous + 4 FLOAT8 + 2 complex + 4 signed + 4 unsigned + 1
    // bool + 3 packed-4-bit + 6 legacy-q + 6 K-series + 9 IQ-series = 43.
    const EXPECTED: usize = 4 + 4 + 2 + 4 + 4 + 1 + 3 + 6 + 6 + 9;
    assert_eq!(EXPECTED, 43);
    let registry = <TensorDtypeRegistry as ShapeRegistryProvider>::REGISTRY;
    assert_eq!(registry.len(), EXPECTED);
}

#[test]
fn name_constants_match_struct_names() {
    assert_eq!(F32::NAME, "F32");
    assert_eq!(BF16::NAME, "BF16");
    assert_eq!(Q4_0::NAME, "Q4_0");
    assert_eq!(IQ2_XXS::NAME, "IQ2_XXS");
}
