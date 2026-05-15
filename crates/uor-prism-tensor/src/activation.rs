//! `ActivationAxis` declaration + parametric element-wise i8 nonlinearity
//! reference impls.

#![allow(missing_docs)]

use uor_foundation::enforcement::ShapeViolation;
use uor_foundation::pipeline::AxisExtension;
use uor_foundation_sdk::axis;

axis! {
    /// Wiki ADR-031 element-wise nonlinearity axis.
    ///
    /// Reference kernels operate on a fixed-length `N`-element `i8`
    /// vector. `relu` clamps negative values to zero. `sigmoid_q` is
    /// the Q1.7 piecewise-linear sigmoid approximation — the canonical
    /// integer-arithmetic determinism contract per ADR-030.
    pub trait ActivationAxis: AxisExtension {
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/ActivationAxis";
        /// Vector byte-width (overridden per impl).
        const MAX_OUTPUT_BYTES: usize = 16;
        /// Apply ReLU elementwise.
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output length mismatch.
        fn relu(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
        /// Apply Q1.7 piecewise-linear sigmoid elementwise.
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output length mismatch.
        fn sigmoid_q(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

/// Maximum vector length any [`CpuI8VectorActivation`] instantiation
/// supports.
pub const MAX_ACTIVATION_LEN: usize = 256;

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

fn check_lens(input: &[u8], out: &[u8], n: usize) -> Result<(), ShapeViolation> {
    if input.len() != n {
        return Err(arity_violation(
            "https://uor.foundation/axis/ActivationAxisShape/inputByteLength",
        ));
    }
    if out.len() < n {
        return Err(arity_violation(
            "https://uor.foundation/axis/ActivationAxisShape/outputByteLength",
        ));
    }
    Ok(())
}

/// Parametric element-wise activation kernels over an `N`-element `i8`
/// vector.
///
/// `N` is the vector length. The same kernels (ReLU, Q1.7 sigmoid) are
/// applied to every element independently; per-element determinism
/// composes to per-vector determinism per ADR-030.
#[derive(Debug, Clone, Copy)]
pub struct CpuI8VectorActivation<const N: usize>;

impl<const N: usize> Default for CpuI8VectorActivation<N> {
    fn default() -> Self {
        Self
    }
}

impl<const N: usize> ActivationAxis for CpuI8VectorActivation<N> {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/ActivationAxis/CpuI8Vector";
    const MAX_OUTPUT_BYTES: usize = N;

    fn relu(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if N == 0 || N > MAX_ACTIVATION_LEN {
            return Err(arity_violation(
                "https://uor.foundation/axis/ActivationAxisShape/nInRange",
            ));
        }
        check_lens(input, out, N)?;
        for i in 0..N {
            #[allow(clippy::cast_possible_wrap)]
            let v = input[i] as i8;
            out[i] = if v > 0 { input[i] } else { 0 };
        }
        Ok(N)
    }

    fn sigmoid_q(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if N == 0 || N > MAX_ACTIVATION_LEN {
            return Err(arity_violation(
                "https://uor.foundation/axis/ActivationAxisShape/nInRange",
            ));
        }
        check_lens(input, out, N)?;
        for i in 0..N {
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
        Ok(N)
    }
}

// ADR-052 generic-form companion.
axis_extension_impl_for_activation_axis!(@generic CpuI8VectorActivation<N>, [const N: usize]);

/// 16-element `i8` vector activation (the canonical small-vector reference).
pub type CpuI8VectorActivation16 = CpuI8VectorActivation<16>;
/// 32-element `i8` vector activation.
pub type CpuI8VectorActivation32 = CpuI8VectorActivation<32>;
/// 64-element `i8` vector activation.
pub type CpuI8VectorActivation64 = CpuI8VectorActivation<64>;
/// 128-element `i8` vector activation.
pub type CpuI8VectorActivation128 = CpuI8VectorActivation<128>;
/// 256-element `i8` vector activation (the `MAX_ACTIVATION_LEN` ceiling).
pub type CpuI8VectorActivation256 = CpuI8VectorActivation<256>;
