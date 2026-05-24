//! `BigIntAxis` declaration + parametric modular-arithmetic impls + shape.
//!
//! Per [Wiki ADR-031][09-adr-031] the numerics sub-crate exposes
//! `BigIntAxis` as the canonical Layer-3 vocabulary for fixed-width
//! integer arithmetic. The reference impl [`BigIntModularNumeric`] is
//! generic over operand byte-width per ADR-031's `BigInt<MaxBits>`
//! shape commitment — every `BYTES ≥ 1` instantiation is a distinct
//! sealed `AxisExtension` that the application's `AxisTuple` can select.
//! The kernels carry no fixed-width scratch, so the operand width scales
//! arbitrarily with no ceiling (§ 11.10).
//!
//! [`BigIntShape`] is the matching `ConstrainedTypeShape` so
//! application authors can declare `BigInt<N>`-typed inputs and outputs
//! to their `prism_model!` invocations without re-rolling the shape.
//!
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(missing_docs)]

use uor_foundation::enforcement::{GroundedShape, ShapeViolation};
use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef, IntoBindingValue, TermValue};
use uor_foundation_sdk::axis;

use crate::{check_output, split_pair};

axis! {
    /// Wiki ADR-031 fixed-width integer arithmetic axis.
    ///
    /// Kernels take input `a || b` (big-endian-encoded equal-width
    /// operands) and emit modular arithmetic results. The reference
    /// impl `BigIntModularNumeric<BYTES>` is generic in `BYTES` for
    /// any `BYTES ≥ 1`.
    pub trait BigIntAxis: AxisExtension {
        /// ADR-017 content address.
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/BigIntAxis";
        /// Operand byte-width (overridden per impl).
        const MAX_OUTPUT_BYTES: usize = 32;
        /// `(a + b) mod 2^(8*N)` — input is `a || b` (`2N` bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn add(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
        /// `(a - b) mod 2^(8*N)` — input is `a || b` (`2N` bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn sub(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
        /// `(a * b) mod 2^(8*N)` — input is `a || b` (`2N` bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

/// `BigIntModularNumeric<BYTES>` admits **any** operand byte-width
/// `BYTES ≥ 1`: add/sub stream carries directly into `out`, and `mul`
/// computes the modular product column-by-column with a single running
/// `u64` carry (`O(1)` scratch), so there is no fixed-width accumulator
/// and therefore no upper ceiling on the width (§ 11.10). The only floor
/// is non-emptiness: a fixed-width integer needs at least one byte.
fn width_violation() -> ShapeViolation {
    ShapeViolation {
        shape_iri: "https://uor.foundation/axis/BigIntAxis",
        constraint_iri: "https://uor.foundation/axis/BigIntAxis/widthPositive",
        property_iri: "https://uor.foundation/axis/operandByteWidth",
        expected_range: "https://uor.foundation/axis/BigIntAxis/PositiveByteWidth",
        min_count: 1,
        max_count: u32::MAX,
        kind: uor_foundation::ViolationKind::ValueCheck,
    }
}

/// Parametric `N`-byte modular-arithmetic impl of [`BigIntAxis`].
///
/// `BYTES` is the operand width in bytes (`8 * BYTES` bits). Arithmetic
/// is mod `2^(8 * BYTES)` (wrapping). Any `BYTES ≥ 1` is supported — the
/// kernels carry no fixed-width scratch, so the width has no ceiling.
#[derive(Debug, Clone, Copy)]
pub struct BigIntModularNumeric<const BYTES: usize>;

impl<const BYTES: usize> Default for BigIntModularNumeric<BYTES> {
    fn default() -> Self {
        Self
    }
}

impl<const BYTES: usize> BigIntAxis for BigIntModularNumeric<BYTES> {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/BigIntAxis/Modular";
    const MAX_OUTPUT_BYTES: usize = BYTES;

    fn add(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if BYTES == 0 {
            return Err(width_violation());
        }
        let (a, b) = split_pair(input, BYTES)?;
        check_output(out, BYTES)?;
        let mut carry: u16 = 0;
        for i in (0..BYTES).rev() {
            let sum = u16::from(a[i]) + u16::from(b[i]) + carry;
            #[allow(clippy::cast_possible_truncation)]
            {
                out[i] = (sum & 0xff) as u8;
            }
            carry = sum >> 8;
        }
        Ok(BYTES)
    }

    fn sub(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if BYTES == 0 {
            return Err(width_violation());
        }
        let (a, b) = split_pair(input, BYTES)?;
        check_output(out, BYTES)?;
        let mut borrow: i16 = 0;
        for i in (0..BYTES).rev() {
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
        Ok(BYTES)
    }

    fn mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        if BYTES == 0 {
            return Err(width_violation());
        }
        let (a, b) = split_pair(input, BYTES)?;
        check_output(out, BYTES)?;
        // Modular product mod 2^(8*BYTES): only the low `BYTES` bytes of
        // the schoolbook product survive, so we compute them column by
        // column from least-significant upward, carrying forward in a
        // single running `u64`. This needs `O(1)` scratch — no fixed-width
        // accumulator — so the operand width has no ceiling. Operands are
        // big-endian; little-endian position `p` is byte `BYTES - 1 - p`.
        // For `p < BYTES` every (x, p - x) index pair is in range, so the
        // inner sum runs `x` from `0..=p`. The column sum is bounded by
        // `BYTES * 255^2 + carry`, which stays well within `u64` for any
        // width whose operands fit in memory.
        let mut carry: u64 = 0;
        for p in 0..BYTES {
            let mut col: u64 = carry;
            for x in 0..=p {
                let y = p - x;
                col += u64::from(a[BYTES - 1 - x]) * u64::from(b[BYTES - 1 - y]);
            }
            #[allow(clippy::cast_possible_truncation)]
            {
                out[BYTES - 1 - p] = (col & 0xff) as u8;
            }
            carry = col >> 8;
        }
        Ok(BYTES)
    }
}

// ADR-052 generic-form companion: replaces the hand-written
// AxisExtension impl. The macro's @generic arm accepts a `:ty` plus a
// generic parameter list so parametric Layer-3 axes inherit the
// dispatch body from the `axis!` emission.
axis_extension_impl_for_big_int_axis!(@generic BigIntModularNumeric<BYTES>, [const BYTES: usize]);

/// 256-bit modular arithmetic (mod `2^256`).
pub type BigInt256Numeric = BigIntModularNumeric<32>;
/// 512-bit modular arithmetic (mod `2^512`).
pub type BigInt512Numeric = BigIntModularNumeric<64>;
/// 128-bit modular arithmetic (mod `2^128`).
pub type BigInt128Numeric = BigIntModularNumeric<16>;
/// 64-bit modular arithmetic (mod `2^64`) — matches `u64` wrapping.
pub type BigInt64Numeric = BigIntModularNumeric<8>;

// ---- BigIntShape: ConstrainedTypeShape carrier for BigInt<N> -----------

/// Parametric ConstrainedTypeShape: an `N`-byte big-endian integer.
///
/// Per ADR-031 this is the canonical Layer-3 shape downstream
/// `prism_model!` invocations use to type their `Input` / `Output` as
/// big-integer values. The shape carries `BYTES` sites with no
/// admission constraints; admission discipline (range bounds, modulus,
/// etc.) is the consumer's responsibility through additional
/// constraint refs.
///
/// Per ADR-017's closure rule the IRI is the foundation's shared
/// `ConstrainedType` class; instance identity flows through
/// `(SITE_COUNT, CONSTRAINTS)`.
#[derive(Debug, Clone, Copy)]
pub struct BigIntShape<const BYTES: usize>;

impl<const BYTES: usize> Default for BigIntShape<BYTES> {
    fn default() -> Self {
        Self
    }
}

impl<const BYTES: usize> ConstrainedTypeShape for BigIntShape<BYTES> {
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(BYTES as u32);
}

impl<const BYTES: usize> uor_foundation::pipeline::__sdk_seal::Sealed for BigIntShape<BYTES> {}
impl<const BYTES: usize> GroundedShape for BigIntShape<BYTES> {}
impl<'a, const BYTES: usize> IntoBindingValue<'a> for BigIntShape<BYTES> {
    fn as_binding_value<const INLINE_BYTES: usize>(&self) -> TermValue<'a, INLINE_BYTES> {
        // The shape is a phantom carrier; downstream impls that want to
        // bind an actual N-byte big-int value wrap this shape in a
        // newtype carrying the data + a bespoke carrier.
        TermValue::empty()
    }
}

// ADR-033 G20 leaf-shape PartitionProductFields impl per
// foundation-sdk 0.4.11's depth-2 verb!-macro projection chain.
// Foundation-sdk 0.4.11 requires `PartitionProductFields` on every
// type used as a partition-product factor (including leaves) for
// the depth-2 chained-field-access trait-bound check to resolve.
// Empty FIELDS signals "atomic byte-sequence carrier — no further
// projection possible"; the macro respects the termination marker
// without indexing into the empty array (the 0.4.10 const-eval
// panic on empty FIELDS is fixed in 0.4.11).
impl<const BYTES: usize> uor_foundation::pipeline::PartitionProductFields for BigIntShape<BYTES> {
    const FIELDS: &'static [(u32, u32)] = &[];
    const FIELD_NAMES: &'static [&'static str] = &[];
}
