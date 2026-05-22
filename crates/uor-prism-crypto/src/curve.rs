//! `CurveAxis` declaration (wiki ADR-031 elliptic-curve operations).
//!
//! Concrete impls (Secp256k1, Ed25519Curve, Bls12_381, BN254) are
//! operational policy per ADR-031 and live in application crates or
//! follow-on standard-library sub-crate revisions. This module
//! declares the axis trait so application authors can reach it through
//! `prism::crypto::CurveAxis` per the wiki's "use prism::*" promise.

#![allow(missing_docs)]

use uor_foundation::enforcement::ShapeViolation;
use uor_foundation_sdk::axis;

axis! {
    /// Wiki ADR-031 elliptic-curve operations axis.
    pub trait CurveAxis: AxisExtension {
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/CurveAxis";
        const MAX_OUTPUT_BYTES: usize = 96;
        /// Scalar multiplication `k · P` where `input = k || P`.
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on malformed scalar/point encoding.
        fn scalar_mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
        /// Point addition `P + Q` where `input = P || Q`.
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on malformed point encoding.
        fn point_add(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}
