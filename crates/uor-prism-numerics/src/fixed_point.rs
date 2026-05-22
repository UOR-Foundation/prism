//! `FixedPointAxis` declaration + parametric Q-format impl + shape.
//!
//! Per [Wiki ADR-031][09-adr-031] the numerics sub-crate exposes
//! `FixedPointAxis` and `FixedPoint<I, F>` as the canonical Layer-3
//! surface for Q-format fixed-point arithmetic. The reference impl
//! [`FixedPointQNumeric`] is generic over integer-bit width `I` and
//! fraction-bit width `F`, with `I + F ≤ 64` (so each value fits a
//! single signed 64-bit container).
//!
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(missing_docs)]

use uor_foundation::enforcement::{GroundedShape, ShapeViolation};
use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef, IntoBindingValue, TermValue};
use uor_foundation_sdk::axis;

use crate::{check_output, split_pair};

axis! {
    /// Wiki ADR-031 fixed-point arithmetic axis.
    ///
    /// Operates on Q-format two's-complement integers within a signed
    /// 64-bit container. The reference impl
    /// `FixedPointQNumeric<I, F>` is generic over the integer-bit /
    /// fraction-bit split, with `I + F ≤ 64`.
    pub trait FixedPointAxis: AxisExtension {
        /// ADR-017 content address.
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/FixedPointAxis";
        /// Operand byte-width (fixed 8 bytes = i64 container).
        const MAX_OUTPUT_BYTES: usize = 8;
        /// Q-format addition: `a + b`. Input `a || b` (16 bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn add(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
        /// Q-format subtraction: `a - b`. Input `a || b` (16 bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn sub(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
        /// Q-format multiplication with bias-aware re-scaling by `F`
        /// fraction bits.
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

const WIDTH: usize = 8;

fn format_violation() -> ShapeViolation {
    ShapeViolation {
        shape_iri: "https://uor.foundation/axis/FixedPointAxis",
        constraint_iri: "https://uor.foundation/axis/FixedPointAxis/iPlusFInRange",
        property_iri: "https://uor.foundation/axis/qFormatTotalBits",
        expected_range: "https://uor.foundation/axis/FixedPointAxis/I64Fit",
        min_count: 1,
        max_count: 64,
        kind: uor_foundation::ViolationKind::ValueCheck,
    }
}

fn decode(slice: &[u8]) -> i64 {
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&slice[..8]);
    i64::from_be_bytes(buf)
}

fn encode(value: i64) -> [u8; 8] {
    value.to_be_bytes()
}

/// Parametric Q-format fixed-point arithmetic.
///
/// `INT_BITS + FRAC_BITS ≤ 64` and `INT_BITS + FRAC_BITS ≥ 1`. Values
/// are two's-complement signed integers in the canonical 64-bit
/// container; the `INT_BITS`/`FRAC_BITS` split governs the implicit
/// decimal point and the multiplication re-scaling.
#[derive(Debug, Clone, Copy)]
pub struct FixedPointQNumeric<const INT_BITS: u32, const FRAC_BITS: u32>;

impl<const I: u32, const F: u32> Default for FixedPointQNumeric<I, F> {
    fn default() -> Self {
        Self
    }
}

impl<const INT_BITS: u32, const FRAC_BITS: u32> FixedPointAxis
    for FixedPointQNumeric<INT_BITS, FRAC_BITS>
{
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/FixedPointAxis/Q";
    const MAX_OUTPUT_BYTES: usize = WIDTH;

    fn add(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if INT_BITS + FRAC_BITS == 0 || INT_BITS + FRAC_BITS > 64 {
            return Err(format_violation());
        }
        let (a, b) = split_pair(input, WIDTH)?;
        check_output(out, WIDTH)?;
        let result = decode(a).saturating_add(decode(b));
        out[..WIDTH].copy_from_slice(&encode(result));
        Ok(WIDTH)
    }

    fn sub(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if INT_BITS + FRAC_BITS == 0 || INT_BITS + FRAC_BITS > 64 {
            return Err(format_violation());
        }
        let (a, b) = split_pair(input, WIDTH)?;
        check_output(out, WIDTH)?;
        let result = decode(a).saturating_sub(decode(b));
        out[..WIDTH].copy_from_slice(&encode(result));
        Ok(WIDTH)
    }

    fn mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if INT_BITS + FRAC_BITS == 0 || INT_BITS + FRAC_BITS > 64 {
            return Err(format_violation());
        }
        let (a, b) = split_pair(input, WIDTH)?;
        check_output(out, WIDTH)?;
        let product = i128::from(decode(a)) * i128::from(decode(b));
        let rescaled = product >> FRAC_BITS;
        let saturated: i64 = if rescaled > i128::from(i64::MAX) {
            i64::MAX
        } else if rescaled < i128::from(i64::MIN) {
            i64::MIN
        } else {
            #[allow(clippy::cast_possible_truncation)]
            {
                rescaled as i64
            }
        };
        out[..WIDTH].copy_from_slice(&encode(saturated));
        Ok(WIDTH)
    }
}

// ADR-052 generic-form companion: parametric impl inherits the
// dispatch body from the `axis!` emission.
axis_extension_impl_for_fixed_point_axis!(
    @generic FixedPointQNumeric<INT_BITS, FRAC_BITS>,
    [const INT_BITS: u32, const FRAC_BITS: u32]
);

/// Q32.32 — 32 integer bits, 32 fraction bits.
pub type FixedPointQ32_32Numeric = FixedPointQNumeric<32, 32>;
/// Q16.16 — DSP / graphics canonical split.
pub type FixedPointQ16_16Numeric = FixedPointQNumeric<16, 16>;
/// Q1.31 — high-precision fraction-heavy split (financial / signal).
pub type FixedPointQ1_31Numeric = FixedPointQNumeric<1, 31>;
/// Q48.16 — large-magnitude integer with sub-integer precision.
pub type FixedPointQ48_16Numeric = FixedPointQNumeric<48, 16>;

// ---- FixedPointShape: ConstrainedTypeShape carrier ----

/// Parametric ConstrainedTypeShape carrying an 8-byte Q-format value.
///
/// `INT_BITS + FRAC_BITS = 64` is the canonical full-container case;
/// other splits within `≤ 64` are admissible. The shape's identity
/// flows through `(SITE_COUNT, CONSTRAINTS)` per ADR-017's closure
/// rule — distinct `(I, F)` instantiations content-address identically
/// when their site counts coincide.
#[derive(Debug, Clone, Copy)]
pub struct FixedPointShape<const INT_BITS: u32, const FRAC_BITS: u32>;

impl<const I: u32, const F: u32> Default for FixedPointShape<I, F> {
    fn default() -> Self {
        Self
    }
}

impl<const INT_BITS: u32, const FRAC_BITS: u32> ConstrainedTypeShape
    for FixedPointShape<INT_BITS, FRAC_BITS>
{
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = WIDTH;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(WIDTH as u32);
}

impl<const INT_BITS: u32, const FRAC_BITS: u32> uor_foundation::pipeline::__sdk_seal::Sealed
    for FixedPointShape<INT_BITS, FRAC_BITS>
{
}
impl<const INT_BITS: u32, const FRAC_BITS: u32> GroundedShape
    for FixedPointShape<INT_BITS, FRAC_BITS>
{
}
impl<'a, const INT_BITS: u32, const FRAC_BITS: u32> IntoBindingValue<'a>
    for FixedPointShape<INT_BITS, FRAC_BITS>
{
    fn as_binding_value<const INLINE_BYTES: usize>(&self) -> TermValue<'a, INLINE_BYTES> {
        TermValue::empty()
    }
}
