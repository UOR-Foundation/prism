//! `TensorAxis` declaration and 4×4 i8 matmul reference impl.

#![allow(missing_docs)]

use uor_foundation::enforcement::ShapeViolation;
use uor_foundation::pipeline::AxisExtension;
use uor_foundation_sdk::axis;

axis! {
    /// Wiki ADR-031 tensor-compute axis.
    ///
    /// Reference kernel `matmul` over fixed 4×4 `i8` matrices —
    /// canonical bit-deterministic integer-precision tensor primitive.
    /// Variable-rank tensor compute composes through verbs over the
    /// `partition_product!` shape mechanism (ADR-033/044).
    pub trait TensorAxis: AxisExtension {
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/TensorAxis";
        const MAX_OUTPUT_BYTES: usize = 32;
        /// Multiply two row-major 4×4 `i8` matrices into a 4×4 `i16`
        /// product (saturating). Input is `A || B` (32 bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output byte-length mismatch.
        fn matmul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

const MAT_BYTES: usize = 16;
const INPUT_BYTES: usize = 2 * MAT_BYTES;
const OUTPUT_BYTES: usize = 32;
const DIM: usize = 4;

fn arity_violation(constraint: &'static str) -> ShapeViolation {
    ShapeViolation {
        shape_iri: "https://uor.foundation/axis/TensorAxisShape",
        constraint_iri: constraint,
        property_iri: "https://uor.foundation/axis/inputBytes",
        expected_range: "https://uor.foundation/axis/TensorInputArity",
        min_count: 0,
        max_count: 0,
        kind: uor_foundation::ViolationKind::ValueCheck,
    }
}

fn idx(row: usize, col: usize) -> usize {
    row * DIM + col
}

/// Fixed-shape 4×4 `i8` matrix-matrix multiply emitting a 4×4 `i16`
/// product. Determinism: per ADR-030's per-axis substitution-determinism
/// note, the integer-arithmetic CPU impl preserves bit-identity across
/// targets.
#[derive(Debug, Clone, Copy, Default)]
pub struct CpuI8Tensor4x4Matmul;

impl TensorAxis for CpuI8Tensor4x4Matmul {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/TensorAxis/CpuI8Matmul4x4";
    const MAX_OUTPUT_BYTES: usize = OUTPUT_BYTES;

    fn matmul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if input.len() != INPUT_BYTES {
            return Err(arity_violation(
                "https://uor.foundation/axis/TensorAxisShape/inputByteLength",
            ));
        }
        if out.len() < OUTPUT_BYTES {
            return Err(arity_violation(
                "https://uor.foundation/axis/TensorAxisShape/outputByteLength",
            ));
        }
        let (a_bytes, b_bytes) = input.split_at(MAT_BYTES);
        for row in 0..DIM {
            for col in 0..DIM {
                let mut acc: i32 = 0;
                for k in 0..DIM {
                    #[allow(clippy::cast_possible_wrap)]
                    let a = i32::from(a_bytes[idx(row, k)] as i8);
                    #[allow(clippy::cast_possible_wrap)]
                    let b = i32::from(b_bytes[idx(k, col)] as i8);
                    acc += a * b;
                }
                let saturated: i16 = if acc > i32::from(i16::MAX) {
                    i16::MAX
                } else if acc < i32::from(i16::MIN) {
                    i16::MIN
                } else {
                    #[allow(clippy::cast_possible_truncation)]
                    {
                        acc as i16
                    }
                };
                let cell = idx(row, col);
                out[2 * cell..2 * cell + 2].copy_from_slice(&saturated.to_be_bytes());
            }
        }
        Ok(OUTPUT_BYTES)
    }
}

axis_extension_impl_for_tensor_axis!(CpuI8Tensor4x4Matmul);
