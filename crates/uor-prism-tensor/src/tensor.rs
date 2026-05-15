//! `TensorAxis` declaration + parametric square-matmul impl + shape.
//!
//! Per [Wiki ADR-031][09-adr-031] the tensor sub-crate exposes
//! `TensorAxis` as the canonical Layer-3 surface for tensor compute.
//! The reference impl [`CpuI8MatmulSquare`] is generic over the square
//! dimension `DIM`, with `i8` inputs and saturating-`i16` outputs —
//! the integer-arithmetic determinism contract ADR-030 names as the
//! axis substitution-determinism baseline.
//!
//! Variable-rank tensor compute composes through verbs over
//! `partition_product!`-declared shapes per ADR-033/044; the axis's
//! role is the fixed-shape atomic primitive.
//!
//! # ADR-054 (4) substrate-Term verb body — forward work
//!
//! Per [Wiki ADR-055](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
//! every `AxisExtension` impl satisfies the substrate-Term verb body
//! discipline; the hand-written kernel below uses the default empty
//! `body_arena()` emitted by foundation-sdk 0.4.8's `axis!` companion
//! macro (the primitive-fast-path-equivalent realization).
//!
//! The richer explicit substrate-Term decomposition of
//! `CpuI8MatmulSquare<DIM>::matmul` is `fold_n(DIM, ...)` over rows ×
//! `fold_n(DIM, ...)` over columns × `fold_n(DIM, ...)` over
//! reductions, with a `sign_extend` sub-verb (matching `Ge(operand,
//! Literal(0x80, W8))` to select between `Concat(0x00, operand)` and
//! `Concat(0xff, operand)`) plus W16 `Mul` + W16 `Add` accumulation
//! plus saturation via `Match` over `Ge(acc, Literal(0x7fff, W16))` /
//! `Lt(acc, Literal(0x8000, W16))`. Forward work co-gated on
//! foundation-sdk admitting `le`/`lt`/`ge`/`gt` and `concat` as
//! verb-body call forms (currently rejected per ADR-035 ψ-residuals
//! discipline).
//!
//! **Blocked on upstream `uor-foundation-sdk` grammar extension.**
//! Foundation-sdk 0.4.7's `verb!` body grammar rejects `le`/`lt`/`ge`/`gt`
//! per ADR-035 ψ-residuals discipline (these comparison primitives are
//! reserved for the ψ-chain, not verb-body branching) and rejects
//! `concat` (needed for the byte-prefixing pattern of the sign-extend
//! sub-verb). Until foundation-sdk extends the grammar to admit a
//! `match`-able byte-level comparison form for use in tensor-saturation
//! bodies, the substrate-Term matmul verb cannot be expressed.
//!
//! The hand-written `for`-loop kernel below is the operational form;
//! byte-output equivalence with BLAS reference outputs at integer
//! precision is checked at `tests/conformance.rs`.
//!
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-054]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(missing_docs)]

use uor_foundation::enforcement::{GroundedShape, ShapeViolation};
use uor_foundation::pipeline::{
    AxisExtension, ConstrainedTypeShape, ConstraintRef, IntoBindingValue,
};
use uor_foundation_sdk::axis;

axis! {
    /// Wiki ADR-031 tensor-compute axis.
    ///
    /// The reference impl `CpuI8MatmulSquare<DIM>` is parametric in
    /// `DIM` for square `DIM × DIM` `i8` matrices, emitting a `DIM ×
    /// DIM` `i16` product (saturating) per ADR-030's bit-determinism
    /// commitment.
    pub trait TensorAxis: AxisExtension {
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/TensorAxis";
        /// `2 * MAX_TENSOR_DIM * MAX_TENSOR_DIM` = 512 bytes for
        /// 16×16. Overridden per impl.
        const MAX_OUTPUT_BYTES: usize = 32;
        /// Multiply two row-major `DIM × DIM` `i8` matrices into a
        /// `DIM × DIM` `i16` product (saturating). Input is `A || B`
        /// (`2 * DIM * DIM` bytes); output is `2 * DIM * DIM` bytes.
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output byte-length mismatch.
        fn matmul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

/// Maximum square dimension any [`CpuI8MatmulSquare`] instantiation
/// supports. Cap at 16: output buffer = `2 * 16 * 16` = 512 bytes,
/// inputs = 256 bytes each, total kernel byte budget bounded.
pub const MAX_TENSOR_DIM: usize = 16;

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

/// Parametric square `DIM × DIM` `i8` × `i8` → `i16` matmul.
///
/// Determinism: per ADR-030's per-axis substitution-determinism note,
/// the integer-arithmetic CPU impl preserves bit-identity across
/// targets. `DIM` is the square dimension; for non-square /
/// non-integer / variable-shape tensor compute the wiki's pattern is
/// to compose this axis kernel through verbs over `partition_product!`
/// (per ADR-033/044) — the axis layer fixes the atom shape.
#[derive(Debug, Clone, Copy)]
pub struct CpuI8MatmulSquare<const DIM: usize>;

impl<const DIM: usize> Default for CpuI8MatmulSquare<DIM> {
    fn default() -> Self {
        Self
    }
}

impl<const DIM: usize> CpuI8MatmulSquare<DIM> {
    const fn idx(row: usize, col: usize) -> usize {
        row * DIM + col
    }
}

impl<const DIM: usize> TensorAxis for CpuI8MatmulSquare<DIM> {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/TensorAxis/CpuI8MatmulSquare";
    const MAX_OUTPUT_BYTES: usize = 2 * DIM * DIM;

    fn matmul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if DIM == 0 || DIM > MAX_TENSOR_DIM {
            return Err(arity_violation(
                "https://uor.foundation/axis/TensorAxisShape/dimInRange",
            ));
        }
        let mat_bytes = DIM * DIM;
        let input_bytes = 2 * mat_bytes;
        let output_bytes = 2 * mat_bytes;
        if input.len() != input_bytes {
            return Err(arity_violation(
                "https://uor.foundation/axis/TensorAxisShape/inputByteLength",
            ));
        }
        if out.len() < output_bytes {
            return Err(arity_violation(
                "https://uor.foundation/axis/TensorAxisShape/outputByteLength",
            ));
        }
        let (a_bytes, b_bytes) = input.split_at(mat_bytes);
        for row in 0..DIM {
            for col in 0..DIM {
                let mut acc: i32 = 0;
                for k in 0..DIM {
                    #[allow(clippy::cast_possible_wrap)]
                    let a = i32::from(a_bytes[Self::idx(row, k)] as i8);
                    #[allow(clippy::cast_possible_wrap)]
                    let b = i32::from(b_bytes[Self::idx(k, col)] as i8);
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
                let cell = Self::idx(row, col);
                out[2 * cell..2 * cell + 2].copy_from_slice(&saturated.to_be_bytes());
            }
        }
        Ok(output_bytes)
    }
}

// ADR-052 generic-form companion.
axis_extension_impl_for_tensor_axis!(@generic CpuI8MatmulSquare<DIM>, [const DIM: usize]);

/// 4×4 `i8` matmul — the canonical small-tensor reference.
pub type CpuI8Tensor4x4Matmul = CpuI8MatmulSquare<4>;
/// 8×8 `i8` matmul.
pub type CpuI8Tensor8x8Matmul = CpuI8MatmulSquare<8>;
/// 16×16 `i8` matmul (the `MAX_TENSOR_DIM` ceiling).
pub type CpuI8Tensor16x16Matmul = CpuI8MatmulSquare<16>;

// ---- MatrixShape: ConstrainedTypeShape carrier ----

/// Parametric ConstrainedTypeShape for a row-major `ROWS × COLS`
/// matrix of `ELEM_BYTES`-byte elements. Per ADR-031's `Tensor<Element,
/// Shape>` shape commitment, restricted to matrix rank-2 here; higher
/// ranks compose through `partition_product!` per ADR-033/044.
#[derive(Debug, Clone, Copy)]
pub struct MatrixShape<const ROWS: usize, const COLS: usize, const ELEM_BYTES: usize>;

impl<const ROWS: usize, const COLS: usize, const ELEM_BYTES: usize> Default
    for MatrixShape<ROWS, COLS, ELEM_BYTES>
{
    fn default() -> Self {
        Self
    }
}

impl<const ROWS: usize, const COLS: usize, const ELEM_BYTES: usize> ConstrainedTypeShape
    for MatrixShape<ROWS, COLS, ELEM_BYTES>
{
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = ROWS * COLS * ELEM_BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow((ROWS * COLS * ELEM_BYTES) as u32);
}

impl<const ROWS: usize, const COLS: usize, const ELEM_BYTES: usize>
    uor_foundation::pipeline::__sdk_seal::Sealed for MatrixShape<ROWS, COLS, ELEM_BYTES>
{
}
impl<const ROWS: usize, const COLS: usize, const ELEM_BYTES: usize> GroundedShape
    for MatrixShape<ROWS, COLS, ELEM_BYTES>
{
}
impl<const ROWS: usize, const COLS: usize, const ELEM_BYTES: usize> IntoBindingValue
    for MatrixShape<ROWS, COLS, ELEM_BYTES>
{
    const MAX_BYTES: usize = ROWS * COLS * ELEM_BYTES;

    fn into_binding_bytes(&self, _out: &mut [u8]) -> Result<usize, ShapeViolation> {
        Ok(0)
    }
}

/// Parametric ConstrainedTypeShape for a length-`N` vector of
/// `ELEM_BYTES`-byte elements. Per ADR-031's `Tensor<Element, Shape>`
/// for rank-1.
#[derive(Debug, Clone, Copy)]
pub struct VectorShape<const N: usize, const ELEM_BYTES: usize>;

impl<const N: usize, const ELEM_BYTES: usize> Default for VectorShape<N, ELEM_BYTES> {
    fn default() -> Self {
        Self
    }
}

impl<const N: usize, const ELEM_BYTES: usize> ConstrainedTypeShape for VectorShape<N, ELEM_BYTES> {
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = N * ELEM_BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow((N * ELEM_BYTES) as u32);
}

impl<const N: usize, const ELEM_BYTES: usize> uor_foundation::pipeline::__sdk_seal::Sealed
    for VectorShape<N, ELEM_BYTES>
{
}
impl<const N: usize, const ELEM_BYTES: usize> GroundedShape for VectorShape<N, ELEM_BYTES> {}
impl<const N: usize, const ELEM_BYTES: usize> IntoBindingValue for VectorShape<N, ELEM_BYTES> {
    const MAX_BYTES: usize = N * ELEM_BYTES;

    fn into_binding_bytes(&self, _out: &mut [u8]) -> Result<usize, ShapeViolation> {
        Ok(0)
    }
}
