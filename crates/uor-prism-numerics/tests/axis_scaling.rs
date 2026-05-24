//! Scaling V&V for the parametric numerics axis kernels — the
//! falsification suite for the claim that `BigIntModularNumeric<BYTES>`
//! and `Gf2NumericAxisN<BYTES>` admit **any** operand width with no
//! ceiling (AGENTS.md § 11.10 category 3).
//!
//! These kernels previously carried fixed `MAX_BIG_INT_BYTES = 64` /
//! `MAX_GF2_BYTES = 128` caps backing on-stack scratch. The kernels now
//! carry no fixed-width scratch — add/sub stream the carry into `out`,
//! `mul` folds the modular product in a single running `u64`, and the
//! GF(2) ops are bytewise — so the width scales arbitrarily. This suite
//! exercises widths an order of magnitude past the retired caps and pins
//! exact arithmetic, so a reintroduced ceiling fails the build's tests.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::needless_range_loop)]

use prism_numerics::{BigIntAxis, BigIntModularNumeric, Gf2NumericAxisN, RingAxis};

/// Build a `width`-byte big-endian operand pair `a || b` (`2*width`
/// bytes) where `a`/`b` are given by their least-significant tail bytes,
/// zero-extended toward the most-significant end.
fn build_pair(width: usize, a_tail: &[u8], b_tail: &[u8]) -> Vec<u8> {
    let mut v = vec![0u8; 2 * width];
    v[width - a_tail.len()..width].copy_from_slice(a_tail);
    v[2 * width - b_tail.len()..2 * width].copy_from_slice(b_tail);
    v
}

// ---- const-generic dispatch over a runtime width ----
//
// The kernels are generic over `const BYTES`; a runtime `width` is routed
// to the fixed set of monomorphizations the tests exercise. Each arm is a
// genuine, distinct instantiation — the point is that none carries a
// width ceiling.
macro_rules! dispatch {
    ($name:ident, $axis:ident, $method:ident) => {
        fn $name(width: usize, input: &[u8], out: &mut [u8]) {
            match width {
                100 => $axis::<100>::$method(input, out).unwrap(),
                128 => $axis::<128>::$method(input, out).unwrap(),
                256 => $axis::<256>::$method(input, out).unwrap(),
                300 => $axis::<300>::$method(input, out).unwrap(),
                512 => $axis::<512>::$method(input, out).unwrap(),
                777 => $axis::<777>::$method(input, out).unwrap(),
                1024 => $axis::<1024>::$method(input, out).unwrap(),
                2048 => $axis::<2048>::$method(input, out).unwrap(),
                4096 => $axis::<4096>::$method(input, out).unwrap(),
                _ => unreachable!("unmapped test width {width}"),
            };
        }
    };
}

dispatch!(dispatch_mul, BigIntModularNumeric, mul);
dispatch!(dispatch_add, BigIntModularNumeric, add);
dispatch!(dispatch_gf2_add, Gf2NumericAxisN, add);
dispatch!(dispatch_gf2_mul, Gf2NumericAxisN, mul);

// ---- BigInt: exact arithmetic far past the retired 64-byte cap ----

#[test]
fn bigint_mul_is_exact_far_past_the_retired_cap() {
    // 0x0102 * 0x0304 = 258 * 772 = 199_176 = 0x03_0A_08. Exercises
    // multi-byte carry. Checked at 100 bytes (> retired 64) up to 1 KiB.
    for &width in &[100usize, 256, 512, 1024] {
        let input = build_pair(width, &[0x01, 0x02], &[0x03, 0x04]);
        let mut out = vec![0u8; width];
        dispatch_mul(width, &input, &mut out);
        assert_eq!(out[width - 1], 0x08, "width {width}: byte 0");
        assert_eq!(out[width - 2], 0x0A, "width {width}: byte 1");
        assert_eq!(out[width - 3], 0x03, "width {width}: byte 2");
        for i in 0..width - 3 {
            assert_eq!(out[i], 0, "width {width}: high byte {i} must be zero");
        }
    }
}

#[test]
fn bigint_mul_carries_across_bytes_at_scale() {
    // 0xFF * 0xFF = 65_025 = 0xFE01 — a full single-byte carry. At any
    // width the low two bytes are 0xFE 0x01, the rest zero.
    for &width in &[100usize, 300] {
        let input = build_pair(width, &[0xFF], &[0xFF]);
        let mut out = vec![0u8; width];
        dispatch_mul(width, &input, &mut out);
        assert_eq!(out[width - 1], 0x01);
        assert_eq!(out[width - 2], 0xFE);
        for i in 0..width - 2 {
            assert_eq!(out[i], 0);
        }
    }
}

#[test]
fn bigint_add_scales_arbitrarily() {
    // Setting the most-significant byte of each operand to 0x01 and
    // adding yields 0x02 there with no carry out (everything else zero).
    for &width in &[128usize, 777, 2048] {
        // Most-significant byte of each operand set to 0x01.
        let mut input = vec![0u8; 2 * width];
        input[0] = 0x01;
        input[width] = 0x01;
        let mut out = vec![0u8; width];
        dispatch_add(width, &input, &mut out);
        assert_eq!(out[0], 0x02, "width {width}");
        for i in 1..width {
            assert_eq!(out[i], 0);
        }
    }
}

// ---- GF(2): bytewise ops far past the retired 128-byte cap ----

#[test]
fn gf2_scales_arbitrarily() {
    for &width in &[256usize, 1024, 4096] {
        let mut input = vec![0u8; 2 * width];
        input[..width].fill(0xFF);
        input[width..].fill(0x0F);

        let mut xor_out = vec![0u8; width];
        let mut and_out = vec![0u8; width];
        dispatch_gf2_add(width, &input, &mut xor_out);
        dispatch_gf2_mul(width, &input, &mut and_out);
        for i in 0..width {
            assert_eq!(xor_out[i], 0xF0, "XOR width {width} byte {i}");
            assert_eq!(and_out[i], 0x0F, "AND width {width} byte {i}");
        }
    }
}
