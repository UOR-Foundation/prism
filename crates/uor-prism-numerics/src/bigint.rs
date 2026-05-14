//! `BigIntAxis` declaration and 256-bit modular arithmetic impl.

#![allow(missing_docs)]

use uor_foundation::enforcement::ShapeViolation;
use uor_foundation::pipeline::AxisExtension;
use uor_foundation_sdk::axis;

use crate::{check_output, split_pair};

axis! {
    /// Wiki ADR-031 fixed-width integer arithmetic axis.
    ///
    /// Each kernel takes input `a || b` (big-endian-encoded operands
    /// of equal width) and writes the result into `out`. The reference
    /// impl `BigInt256Numeric` fixes the operand width at 32 bytes
    /// (256 bits) and computes modular arithmetic mod `2^256`.
    pub trait BigIntAxis: AxisExtension {
        /// ADR-017 content address.
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/BigIntAxis";
        /// Maximum operand byte-width (32 bytes = 256 bits).
        const MAX_OUTPUT_BYTES: usize = 32;
        /// `(a + b) mod 2^256` — input is `a || b` (64 bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn add(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
        /// `(a - b) mod 2^256` — input is `a || b` (64 bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn sub(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
        /// `(a * b) mod 2^256` — input is `a || b` (64 bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

const WIDTH: usize = 32;

/// 256-bit big-endian unsigned integer modular arithmetic.
#[derive(Debug, Clone, Copy, Default)]
pub struct BigInt256Numeric;

impl BigIntAxis for BigInt256Numeric {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/BigIntAxis/Mod256";
    const MAX_OUTPUT_BYTES: usize = WIDTH;

    fn add(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        let (a, b) = split_pair(input, WIDTH)?;
        check_output(out, WIDTH)?;
        let mut carry: u16 = 0;
        for i in (0..WIDTH).rev() {
            let sum = u16::from(a[i]) + u16::from(b[i]) + carry;
            #[allow(clippy::cast_possible_truncation)]
            {
                out[i] = (sum & 0xff) as u8;
            }
            carry = sum >> 8;
        }
        Ok(WIDTH)
    }

    fn sub(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        let (a, b) = split_pair(input, WIDTH)?;
        check_output(out, WIDTH)?;
        let mut borrow: i16 = 0;
        for i in (0..WIDTH).rev() {
            let diff = i16::from(a[i]) - i16::from(b[i]) - borrow;
            if diff < 0 {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                {
                    out[i] = (diff + 256) as u8;
                }
                borrow = 1;
            } else {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                {
                    out[i] = diff as u8;
                }
                borrow = 0;
            }
        }
        Ok(WIDTH)
    }

    fn mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        let (a, b) = split_pair(input, WIDTH)?;
        check_output(out, WIDTH)?;
        let mut acc = [0u32; 2 * WIDTH];
        for i in (0..WIDTH).rev() {
            for j in (0..WIDTH).rev() {
                let prod = u32::from(a[i]) * u32::from(b[j]);
                let pos = i + j + 1;
                let sum = acc[pos] + (prod & 0xff);
                acc[pos] = sum & 0xff;
                let mut carry = (sum >> 8) + (prod >> 8);
                let mut k = pos;
                while carry > 0 && k > 0 {
                    k -= 1;
                    let next = acc[k] + carry;
                    acc[k] = next & 0xff;
                    carry = next >> 8;
                }
            }
        }
        for i in 0..WIDTH {
            #[allow(clippy::cast_possible_truncation)]
            {
                out[i] = (acc[i + WIDTH] & 0xff) as u8;
            }
        }
        Ok(WIDTH)
    }
}

axis_extension_impl_for_big_int_axis!(BigInt256Numeric);
