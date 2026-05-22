//! `SignatureAxis` declaration (wiki ADR-031 signing/verification).
//!
//! Concrete impls are operational policy per ADR-031 and live in
//! application crates. This module declares the axis trait so
//! application authors can reach it through `prism::crypto::SignatureAxis`.

#![allow(missing_docs)]

use uor_foundation::enforcement::ShapeViolation;
use uor_foundation_sdk::axis;

axis! {
    /// Wiki ADR-031 signing / verification axis.
    pub trait SignatureAxis: AxisExtension {
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/SignatureAxis";
        const MAX_OUTPUT_BYTES: usize = 96;
        /// Verify a signature against `input = pubkey || msg || sig`,
        /// emitting `[0x01]` for accept or `[0x00]` for reject in `out[0]`.
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on malformed encoding.
        fn verify(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}
