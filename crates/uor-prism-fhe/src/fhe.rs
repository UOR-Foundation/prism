//! `FheAxis` declaration + parametric one-time-pad reference impl +
//! shape carrier.

#![allow(missing_docs)]

use uor_foundation::enforcement::{GroundedShape, ShapeViolation};
use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef, IntoBindingValue, TermValue};
use uor_foundation_sdk::axis;

axis! {
    /// Wiki ADR-031 homomorphic-encryption axis.
    ///
    /// Reference kernel `add_ciphertexts` is the additive operation
    /// over a fixed `BLOCK_BYTES`-byte ciphertext block; the scheme's
    /// correctness predicate is `Dec(Enc(a) ⊕ Enc(b)) = a + b` (XOR
    /// for the one-time-pad reference impl, real homomorphism for
    /// production FHE schemes).
    pub trait FheAxis: AxisExtension {
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/FheAxis";
        /// Ciphertext block width (overridden per impl).
        const MAX_OUTPUT_BYTES: usize = 32;
        /// Homomorphic addition of two ciphertext blocks.
        /// Input = `c_a || c_b` (`2 * BLOCK_BYTES`); output = `c_a ⊕_FHE c_b`.
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on malformed ciphertext encoding.
        fn add_ciphertexts(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

/// Maximum ciphertext block width any [`OneTimePadFhe`] instantiation
/// supports. The XOR kernel is byte-loop, so the cap is more about
/// admission-error metadata cohesion than implementation cost.
pub const MAX_FHE_BLOCK_BYTES: usize = 256;

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

/// Parametric one-time-pad "FHE" — additive over ciphertexts under XOR.
///
/// Reference impl suitable for conformance testing the axis dispatch
/// path; not a cryptographic FHE scheme. `BLOCK_BYTES` is the
/// ciphertext block width. Production FHE schemes (TFHE, BGV, CKKS
/// per ADR-031's roster) are application-level integrations that
/// satisfy the same `FheAxis` contract with cryptographically secure
/// schemes.
#[derive(Debug, Clone, Copy)]
pub struct OneTimePadFhe<const BLOCK_BYTES: usize>;

impl<const BLOCK_BYTES: usize> Default for OneTimePadFhe<BLOCK_BYTES> {
    fn default() -> Self {
        Self
    }
}

impl<const BLOCK_BYTES: usize> FheAxis for OneTimePadFhe<BLOCK_BYTES> {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/FheAxis/OneTimePadReference";
    const MAX_OUTPUT_BYTES: usize = BLOCK_BYTES;

    fn add_ciphertexts(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if BLOCK_BYTES == 0 || BLOCK_BYTES > MAX_FHE_BLOCK_BYTES {
            return Err(shape_violation(
                "https://uor.foundation/axis/FheAxis/blockBytesInRange",
            ));
        }
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

// ADR-052 generic-form companion.
axis_extension_impl_for_fhe_axis!(@generic OneTimePadFhe<BLOCK_BYTES>, [const BLOCK_BYTES: usize]);

/// 32-byte one-time-pad FHE (canonical block width).
pub type OneTimePadFheAxis = OneTimePadFhe<32>;
/// 16-byte one-time-pad FHE.
pub type OneTimePadFhe16 = OneTimePadFhe<16>;
/// 64-byte one-time-pad FHE.
pub type OneTimePadFhe64 = OneTimePadFhe<64>;
/// 128-byte one-time-pad FHE.
pub type OneTimePadFhe128 = OneTimePadFhe<128>;

// ---- CiphertextShape: ConstrainedTypeShape carrier ----

/// Parametric ConstrainedTypeShape for an `N`-byte ciphertext block.
///
/// Per ADR-031's `Ciphertext<Plaintext, Scheme>` shape commitment —
/// reduced to byte-width parametricity here since the
/// scheme-and-plaintext type-level pair would require richer
/// const-generic machinery. The byte width carries the structural
/// commitment; downstream schemes wrap this shape in a newtype
/// associating the plaintext type's IRI per ADR-017.
#[derive(Debug, Clone, Copy)]
pub struct CiphertextShape<const BYTES: usize>;

impl<const BYTES: usize> Default for CiphertextShape<BYTES> {
    fn default() -> Self {
        Self
    }
}

impl<const BYTES: usize> ConstrainedTypeShape for CiphertextShape<BYTES> {
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(BYTES as u32);
}

impl<const BYTES: usize> uor_foundation::pipeline::__sdk_seal::Sealed for CiphertextShape<BYTES> {}
impl<const BYTES: usize> GroundedShape for CiphertextShape<BYTES> {}
impl<'a, const BYTES: usize> IntoBindingValue<'a> for CiphertextShape<BYTES> {
    fn as_binding_value<const INLINE_BYTES: usize>(&self) -> TermValue<'a, INLINE_BYTES> {
        TermValue::empty()
    }
}
