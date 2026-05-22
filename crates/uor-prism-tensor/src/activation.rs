//! `ActivationAxis` declaration + parametric element-wise i8 nonlinearity
//! reference impls.

#![allow(missing_docs)]

use uor_foundation::enforcement::ShapeViolation;
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
        /// Per-impl structural output-byte hint. Per ADR-060 the
        /// foundation derives carrier widths from the application's
        /// `HostBounds` structural-count primitives; the axis impl
        /// carries no substrate-arbitrary byte-width cap.
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
///
/// # `HostBounds` discipline
///
/// `N` is unconstrained at the axis level. Per [Wiki ADR-060][09] the
/// foundation removed the `AXIS_OUTPUT_BYTES_MAX` cap: a length-`N`
/// kernel's output flows through the source-polymorphic `TermValue`
/// carrier, whose widths derive from the application's
/// [`HostBounds`][uor_foundation::HostBounds] structural-count
/// primitives via foundation `const fn`s — never a pinned byte-width
/// literal. Specific `N` values (16, 32, 64, 128, 256, …) are picked
/// by the application; this crate imposes no ceiling.
///
/// [09]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
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
        // Structural well-formedness only — a zero-length vector is
        // not a vector. Per ADR-060 there is no byte-width cap; the
        // output flows through the source-polymorphic `TermValue`
        // carrier sized from the application's `HostBounds` primitives.
        if N == 0 {
            return Err(arity_violation(
                "https://uor.foundation/axis/ActivationAxisShape/nNonZero",
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
        if N == 0 {
            return Err(arity_violation(
                "https://uor.foundation/axis/ActivationAxisShape/nNonZero",
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
