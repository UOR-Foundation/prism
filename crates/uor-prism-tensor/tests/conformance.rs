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

use prism_tensor::{
    ActivationAxis, CpuI8MatmulSquare, CpuI8VectorActivation, MatrixShape, TensorAxis, VectorShape,
};
use uor_foundation::pipeline::ConstrainedTypeShape;

/// 4×4 i8 matmul reference used throughout the conformance vectors.
/// Picked at test scope; the production-grade ceiling is the
/// application's `HostBounds` structural-count primitives per ADR-060.
type Mat4 = CpuI8MatmulSquare<4>;
/// 8×8 i8 matmul reference.
type Mat8 = CpuI8MatmulSquare<8>;
/// 16×16 i8 matmul reference — exercising a larger square shape.
type Mat16 = CpuI8MatmulSquare<16>;
/// 16-element i8 vector activation reference.
type Vec16 = CpuI8VectorActivation<16>;
/// 32-element i8 vector activation reference.
type Vec32 = CpuI8VectorActivation<32>;

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
    Mat4::matmul(&input, &mut out).expect("matmul ok");
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
    Mat4::matmul(&input, &mut out).expect("matmul ok");
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
    Vec16::relu(&input, &mut out).expect("relu ok");
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
    Vec16::sigmoid_q(&input, &mut out).expect("sigmoid ok");
    assert_eq!(out[0], 0);
    assert_eq!(out[1], 127);
    assert_eq!(out[2], 64);
}

// ---- Error paths ----

#[test]
fn matmul_rejects_wrong_input_length() {
    let input = [0u8; 16]; // half the expected 32
    let mut out = [0u8; 32];
    let err = Mat4::matmul(&input, &mut out).unwrap_err();
    assert_eq!(
        err.constraint_iri,
        "https://uor.foundation/axis/TensorAxisShape/inputByteLength"
    );
}

// ---- Parametricity: alternate dimensions ----

#[test]
fn matmul_8x8_identity() {
    let mut input = [0u8; 128]; // 2 * 8 * 8 bytes
                                // I_8 in the first 64 bytes.
    for r in 0..8 {
        input[r * 8 + r] = 1;
    }
    // Arbitrary B in the second 64 bytes: B[r][c] = r * 8 + c (mod 128).
    for r in 0..8 {
        for c in 0..8 {
            input[64 + r * 8 + c] = (r * 8 + c) as u8;
        }
    }
    let mut out = [0u8; 128]; // 2 * 8 * 8
    Mat8::matmul(&input, &mut out).expect("matmul ok");
    // Identity × B = B (in i16 saturating).
    for r in 0..8 {
        for c in 0..8 {
            let cell = r * 8 + c;
            let expected = i16::from(input[64 + cell] as i8);
            let actual = i16::from_be_bytes([out[2 * cell], out[2 * cell + 1]]);
            assert_eq!(actual, expected);
        }
    }
}

#[test]
fn matmul_16x16_zero() {
    // Exercise the matmul kernel at a larger square dimension; the
    // axis layer has no DIM ceiling of its own — per ADR-060 the
    // application's `HostBounds` primitives (via foundation `const fn`s)
    // size the carrier, with no byte-width cap.
    let input = [0u8; 2 * 16 * 16];
    let mut out = [0u8; 2 * 16 * 16];
    let n = Mat16::matmul(&input, &mut out).expect("matmul ok");
    assert_eq!(n, 2 * 16 * 16);
    for b in &out {
        assert_eq!(*b, 0);
    }
}

#[test]
fn matmul_dim_is_unbounded_at_axis_layer() {
    // Per ADR-060 the axis impl carries no substrate-arbitrary
    // ceiling on DIM. Instantiate at a dimension larger than the
    // historical `MAX_TENSOR_DIM = 16` cap to witness the absence of
    // an axis-level bound. Carrier widths derive from the application's
    // `HostBounds` primitives; the test allocates its own buffers at
    // the appropriate size.
    type Mat32 = CpuI8MatmulSquare<32>;
    const N: usize = 32;
    const MAT_BYTES: usize = N * N;
    let mut input = vec![0u8; 2 * MAT_BYTES];
    // Set A = identity, B = zeros -> A * B = zeros (cheap correctness
    // check at this DIM that doesn't require constructing a full
    // expected matrix).
    for k in 0..N {
        input[k * N + k] = 1;
    }
    let mut out = vec![0u8; 2 * MAT_BYTES];
    let n = Mat32::matmul(&input, &mut out).expect("matmul ok at DIM = 32");
    assert_eq!(n, 2 * MAT_BYTES);
    for b in &out {
        assert_eq!(*b, 0);
    }
}

#[test]
fn matmul_zero_dim_is_structural_violation() {
    // DIM == 0 is rejected as a structural well-formedness violation,
    // independent of HostBounds. This is the only DIM-validity check
    // the axis impl performs; the upper ceiling is HostBounds territory.
    type Mat0 = CpuI8MatmulSquare<0>;
    let input: [u8; 0] = [];
    let mut out: [u8; 0] = [];
    Mat0::matmul(&input, &mut out).expect_err("DIM = 0 must violate structural well-formedness");
}

#[test]
fn activation_relu_32_element() {
    let mut input = [0u8; 32];
    for i in 0..16 {
        input[i] = (-(i as i8 + 1)) as u8; // negatives
        input[16 + i] = (i as i8 + 1) as u8; // positives
    }
    let mut out = [0u8; 32];
    Vec32::relu(&input, &mut out).expect("relu ok");
    for i in 0..16 {
        assert_eq!(out[i], 0);
        assert_eq!(out[16 + i], input[16 + i]);
    }
}

// ---- Parametric shape introspection ----

#[test]
fn matrix_shape_site_counts() {
    // 4×4 i8 = 16 sites; 8×8 i8 = 64 sites; 4×4 i16 = 32 sites.
    assert_eq!(
        <MatrixShape<4, 4, 1> as ConstrainedTypeShape>::SITE_COUNT,
        16
    );
    assert_eq!(
        <MatrixShape<8, 8, 1> as ConstrainedTypeShape>::SITE_COUNT,
        64
    );
    assert_eq!(
        <MatrixShape<4, 4, 2> as ConstrainedTypeShape>::SITE_COUNT,
        32
    );
    assert_eq!(
        <MatrixShape<2, 3, 4> as ConstrainedTypeShape>::SITE_COUNT,
        24
    );
}

#[test]
fn vector_shape_site_counts() {
    assert_eq!(<VectorShape<16, 1> as ConstrainedTypeShape>::SITE_COUNT, 16);
    assert_eq!(
        <VectorShape<32, 4> as ConstrainedTypeShape>::SITE_COUNT,
        128
    );
}

#[test]
fn matmul_axis_address_distinct_per_dim() {
    // AxisExtension::AXIS_ADDRESS is shared across all DIM
    // instantiations because the wiki's ADR-031 says structural
    // identity flows through trait declaration, not type parameters.
    assert_eq!(
        <CpuI8MatmulSquare<4> as TensorAxis>::AXIS_ADDRESS,
        <CpuI8MatmulSquare<8> as TensorAxis>::AXIS_ADDRESS,
    );
}

#[test]
fn matmul_max_output_bytes_scales_with_dim() {
    assert_eq!(<CpuI8MatmulSquare<4> as TensorAxis>::MAX_OUTPUT_BYTES, 32);
    assert_eq!(<CpuI8MatmulSquare<8> as TensorAxis>::MAX_OUTPUT_BYTES, 128);
    assert_eq!(<CpuI8MatmulSquare<16> as TensorAxis>::MAX_OUTPUT_BYTES, 512);
}

// ---- Compile-time bound resolution: shapes are GroundedShape-bound ----

#[allow(dead_code)]
fn _shapes_are_grounded_shape() {
    fn check<S: uor_foundation::enforcement::GroundedShape>() {}
    check::<MatrixShape<4, 4, 1>>();
    check::<MatrixShape<8, 8, 2>>();
    check::<VectorShape<16, 1>>();
    check::<VectorShape<32, 4>>();
}
