//! `ActivationAxis` declaration and 16-element i8 ReLU + Q1.7 sigmoid impl.

#![allow(missing_docs)]

use uor_foundation::enforcement::ShapeViolation;
use uor_foundation::pipeline::AxisExtension;
use uor_foundation_sdk::axis;

axis! {
    /// Wiki ADR-031 element-wise nonlinearity axis.
    ///
    /// Reference kernels operate on a fixed-length 16-element `i8`
    /// vector. `relu` clamps negative values to zero. `sigmoid_q` is
    /// the Q1.7 piecewise-linear sigmoid approximation (the canonical
    /// integer-arithmetic determinism contract).
    pub trait ActivationAxis: AxisExtension {
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/ActivationAxis";
        const MAX_OUTPUT_BYTES: usize = 16;
        /// Apply ReLU elementwise. Input = 16 bytes.
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output length mismatch.
        fn relu(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
        /// Apply Q1.7 piecewise-linear sigmoid. Input = 16 bytes.
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output length mismatch.
        fn sigmoid_q(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

const VEC_BYTES: usize = 16;

fn arity_violation(constraint: &'static str) -> ShapeViolation {
    ShapeViolation {
        shape_iri: "https://uor.foundation/axis/ActivationAxisShape",
        constraint_iri: constraint,
        property_iri: "https://uor.foundation/axis/inputBytes",
        expected_range: "https://uor.foundation/axis/ActivationInputArity",
        min_count: 0,
        max_count: 0,
        kind: uor_foundation::ViolationKind::ValueCheck,
    }
}

fn check_lens(input: &[u8], out: &[u8]) -> Result<(), ShapeViolation> {
    if input.len() != VEC_BYTES {
        return Err(arity_violation(
            "https://uor.foundation/axis/ActivationAxisShape/inputByteLength",
        ));
    }
    if out.len() < VEC_BYTES {
        return Err(arity_violation(
            "https://uor.foundation/axis/ActivationAxisShape/outputByteLength",
        ));
    }
    Ok(())
}

/// Element-wise activation kernels over a 16-element `i8` vector.
#[derive(Debug, Clone, Copy, Default)]
pub struct CpuI8VectorActivation16;

impl ActivationAxis for CpuI8VectorActivation16 {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/ActivationAxis/CpuI8Vec16";
    const MAX_OUTPUT_BYTES: usize = VEC_BYTES;

    fn relu(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        check_lens(input, out)?;
        for i in 0..VEC_BYTES {
            #[allow(clippy::cast_possible_wrap)]
            let v = input[i] as i8;
            out[i] = if v > 0 { input[i] } else { 0 };
        }
        Ok(VEC_BYTES)
    }

    fn sigmoid_q(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        check_lens(input, out)?;
        for i in 0..VEC_BYTES {
            #[allow(clippy::cast_possible_wrap)]
            let x = input[i] as i8;
            let y: i8 = if x <= -64 {
                0
            } else if x >= 64 {
                127
            } else {
                #[allow(clippy::cast_possible_truncation)]
                {
                    64i8 + (x / 2)
                }
            };
            #[allow(clippy::cast_sign_loss)]
            {
                out[i] = y as u8;
            }
        }
        Ok(VEC_BYTES)
    }
}

axis_extension_impl_for_activation_axis!(CpuI8VectorActivation16);
