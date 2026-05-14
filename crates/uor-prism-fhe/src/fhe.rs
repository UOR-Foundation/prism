//! `FheAxis` declaration and one-time-pad reference impl.

#![allow(missing_docs)]

use uor_foundation::enforcement::ShapeViolation;
use uor_foundation::pipeline::AxisExtension;
use uor_foundation_sdk::axis;

axis! {
    /// Wiki ADR-031 homomorphic-encryption axis.
    ///
    /// Reference kernel `add_ciphertexts` is the additive operation
    /// over a fixed 32-byte block; the scheme's correctness predicate
    /// is `Dec(Enc(a) ⊕ Enc(b)) = a + b` (XOR for the one-time-pad
    /// reference impl, real homomorphism for production FHE schemes).
    pub trait FheAxis: AxisExtension {
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/FheAxis";
        const MAX_OUTPUT_BYTES: usize = 32;
        /// Homomorphic addition of two ciphertext blocks.
        /// Input = `c_a || c_b` (64 bytes); output = `c_a ⊕_FHE c_b`.
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on malformed ciphertext encoding.
        fn add_ciphertexts(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

const BLOCK_BYTES: usize = 32;

fn shape_violation(constraint: &'static str) -> ShapeViolation {
    ShapeViolation {
        shape_iri: "https://uor.foundation/axis/FheAxis",
        constraint_iri: constraint,
        property_iri: "https://uor.foundation/axis/inputBytes",
        expected_range: "https://uor.foundation/axis/FheBlockShape",
        min_count: 0,
        max_count: 0,
        kind: uor_foundation::ViolationKind::ValueCheck,
    }
}

/// Reference one-time-pad "FHE" — additive over ciphertexts under XOR.
/// Suitable for conformance testing the axis dispatch path; not a
/// cryptographic FHE scheme.
#[derive(Debug, Clone, Copy, Default)]
pub struct OneTimePadFheAxis;

impl FheAxis for OneTimePadFheAxis {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/FheAxis/OneTimePadReference";
    const MAX_OUTPUT_BYTES: usize = BLOCK_BYTES;

    fn add_ciphertexts(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if input.len() != 2 * BLOCK_BYTES {
            return Err(shape_violation(
                "https://uor.foundation/axis/FheAxis/inputBlockPair",
            ));
        }
        if out.len() < BLOCK_BYTES {
            return Err(shape_violation(
                "https://uor.foundation/axis/FheAxis/outputBlock",
            ));
        }
        for i in 0..BLOCK_BYTES {
            out[i] = input[i] ^ input[BLOCK_BYTES + i];
        }
        Ok(BLOCK_BYTES)
    }
}

axis_extension_impl_for_fhe_axis!(OneTimePadFheAxis);
