//! GGML / GGUF / ONNX tensor element-type alphabet per [Wiki ADR-031][09].
//!
//! Each dtype is a sealed unit struct implementing [`ConstrainedTypeShape`]
//! with the generic IRI per [ADR-017][09]'s empty-CONSTRAINTS closure rule:
//! at the framework level two shapes with identical block-byte counts
//! content-address identically. The Rust-type distinction is the
//! application-level ergonomics surface; per-dtype block-layout metadata is
//! exposed through the [`Dtype`] trait's associated constants and through
//! the [`TensorDtypeRegistry`] entries populated by `register_shape!` per
//! [ADR-057][09].
//!
//! Per [ADR-058][09] the κ-derivation is the framework's
//! compression-to-canonical-form operator. Tensor element types occupy the
//! R-level of the operator-geometry codomain per [ADR-059][09] (algebra
//! dimension 1; characteristic identity *existence* — the
//! byte-element-present distinction). Tensor composition through
//! `partition_product!` per ADR-033/044 ascends the convergence tower from
//! R to higher levels.
//!
//! # Container-format consumption
//!
//! Container-format realizations (e.g., `uor-addr` GGUF / ONNX realizations
//! as Layer-2 sibling format families) consume this alphabet through the
//! `TensorDtypeRegistry` per ADR-057's `Term::Recurse` lowering rule. A
//! container's `address_inference` verb body references a dtype shape
//! through a `Recurse` constraint whose `descent_bound` is the
//! application-declared recursion-descent cap per ADR-057 (here `8`, the
//! O-level algebra dimension per ADR-025):
//!
//! ```
//! use uor_foundation::pipeline::ConstraintRef;
//!
//! let dtype_ref = ConstraintRef::Recurse {
//!     shape_iri: "https://uor.foundation/type/ConstrainedType",
//!     descent_bound: 8,
//! };
//! assert!(matches!(
//!     dtype_ref,
//!     ConstraintRef::Recurse { descent_bound: 8, .. }
//! ));
//! ```
//!
//! and the runtime ψ_1 NerveResolver expands the reference through the
//! `TensorDtypeRegistry`'s entries, decrementing `descent_bound` on each
//! traversal per ADR-057.
//!
//! # Element types
//!
//! ## Continuous floating-point
//!
//! | Dtype | Block bytes | Block elems | Notes |
//! |-------|-------------|-------------|-------|
//! | [`F32`] | 4 | 1 | IEEE 754 binary32 |
//! | [`F16`] | 2 | 1 | IEEE 754 binary16 |
//! | [`BF16`] | 2 | 1 | Brain Float 16 |
//! | [`F64`] | 8 | 1 | IEEE 754 binary64 |
//!
//! ## ONNX 8-bit floating-point (TensorProto.DataType 17–20)
//!
//! | Dtype | Block bytes | Block elems | Notes |
//! |-------|-------------|-------------|-------|
//! | [`F8_E4M3`] | 1 | 1 | FLOAT8E4M3FN (no inf, finite-NaN) |
//! | [`F8_E4M3_FNUZ`] | 1 | 1 | FLOAT8E4M3FNUZ (no neg-zero variant) |
//! | [`F8_E5M2`] | 1 | 1 | FLOAT8E5M2 |
//! | [`F8_E5M2_FNUZ`] | 1 | 1 | FLOAT8E5M2FNUZ |
//!
//! ## ONNX complex
//!
//! | Dtype | Block bytes | Block elems | Notes |
//! |-------|-------------|-------------|-------|
//! | [`C64`] | 8 | 1 | COMPLEX64: real + imag as `(F32, F32)` |
//! | [`C128`] | 16 | 1 | COMPLEX128: real + imag as `(F64, F64)` |
//!
//! ## Signed / unsigned integer
//!
//! | Dtype | Block bytes |   | Dtype | Block bytes |
//! |-------|-------------|---|-------|-------------|
//! | [`I8`]  | 1 |   | [`U8`]  | 1 |
//! | [`I16`] | 2 |   | [`U16`] | 2 |
//! | [`I32`] | 4 |   | [`U32`] | 4 |
//! | [`I64`] | 8 |   | [`U64`] | 8 |
//!
//! [`BOOL`] is 1 byte (`0x00` = false, any non-zero = true).
//!
//! ## ONNX packed 4-bit (TensorProto.DataType 21–23)
//!
//! | Dtype | Block bytes | Block elems | Layout |
//! |-------|-------------|-------------|--------|
//! | [`I4`] | 1 | 2 | INT4: two signed 4-bit nibbles |
//! | [`U4`] | 1 | 2 | UINT4: two unsigned 4-bit nibbles |
//! | [`F4_E2M1`] | 1 | 2 | FLOAT4E2M1: two 4-bit floats |
//!
//! ## GGML legacy block-32 quantization (`block_q*_*`)
//!
//! | Dtype | Block bytes | Block elems | Layout (canonical) |
//! |-------|-------------|-------------|--------------------|
//! | [`Q4_0`] | 18 | 32 | `{d:f16, qs:[u8;16]}` |
//! | [`Q4_1`] | 20 | 32 | `{d:f16, m:f16, qs:[u8;16]}` |
//! | [`Q5_0`] | 22 | 32 | `{d:f16, qh:[u8;4], qs:[u8;16]}` |
//! | [`Q5_1`] | 24 | 32 | `{d:f16, m:f16, qh:[u8;4], qs:[u8;16]}` |
//! | [`Q8_0`] | 34 | 32 | `{d:f16, qs:[i8;32]}` |
//! | [`Q8_1`] | 36 | 32 | `{d:f16, s:f16, qs:[i8;32]}` |
//!
//! ## GGML K-series block-256 quantization (`block_q*_K`)
//!
//! | Dtype | Block bytes | Block elems |
//! |-------|-------------|-------------|
//! | [`Q2_K`] | 84  | 256 |
//! | [`Q3_K`] | 110 | 256 |
//! | [`Q4_K`] | 144 | 256 |
//! | [`Q5_K`] | 176 | 256 |
//! | [`Q6_K`] | 210 | 256 |
//! | [`Q8_K`] | 292 | 256 |
//!
//! ## GGML IQ-series importance-aware quantization (`block_iq*`)
//!
//! | Dtype | Block bytes | Block elems | Bits per weight |
//! |-------|-------------|-------------|-----------------|
//! | [`IQ1_S`] | 50 | 256 | ~1.5625 |
//! | [`IQ1_M`] | 56 | 256 | ~1.75 |
//! | [`IQ2_XXS`] | 66 | 256 | ~2.0625 |
//! | [`IQ2_XS`] | 74 | 256 | ~2.3125 |
//! | [`IQ2_S`] | 82 | 256 | ~2.5625 |
//! | [`IQ3_XXS`] | 98 | 256 | ~3.0625 |
//! | [`IQ3_S`] | 110 | 256 | ~3.4375 |
//! | [`IQ4_NL`] | 18 | 32 | ~4.5 (block-32) |
//! | [`IQ4_XS`] | 136 | 256 | ~4.25 |
//!
//! Note: under [ADR-017][09]'s closure rule, dtypes with identical
//! `BLOCK_BYTES` content-address identically at the framework level.
//! Examples include: [`F16`] / [`BF16`] / [`I16`] / [`U16`] (2-byte
//! shapes); [`Q4_0`] / [`IQ4_NL`] (18-byte shapes); [`Q3_K`] /
//! [`IQ3_S`] (110-byte shapes). The Rust-type distinction carries the
//! application-level interpretation; framework canonical-form emission
//! depends only on `(SITE_COUNT, CONSTRAINTS)`.
//!
//! # See also
//!
//! - [Wiki: 09 Architecture Decisions § ADR-017 — empty-CONSTRAINTS closure rule][09]
//! - [Wiki: 09 Architecture Decisions § ADR-031 — `prism` is the standard library][09]
//! - [Wiki: 09 Architecture Decisions § ADR-057 — Bounded recursive structural typing][09]
//! - [Wiki: 09 Architecture Decisions § ADR-058 — κ-derivation as compression operator][09]
//! - [Wiki: 09 Architecture Decisions § ADR-059 — Atlas image / Hopf tower as codomain][09]
//!
//! [09]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(non_camel_case_types)]

use uor_foundation::enforcement::GroundedShape;
use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef, IntoBindingValue, TermValue};
use uor_foundation_sdk::register_shape;

// ---- GGML quantization-block parameters per `ggml-common.h` ----
//
// Every per-dtype `BLOCK_BYTES` constant in this module is derived
// from these named parameters using the same arithmetic the ggml
// header's `static_assert` lines use. No per-dtype literal byte
// counts are written; updating the ggml side reduces to bumping the
// parameters here.

/// Super-block size for K-series and IQ-series quantization
/// (`QK_K` in ggml-common.h).
const QK_K: usize = 256;

/// Legacy 4-bit (linear) quantization block size
/// (`QK4_0` / `QK4_1` in ggml-common.h).
const QK4_0: usize = 32;
/// Legacy 5-bit quantization block size
/// (`QK5_0` / `QK5_1` in ggml-common.h).
const QK5_0: usize = 32;
/// Legacy 8-bit quantization block size
/// (`QK8_0` / `QK8_1` in ggml-common.h).
const QK8_0: usize = 32;
/// Non-linear 4-bit block size
/// (`QK4_NL` in ggml-common.h — distinct named constant despite
/// numeric equality with `QK4_0`).
const QK4_NL: usize = 32;

/// `sizeof(ggml_half)`: 16-bit float byte count — the canonical
/// per-block scale carrier across the K-series and most IQ-series
/// quantization types.
const GGML_HALF: usize = 2;
/// `sizeof(uint16_t)`: the IQ4_XS high-scales field byte count.
const GGML_U16: usize = 2;
/// `sizeof(float)`: 32-bit float byte count used by `block_q8_K`'s
/// super-block scale.
const GGML_FLOAT: usize = 4;
/// `sizeof(int16_t)`: per-sub-block sum carrier in `block_q8_K`.
const GGML_I16: usize = 2;

/// K-quant scales array byte count
/// (`K_SCALE_SIZE` in ggml-common.h).
const K_SCALE_SIZE: usize = 12;
/// IQ3_S per-block extra scale byte count
/// (`IQ3S_N_SCALE` in ggml-common.h).
const IQ3S_N_SCALE: usize = 4;

// ---- Native-width byte parameters (continuous IEEE 754 / integer
// element sizes — sized in bits, byte count = bits / 8) ----

const BITS_PER_BYTE: usize = 8;

/// Bytes in one IEEE 754 binary16 / Brain Float 16 element.
const F16_BYTES: usize = 16 / BITS_PER_BYTE;
/// Bytes in one IEEE 754 binary32 element.
const F32_BYTES: usize = 32 / BITS_PER_BYTE;
/// Bytes in one IEEE 754 binary64 element.
const F64_BYTES: usize = 64 / BITS_PER_BYTE;
/// Bytes in one 8-bit ONNX FLOAT8 element.
const F8_BYTES: usize = 8 / BITS_PER_BYTE;
/// Bytes in one signed/unsigned 8-bit integer.
const INT8_BYTES: usize = 8 / BITS_PER_BYTE;
/// Bytes in one signed/unsigned 16-bit integer.
const INT16_BYTES: usize = 16 / BITS_PER_BYTE;
/// Bytes in one signed/unsigned 32-bit integer.
const INT32_BYTES: usize = 32 / BITS_PER_BYTE;
/// Bytes in one signed/unsigned 64-bit integer.
const INT64_BYTES: usize = 64 / BITS_PER_BYTE;
/// Bytes in one byte-sized boolean.
const BOOL_BYTES: usize = 8 / BITS_PER_BYTE;

/// Bytes in one ONNX 4-bit packed block (two nibbles per byte).
const PACKED_NIBBLE_BYTES: usize = 8 / BITS_PER_BYTE;
/// Elements per ONNX 4-bit packed block (two nibbles per byte).
const PACKED_NIBBLE_ELEMS: usize = 2;

/// Bytes in one ONNX COMPLEX64 element (`(F32, F32)` real + imag).
const C64_BYTES: usize = 2 * F32_BYTES;
/// Bytes in one ONNX COMPLEX128 element (`(F64, F64)` real + imag).
const C128_BYTES: usize = 2 * F64_BYTES;

mod sealed {
    pub trait Sealed {}
}

/// Tensor element-type discipline.
///
/// A `Dtype` declares the canonical block layout of one tensor element
/// (continuous types) or one quantization block (block-32 / block-256
/// quantized types). The trait is sealed per ADR-014; impls are confined
/// to this module and reachable through [`TensorDtypeRegistry`] for
/// container-format-realization enumeration per ADR-057.
///
/// Per ADR-017's closure rule, two `Dtype` impls with identical
/// [`BLOCK_BYTES`](Dtype::BLOCK_BYTES) content-address identically at the
/// framework level. The Rust-type distinction is the application-level
/// ergonomics surface (e.g., `F16` vs `BF16` are framework-equivalent
/// 2-byte shapes but Rust-distinct dtypes carrying distinct
/// floating-point interpretations at the container-format layer).
pub trait Dtype:
    sealed::Sealed
    + ConstrainedTypeShape
    + GroundedShape
    + for<'a> IntoBindingValue<'a>
    + Default
    + Copy
    + 'static
{
    /// Per-dtype name string (uppercase, GGML-convention spelling).
    const NAME: &'static str;
    /// Bytes per block. For continuous element types this is the element
    /// byte count; for quantized types this is the block-bytes count
    /// (= the GGML `block_q*_*` `sizeof`).
    const BLOCK_BYTES: usize;
    /// Elements per block. `1` for continuous types; `32` for legacy
    /// quantization; `256` for K-series quantization.
    const BLOCK_ELEMS: usize;
}

macro_rules! decl_dtype {
    ($(#[$attr:meta])* $name:ident, $bytes:expr, $elems:expr) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $name;

        impl sealed::Sealed for $name {}
        impl uor_foundation::pipeline::__sdk_seal::Sealed for $name {}

        impl ConstrainedTypeShape for $name {
            const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
            const SITE_COUNT: usize = $bytes;
            const CONSTRAINTS: &'static [ConstraintRef] = &[];
            #[allow(clippy::cast_possible_truncation)]
            const CYCLE_SIZE: u64 = 256u64.saturating_pow(($bytes) as u32);
        }

        impl GroundedShape for $name {}

        impl<'a> IntoBindingValue<'a> for $name {
            fn as_binding_value<const INLINE_BYTES: usize>(&self) -> TermValue<'a, INLINE_BYTES> {
                TermValue::empty()
            }
        }

        impl Dtype for $name {
            const NAME: &'static str = stringify!($name);
            const BLOCK_BYTES: usize = $bytes;
            const BLOCK_ELEMS: usize = $elems;
        }
    };
}

// ---- Continuous floating-point ----

decl_dtype!(
    /// 32-bit IEEE 754 binary32.
    F32, F32_BYTES, 1
);
decl_dtype!(
    /// 16-bit IEEE 754 binary16
    /// (1 sign + 5 exponent + 10 mantissa).
    F16, F16_BYTES, 1
);
decl_dtype!(
    /// Brain Float 16
    /// (1 sign + 8 exponent + 7 mantissa). Framework-equivalent to
    /// [`F16`] under ADR-017's closure rule (same `BLOCK_BYTES`).
    BF16, F16_BYTES, 1
);
decl_dtype!(
    /// 64-bit IEEE 754 binary64.
    F64, F64_BYTES, 1
);

// ---- ONNX 8-bit floating-point ----

decl_dtype!(
    /// FLOAT8E4M3FN (1 sign + 4 exponent + 3 mantissa,
    /// finite-NaN, no infinities). ONNX TensorProto.DataType = 17.
    F8_E4M3, F8_BYTES, 1
);
decl_dtype!(
    /// FLOAT8E4M3FNUZ (no negative-zero variant). ONNX
    /// TensorProto.DataType = 18.
    F8_E4M3_FNUZ, F8_BYTES, 1
);
decl_dtype!(
    /// FLOAT8E5M2 (1 sign + 5 exponent + 2 mantissa). ONNX
    /// TensorProto.DataType = 19.
    F8_E5M2, F8_BYTES, 1
);
decl_dtype!(
    /// FLOAT8E5M2FNUZ (no negative-zero variant). ONNX
    /// TensorProto.DataType = 20.
    F8_E5M2_FNUZ, F8_BYTES, 1
);

// ---- ONNX complex ----

decl_dtype!(
    /// COMPLEX64: `(F32, F32)` real + imaginary. ONNX
    /// TensorProto.DataType = 14.
    C64, C64_BYTES, 1
);
decl_dtype!(
    /// COMPLEX128: `(F64, F64)` real + imaginary. ONNX
    /// TensorProto.DataType = 15.
    C128, C128_BYTES, 1
);

// ---- Signed integers ----

decl_dtype!(
    /// 8-bit signed integer.
    I8, INT8_BYTES, 1
);
decl_dtype!(
    /// 16-bit signed integer.
    I16, INT16_BYTES, 1
);
decl_dtype!(
    /// 32-bit signed integer.
    I32, INT32_BYTES, 1
);
decl_dtype!(
    /// 64-bit signed integer.
    I64, INT64_BYTES, 1
);

// ---- Unsigned integers ----

decl_dtype!(
    /// 8-bit unsigned integer.
    U8, INT8_BYTES, 1
);
decl_dtype!(
    /// 16-bit unsigned integer.
    U16, INT16_BYTES, 1
);
decl_dtype!(
    /// 32-bit unsigned integer.
    U32, INT32_BYTES, 1
);
decl_dtype!(
    /// 64-bit unsigned integer.
    U64, INT64_BYTES, 1
);

decl_dtype!(
    /// Boolean (`0x00` = false, any non-zero = true).
    BOOL, BOOL_BYTES, 1
);

// ---- ONNX packed 4-bit (TensorProto.DataType 21–23) ----

decl_dtype!(
    /// INT4: two signed 4-bit nibbles per byte. ONNX
    /// TensorProto.DataType = 22.
    I4, PACKED_NIBBLE_BYTES, PACKED_NIBBLE_ELEMS
);
decl_dtype!(
    /// UINT4: two unsigned 4-bit nibbles per byte. ONNX
    /// TensorProto.DataType = 21.
    U4, PACKED_NIBBLE_BYTES, PACKED_NIBBLE_ELEMS
);
decl_dtype!(
    /// FLOAT4E2M1: two 4-bit floats per byte
    /// (1 sign + 2 exponent + 1 mantissa). ONNX TensorProto.DataType
    /// = 23.
    F4_E2M1, PACKED_NIBBLE_BYTES, PACKED_NIBBLE_ELEMS
);

// ---- GGML legacy block-32 quantization ----
//
// Per `ggml-common.h` `static_assert` lines, the block sizes are:
//
//   block_q4_0:  sizeof(ggml_half) +   QK4_0 / 2
//   block_q4_1: 2*sizeof(ggml_half) +  QK4_1 / 2
//   block_q5_0:  sizeof(ggml_half) +   sizeof(uint32_t) + QK5_0 / 2
//   block_q5_1: 2*sizeof(ggml_half) +  sizeof(uint32_t) + QK5_1 / 2
//   block_q8_0:  sizeof(ggml_half) +   QK8_0
//   block_q8_1: 2*sizeof(ggml_half) +  QK8_1
//
// (Q5_0/Q5_1 use a `uint32_t qh` for high-bit storage, encoded as
// `2 * GGML_U16` here.)

decl_dtype!(
    /// Legacy 4-bit quant (scale only): `{d:f16, qs:[u8;16]}`.
    Q4_0, GGML_HALF + QK4_0 / 2, QK4_0
);
decl_dtype!(
    /// Legacy 4-bit quant (scale + min): `{d:f16, m:f16, qs:[u8;16]}`.
    Q4_1, 2 * GGML_HALF + QK4_0 / 2, QK4_0
);
decl_dtype!(
    /// Legacy 5-bit quant: `{d:f16, qh:u32, qs:[u8;16]}`.
    Q5_0, GGML_HALF + 2 * GGML_U16 + QK5_0 / 2, QK5_0
);
decl_dtype!(
    /// Legacy 5-bit quant (with min): `{d:f16, m:f16, qh:u32, qs:[u8;16]}`.
    Q5_1, 2 * GGML_HALF + 2 * GGML_U16 + QK5_0 / 2, QK5_0
);
decl_dtype!(
    /// Legacy 8-bit quant: `{d:f16, qs:[i8;32]}`.
    Q8_0, GGML_HALF + QK8_0, QK8_0
);
decl_dtype!(
    /// Legacy 8-bit quant (with sum): `{d:f16, s:f16, qs:[i8;32]}`.
    Q8_1, 2 * GGML_HALF + QK8_0, QK8_0
);

// ---- GGML K-series block-256 quantization ----
//
// Per `ggml-common.h` `static_assert` lines:
//
//   block_q2_K: 2*sizeof(ggml_half) + QK_K/16 + QK_K/4
//   block_q3_K:  sizeof(ggml_half) + QK_K/4  + QK_K/8  + K_SCALE_SIZE
//   block_q4_K: 2*sizeof(ggml_half) + K_SCALE_SIZE + QK_K/2
//   block_q5_K: 2*sizeof(ggml_half) + K_SCALE_SIZE + QK_K/2 + QK_K/8
//   block_q6_K:  sizeof(ggml_half) + QK_K/16 + 3*QK_K/4
//   block_q8_K:  sizeof(float)     + QK_K    + QK_K/16 * sizeof(int16_t)

decl_dtype!(
    /// K-quant 2-bit.
    Q2_K, 2 * GGML_HALF + QK_K / 16 + QK_K / 4, QK_K
);
decl_dtype!(
    /// K-quant 3-bit.
    Q3_K, GGML_HALF + QK_K / 4 + QK_K / 8 + K_SCALE_SIZE, QK_K
);
decl_dtype!(
    /// K-quant 4-bit.
    Q4_K, 2 * GGML_HALF + K_SCALE_SIZE + QK_K / 2, QK_K
);
decl_dtype!(
    /// K-quant 5-bit.
    Q5_K, 2 * GGML_HALF + K_SCALE_SIZE + QK_K / 2 + QK_K / 8, QK_K
);
decl_dtype!(
    /// K-quant 6-bit.
    Q6_K, GGML_HALF + QK_K / 16 + 3 * QK_K / 4, QK_K
);
decl_dtype!(
    /// K-quant 8-bit (intermediate accumulator).
    Q8_K, GGML_FLOAT + QK_K + QK_K / 16 * GGML_I16, QK_K
);

// ---- GGML IQ-series importance-aware quantization ----
//
// Per `ggml-common.h` `static_assert` lines:
//
//   block_iq1_s:   sizeof(ggml_half) + QK_K/8  + QK_K/16
//   block_iq1_m:                       QK_K/8  + QK_K/16 + QK_K/32
//                                      (no half-prefix; per-block
//                                       scales replace the half scale)
//   block_iq2_xxs: sizeof(ggml_half) + QK_K/8 * sizeof(uint16_t)
//   block_iq2_xs:  sizeof(ggml_half) + QK_K/8 * sizeof(uint16_t) + QK_K/32
//   block_iq2_s:   sizeof(ggml_half) + QK_K/4 + QK_K/16
//   block_iq3_xxs: sizeof(ggml_half) + 3*(QK_K/8)
//   block_iq3_s:   sizeof(ggml_half) + 13*(QK_K/32) + IQ3S_N_SCALE
//   block_iq4_nl:  sizeof(ggml_half) + QK4_NL/2
//   block_iq4_xs:  sizeof(ggml_half) + sizeof(uint16_t) + QK_K/64 + QK_K/2

decl_dtype!(
    /// IQ1_S — ~1.5625 bits per weight.
    IQ1_S, GGML_HALF + QK_K / 8 + QK_K / 16, QK_K
);
decl_dtype!(
    /// IQ1_M — ~1.75 bits per weight (no `ggml_half d` prefix; scales
    /// are per-sub-block).
    IQ1_M, QK_K / 8 + QK_K / 16 + QK_K / 32, QK_K
);
decl_dtype!(
    /// IQ2_XXS — ~2.0625 bits per weight. Grid-indexed via lookup
    /// table.
    IQ2_XXS, GGML_HALF + QK_K / 8 * GGML_U16, QK_K
);
decl_dtype!(
    /// IQ2_XS — ~2.3125 bits per weight.
    IQ2_XS, GGML_HALF + QK_K / 8 * GGML_U16 + QK_K / 32, QK_K
);
decl_dtype!(
    /// IQ2_S — ~2.5625 bits per weight.
    IQ2_S, GGML_HALF + QK_K / 4 + QK_K / 16, QK_K
);
decl_dtype!(
    /// IQ3_XXS — ~3.0625 bits per weight.
    IQ3_XXS, GGML_HALF + 3 * (QK_K / 8), QK_K
);
decl_dtype!(
    /// IQ3_S — ~3.4375 bits per weight.
    IQ3_S, GGML_HALF + 13 * (QK_K / 32) + IQ3S_N_SCALE, QK_K
);
decl_dtype!(
    /// IQ4_NL — non-linear 4-bit; block-32 (not K-series).
    IQ4_NL, GGML_HALF + QK4_NL / 2, QK4_NL
);
decl_dtype!(
    /// IQ4_XS — ~4.25 bits per weight.
    IQ4_XS, GGML_HALF + GGML_U16 + QK_K / 64 + QK_K / 2, QK_K
);

// ---- Shape-IRI registry per ADR-057 ----

register_shape!(
    TensorDtypeRegistry,
    // Continuous floating-point
    F32,
    F16,
    BF16,
    F64,
    // ONNX FLOAT8 family
    F8_E4M3,
    F8_E4M3_FNUZ,
    F8_E5M2,
    F8_E5M2_FNUZ,
    // ONNX complex
    C64,
    C128,
    // Signed integers
    I8,
    I16,
    I32,
    I64,
    // Unsigned integers
    U8,
    U16,
    U32,
    U64,
    // Boolean
    BOOL,
    // ONNX packed 4-bit
    I4,
    U4,
    F4_E2M1,
    // GGML legacy block-32 quantization
    Q4_0,
    Q4_1,
    Q5_0,
    Q5_1,
    Q8_0,
    Q8_1,
    // GGML K-series block-256 quantization
    Q2_K,
    Q3_K,
    Q4_K,
    Q5_K,
    Q6_K,
    Q8_K,
    // GGML IQ-series importance-aware quantization
    IQ1_S,
    IQ1_M,
    IQ2_XXS,
    IQ2_XS,
    IQ2_S,
    IQ3_XXS,
    IQ3_S,
    IQ4_NL,
    IQ4_XS
);
