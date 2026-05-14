//! `RingAxis` declaration and GF(2)-over-256-bit reference impl.

#![allow(missing_docs)]

use uor_foundation::enforcement::ShapeViolation;
use uor_foundation::pipeline::AxisExtension;
use uor_foundation_sdk::axis;

use crate::{check_output, split_pair};

axis! {
    /// Wiki ADR-031 finite-ring arithmetic axis.
    ///
    /// The reference impl `Gf2NumericAxis` fixes the ring at GF(2) over
    /// 256-bit operands — addition is XOR, multiplication is AND
    /// (each bit treated as an independent GF(2) element).
    pub trait RingAxis: AxisExtension {
        /// ADR-017 content address.
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/RingAxis";
        /// Operand byte-width.
        const MAX_OUTPUT_BYTES: usize = 32;
        /// Ring addition. Input `a || b` (64 bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn add(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
        /// Ring multiplication. Input `a || b` (64 bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

const WIDTH: usize = 32;

/// GF(2) arithmetic over 256-bit operands — bitwise XOR / AND.
#[derive(Debug, Clone, Copy, Default)]
pub struct Gf2NumericAxis;

impl RingAxis for Gf2NumericAxis {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/RingAxis/Gf2_256";
    const MAX_OUTPUT_BYTES: usize = WIDTH;

    fn add(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        let (a, b) = split_pair(input, WIDTH)?;
        check_output(out, WIDTH)?;
        for i in 0..WIDTH {
            out[i] = a[i] ^ b[i];
        }
        Ok(WIDTH)
    }

    fn mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        let (a, b) = split_pair(input, WIDTH)?;
        check_output(out, WIDTH)?;
        for i in 0..WIDTH {
            out[i] = a[i] & b[i];
        }
        Ok(WIDTH)
    }
}

axis_extension_impl_for_ring_axis!(Gf2NumericAxis);
