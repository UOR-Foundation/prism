//! `RingAxis` declaration + parametric GF(2)-over-N-bytes impl + shape.
//!
//! Per [Wiki ADR-031][09-adr-031] the numerics sub-crate exposes
//! `RingAxis` as the canonical Layer-3 surface for finite-ring
//! arithmetic. The reference impl [`Gf2NumericAxisN`] is generic over
//! byte-width: addition is bitwise XOR, multiplication is bitwise AND
//! (each bit treated as an independent GF(2) element).
//!
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(missing_docs)]

use uor_foundation::enforcement::{GroundedShape, ShapeViolation};
use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef, IntoBindingValue, TermValue};
use uor_foundation_sdk::axis;

use crate::{check_output, split_pair};

axis! {
    /// Wiki ADR-031 finite-ring arithmetic axis.
    ///
    /// Addition and multiplication mod a fixed finite ring. The
    /// reference impl `Gf2NumericAxisN<BYTES>` is GF(2) per byte
    /// (bitwise XOR / AND).
    pub trait RingAxis: AxisExtension {
        /// ADR-017 content address.
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/RingAxis";
        /// Operand byte-width (overridden per impl).
        const MAX_OUTPUT_BYTES: usize = 32;
        /// Ring addition. Input `a || b` (`2N` bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn add(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
        /// Ring multiplication. Input `a || b` (`2N` bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

/// `Gf2NumericAxisN<BYTES>` admits **any** operand byte-width `BYTES ≥ 1`:
/// the GF(2) bitwise kernels (XOR / AND) write directly into the caller's
/// `out` buffer with no fixed-width scratch, so there is no upper ceiling
/// on the width — the operand scales arbitrarily (§ 11.10 category 3).
/// The only floor is non-emptiness: a ring element needs at least one byte.
fn width_violation() -> ShapeViolation {
    ShapeViolation {
        shape_iri: "https://uor.foundation/axis/RingAxis",
        constraint_iri: "https://uor.foundation/axis/RingAxis/widthPositive",
        property_iri: "https://uor.foundation/axis/operandByteWidth",
        expected_range: "https://uor.foundation/axis/RingAxis/PositiveByteWidth",
        min_count: 1,
        max_count: u32::MAX,
        kind: uor_foundation::ViolationKind::ValueCheck,
    }
}

/// GF(2) arithmetic over `N`-byte operands — bitwise XOR / AND.
///
/// Each byte position is independently a GF(2)-element under bitwise
/// XOR (addition) and AND (multiplication). Per-byte
/// distributivity / commutativity / GF(2) field properties hold.
#[derive(Debug, Clone, Copy)]
pub struct Gf2NumericAxisN<const BYTES: usize>;

impl<const BYTES: usize> Default for Gf2NumericAxisN<BYTES> {
    fn default() -> Self {
        Self
    }
}

impl<const BYTES: usize> RingAxis for Gf2NumericAxisN<BYTES> {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/RingAxis/Gf2";
    const MAX_OUTPUT_BYTES: usize = BYTES;

    fn add(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if BYTES == 0 {
            return Err(width_violation());
        }
        let (a, b) = split_pair(input, BYTES)?;
        check_output(out, BYTES)?;
        for i in 0..BYTES {
            out[i] = a[i] ^ b[i];
        }
        Ok(BYTES)
    }

    fn mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if BYTES == 0 {
            return Err(width_violation());
        }
        let (a, b) = split_pair(input, BYTES)?;
        check_output(out, BYTES)?;
        for i in 0..BYTES {
            out[i] = a[i] & b[i];
        }
        Ok(BYTES)
    }
}

// ADR-052 generic-form companion.
axis_extension_impl_for_ring_axis!(@generic Gf2NumericAxisN<BYTES>, [const BYTES: usize]);

/// 256-bit GF(2) ring (canonical 32-byte width).
pub type Gf2NumericAxis = Gf2NumericAxisN<32>;
/// 128-bit GF(2) ring.
pub type Gf2NumericAxis128 = Gf2NumericAxisN<16>;
/// 512-bit GF(2) ring.
pub type Gf2NumericAxis512 = Gf2NumericAxisN<64>;

// ---- Gf2RingShape: ConstrainedTypeShape carrier ----

/// Parametric ConstrainedTypeShape for an `N`-byte GF(2) ring element.
#[derive(Debug, Clone, Copy)]
pub struct Gf2RingShape<const BYTES: usize>;

impl<const BYTES: usize> Default for Gf2RingShape<BYTES> {
    fn default() -> Self {
        Self
    }
}

impl<const BYTES: usize> ConstrainedTypeShape for Gf2RingShape<BYTES> {
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(BYTES as u32);
}

impl<const BYTES: usize> uor_foundation::pipeline::__sdk_seal::Sealed for Gf2RingShape<BYTES> {}
impl<const BYTES: usize> GroundedShape for Gf2RingShape<BYTES> {}
impl<'a, const BYTES: usize> IntoBindingValue<'a> for Gf2RingShape<BYTES> {
    fn as_binding_value<const INLINE_BYTES: usize>(&self) -> TermValue<'a, INLINE_BYTES> {
        TermValue::empty()
    }
}
