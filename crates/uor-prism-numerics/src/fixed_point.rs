//! `FixedPointAxis` declaration and Q32.32 reference impl.

#![allow(missing_docs)]

use uor_foundation::enforcement::ShapeViolation;
use uor_foundation::pipeline::AxisExtension;
use uor_foundation_sdk::axis;

use crate::{check_output, split_pair};

axis! {
    /// Wiki ADR-031 fixed-point arithmetic axis.
    ///
    /// Operates on Q-format two's-complement integers. The reference
    /// impl `FixedPointQ32_32Numeric` fixes the format at Q32.32
    /// (64-bit total width, 32 integer bits + 32 fraction bits).
    pub trait FixedPointAxis: AxisExtension {
        /// ADR-017 content address.
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/FixedPointAxis";
        /// Operand byte-width.
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
        /// Q-format multiplication with bias-aware re-scaling.
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

const WIDTH: usize = 8;
const FRACTION_BITS: u32 = 32;

fn decode(slice: &[u8]) -> i64 {
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&slice[..8]);
    i64::from_be_bytes(buf)
}

fn encode(value: i64) -> [u8; 8] {
    value.to_be_bytes()
}

/// Q32.32 fixed-point arithmetic (signed two's complement).
#[derive(Debug, Clone, Copy, Default)]
pub struct FixedPointQ32_32Numeric;

impl FixedPointAxis for FixedPointQ32_32Numeric {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/FixedPointAxis/Q32_32";
    const MAX_OUTPUT_BYTES: usize = WIDTH;

    fn add(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        let (a, b) = split_pair(input, WIDTH)?;
        check_output(out, WIDTH)?;
        let result = decode(a).saturating_add(decode(b));
        out[..WIDTH].copy_from_slice(&encode(result));
        Ok(WIDTH)
    }

    fn sub(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        let (a, b) = split_pair(input, WIDTH)?;
        check_output(out, WIDTH)?;
        let result = decode(a).saturating_sub(decode(b));
        out[..WIDTH].copy_from_slice(&encode(result));
        Ok(WIDTH)
    }

    fn mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        let (a, b) = split_pair(input, WIDTH)?;
        check_output(out, WIDTH)?;
        let product = i128::from(decode(a)) * i128::from(decode(b));
        let rescaled = product >> FRACTION_BITS;
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

axis_extension_impl_for_fixed_point_axis!(FixedPointQ32_32Numeric);
