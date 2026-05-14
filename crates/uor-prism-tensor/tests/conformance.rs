//! Conformance vectors for prism-tensor's `TensorAxis` and `ActivationAxis`
//! impls per ADR-031.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::needless_range_loop
)]

use prism_tensor::{ActivationAxis, CpuI8Tensor4x4Matmul, CpuI8VectorActivation16, TensorAxis};

// ---- TensorAxis: 4x4 matmul ----

#[test]
fn matmul_identity_times_a_equals_a() {
    // Identity matrix times any A should equal A (in i8).
    // i8 identity: 1 on diag, 0 elsewhere.
    let identity: [u8; 16] = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];
    let a: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut input = [0u8; 32];
    input[..16].copy_from_slice(&identity);
    input[16..].copy_from_slice(&a);
    let mut out = [0u8; 32];
    CpuI8Tensor4x4Matmul::matmul(&input, &mut out).expect("matmul ok");
    // Each output cell is i16 BE. Cell (r,c) of I·A = A[r][c].
    for cell in 0..16 {
        let expected = i16::from(a[cell] as i8);
        let actual = i16::from_be_bytes([out[2 * cell], out[2 * cell + 1]]);
        assert_eq!(
            actual, expected,
            "cell {cell}: expected {expected}, got {actual}"
        );
    }
}

#[test]
fn matmul_zero_times_a_equals_zero() {
    let zero = [0u8; 16];
    let a: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut input = [0u8; 32];
    input[..16].copy_from_slice(&zero);
    input[16..].copy_from_slice(&a);
    let mut out = [0u8; 32];
    CpuI8Tensor4x4Matmul::matmul(&input, &mut out).expect("matmul ok");
    for byte in out {
        assert_eq!(byte, 0);
    }
}

// ---- ActivationAxis: ReLU + Q1.7 sigmoid ----

#[test]
fn relu_clamps_negatives() {
    let mut input = [0u8; 16];
    for i in 0..8 {
        input[i] = (-(i as i8 + 1)) as u8; // negatives
        input[8 + i] = (i as i8 + 1) as u8; // positives
    }
    let mut out = [0u8; 16];
    CpuI8VectorActivation16::relu(&input, &mut out).expect("relu ok");
    for i in 0..8 {
        assert_eq!(out[i], 0, "negative input at {i} should clamp");
        assert_eq!(
            out[8 + i],
            input[8 + i],
            "positive input at {} should pass",
            8 + i
        );
    }
}

#[test]
fn sigmoid_q_saturates_at_extremes() {
    // x = -128 → y = 0; x = +127 → y = 127.
    let mut input = [0u8; 16];
    input[0] = (-128_i8) as u8;
    input[1] = 127_u8;
    // mid-range x = 0 → y = 64 (the Q1.7 mid-point per the piecewise impl).
    input[2] = 0;
    let mut out = [0u8; 16];
    CpuI8VectorActivation16::sigmoid_q(&input, &mut out).expect("sigmoid ok");
    assert_eq!(out[0], 0);
    assert_eq!(out[1], 127);
    assert_eq!(out[2], 64);
}

// ---- Error paths ----

#[test]
fn matmul_rejects_wrong_input_length() {
    let input = [0u8; 16]; // half the expected 32
    let mut out = [0u8; 32];
    let err = CpuI8Tensor4x4Matmul::matmul(&input, &mut out).unwrap_err();
    assert_eq!(
        err.constraint_iri,
        "https://uor.foundation/axis/TensorAxisShape/inputByteLength"
    );
}
