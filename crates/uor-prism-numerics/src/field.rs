//! `FieldAxis` declaration, secp256k1 base-field reference impl, and
//! parametric `FieldElementShape<BYTES>`.
//!
//! Prime-field arithmetic depends on the specific modulus, so this
//! sub-crate ships the secp256k1 base field
//! (`p = 2^256 - 2^32 - 977`) as the canonical reference. Other primes
//! are operational policy per ADR-031: an application that needs the
//! Ed25519 field (`p = 2^255 - 19`), the BLS12-381 base field, or a
//! Mersenne prime declares its own `FieldAxis` impl alongside the
//! standard library's secp256k1 impl through its `AxisTuple`.
//!
//! # ADR-055 substrate-Term verb body discipline
//!
//! Per [Wiki ADR-055](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)
//! every `AxisExtension` impl carries a substrate-Term verb body via
//! the foundation-declared `SubstrateTermBody` supertrait. The
//! `axis!` companion macro in foundation-sdk 0.4.11 emits a default
//! empty `body_arena()` for every impl that doesn't supply an
//! explicit `body = |input| { … };` clause (the
//! primitive-fast-path-equivalent realization); the hand-written
//! kernel below satisfies the discipline as-shipped.
//!
//! The substrate-Term canonical body for
//! `PrimeFieldNumericSecp256k1::{add, sub, mul}` —
//! `r#mod(<ring-arithmetic>(input.0, input.1), literal_bytes(SECP256K1_P_BYTES, W256_LEVEL))`
//! per ADR-054 (4) — **ships as verbs** in
//! [`crate::verbs::{secp256k1_field_add, secp256k1_field_sub,
//! secp256k1_field_mul}`]. Foundation-sdk 0.4.10's `literal_bytes`
//! wide-Witt-literal embedding admits the secp256k1 P_LITERAL as
//! a W256 inline constant; foundation-sdk 0.4.9 admitted `r#mod` as
//! a verb-body call form per ADR-053. The parametric-prime
//! `field_add` / `field_sub` / `field_mul` verbs (where `p` is an
//! input operand) also ship and exercise foundation-sdk 0.4.11's
//! depth-2 const-generic-leaf partition-product projection.
//!
//! Byte-output equivalence with the SEC 2 §2.4.1 vectors is verified
//! by direct vectors in `tests/conformance.rs`.

#![allow(missing_docs)]

use uor_foundation::enforcement::{GroundedShape, ShapeViolation};
use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef, IntoBindingValue, TermValue};
use uor_foundation_sdk::axis;

use crate::{check_output, split_pair};

axis! {
    /// Wiki ADR-031 prime-field arithmetic axis.
    ///
    /// The reference impl `PrimeFieldNumericSecp256k1` fixes the modulus
    /// at the secp256k1 base field prime: `p = 2^256 - 2^32 - 977`.
    pub trait FieldAxis: AxisExtension {
        /// ADR-017 content address.
        const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/FieldAxis";
        /// Operand byte-width (32 bytes for secp256k1).
        const MAX_OUTPUT_BYTES: usize = 32;
        /// `(a + b) mod p` — input `a || b` (64 bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn add(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
        /// `(a - b) mod p` — input `a || b` (64 bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn sub(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
        /// `(a * b) mod p` — input `a || b` (64 bytes).
        ///
        /// # Errors
        ///
        /// Returns `ShapeViolation` on input/output arity mismatch.
        fn mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation>;
    }
}

const WIDTH: usize = 32;

// secp256k1 base field prime p = 2^256 - 2^32 - 977.
const P: [u8; WIDTH] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0xff, 0xff, 0xfc, 0x2f,
];

fn cmp_ge(a: &[u8; WIDTH], b: &[u8; WIDTH]) -> bool {
    for i in 0..WIDTH {
        if a[i] != b[i] {
            return a[i] > b[i];
        }
    }
    true
}

fn sub_assign(target: &mut [u8; WIDTH], rhs: &[u8; WIDTH]) {
    let mut borrow: i16 = 0;
    for i in (0..WIDTH).rev() {
        let diff = i16::from(target[i]) - i16::from(rhs[i]) - borrow;
        if diff < 0 {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            {
                target[i] = (diff + 256) as u8;
            }
            borrow = 1;
        } else {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            {
                target[i] = diff as u8;
            }
            borrow = 0;
        }
    }
}

fn add_with_carry(a: &[u8; WIDTH], b: &[u8; WIDTH]) -> ([u8; WIDTH], u8) {
    let mut out = [0u8; WIDTH];
    let mut carry: u16 = 0;
    for i in (0..WIDTH).rev() {
        let sum = u16::from(a[i]) + u16::from(b[i]) + carry;
        #[allow(clippy::cast_possible_truncation)]
        {
            out[i] = (sum & 0xff) as u8;
        }
        carry = sum >> 8;
    }
    #[allow(clippy::cast_possible_truncation)]
    (out, carry as u8)
}

fn reduce_to_field(value: [u8; WIDTH], had_carry: bool) -> [u8; WIDTH] {
    let mut v = value;
    if had_carry {
        sub_assign(&mut v, &P);
    }
    while cmp_ge(&v, &P) {
        sub_assign(&mut v, &P);
    }
    v
}

fn mod_mul(a: &[u8; WIDTH], b: &[u8; WIDTH]) -> [u8; WIDTH] {
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
    let mut bytes = [0u8; 2 * WIDTH];
    for i in 0..2 * WIDTH {
        #[allow(clippy::cast_possible_truncation)]
        {
            bytes[i] = (acc[i] & 0xff) as u8;
        }
    }
    for shift_bytes in (0..=WIDTH).rev() {
        loop {
            let mut higher_than_p = false;
            for i in 0..WIDTH {
                let lhs = bytes[shift_bytes + i];
                let rhs = P[i];
                if lhs != rhs {
                    higher_than_p = lhs > rhs;
                    break;
                } else if i == WIDTH - 1 {
                    higher_than_p = true;
                }
            }
            let mut upper_zero = true;
            for byte in bytes.iter().take(shift_bytes) {
                if *byte != 0 {
                    upper_zero = false;
                    break;
                }
            }
            if !upper_zero || !higher_than_p {
                break;
            }
            let mut borrow: i16 = 0;
            for i in (0..WIDTH).rev() {
                let diff = i16::from(bytes[shift_bytes + i]) - i16::from(P[i]) - borrow;
                if diff < 0 {
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    {
                        bytes[shift_bytes + i] = (diff + 256) as u8;
                    }
                    borrow = 1;
                } else {
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    {
                        bytes[shift_bytes + i] = diff as u8;
                    }
                    borrow = 0;
                }
            }
        }
    }
    let mut out = [0u8; WIDTH];
    out.copy_from_slice(&bytes[WIDTH..]);
    out
}

fn read32(slice: &[u8]) -> [u8; WIDTH] {
    let mut out = [0u8; WIDTH];
    out.copy_from_slice(&slice[..WIDTH]);
    out
}

/// secp256k1 base-field arithmetic.
#[derive(Debug, Clone, Copy, Default)]
pub struct PrimeFieldNumericSecp256k1;

impl FieldAxis for PrimeFieldNumericSecp256k1 {
    const AXIS_ADDRESS: &'static str = "https://uor.foundation/axis/FieldAxis/Secp256k1Base";
    const MAX_OUTPUT_BYTES: usize = WIDTH;

    fn add(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        let (a, b) = split_pair(input, WIDTH)?;
        check_output(out, WIDTH)?;
        let a = read32(a);
        let b = read32(b);
        let (sum, carry) = add_with_carry(&a, &b);
        let result = reduce_to_field(sum, carry != 0);
        out[..WIDTH].copy_from_slice(&result);
        Ok(WIDTH)
    }

    fn sub(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        let (a, b) = split_pair(input, WIDTH)?;
        check_output(out, WIDTH)?;
        let a = read32(a);
        let b = read32(b);
        let mut p_minus_b = P;
        sub_assign(&mut p_minus_b, &b);
        let (sum, carry) = add_with_carry(&a, &p_minus_b);
        let result = reduce_to_field(sum, carry != 0);
        out[..WIDTH].copy_from_slice(&result);
        Ok(WIDTH)
    }

    fn mul(input: &[u8], out: &mut [u8]) -> Result<usize, ShapeViolation> {
        let (a, b) = split_pair(input, WIDTH)?;
        check_output(out, WIDTH)?;
        let a = read32(a);
        let b = read32(b);
        let result = mod_mul(&a, &b);
        out[..WIDTH].copy_from_slice(&result);
        Ok(WIDTH)
    }
}

axis_extension_impl_for_field_axis!(PrimeFieldNumericSecp256k1);

// ---- FieldElementShape: ConstrainedTypeShape carrier ----

/// Parametric ConstrainedTypeShape carrying an `N`-byte field-element
/// value (big-endian-encoded). Per ADR-031's `FieldElement<P>` shape
/// commitment — but with the byte-width as the type-level parameter
/// rather than the prime itself, since the field-element value
/// occupies exactly `ceil(log_256(p))` bytes for any prime `p` near
/// `2^(8N)`. The secp256k1 base field uses `BYTES = 32`.
#[derive(Debug, Clone, Copy)]
pub struct FieldElementShape<const BYTES: usize>;

impl<const BYTES: usize> Default for FieldElementShape<BYTES> {
    fn default() -> Self {
        Self
    }
}

impl<const BYTES: usize> ConstrainedTypeShape for FieldElementShape<BYTES> {
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(BYTES as u32);
}

impl<const BYTES: usize> uor_foundation::pipeline::__sdk_seal::Sealed for FieldElementShape<BYTES> {}
impl<const BYTES: usize> GroundedShape for FieldElementShape<BYTES> {}
impl<'a, const BYTES: usize> IntoBindingValue<'a> for FieldElementShape<BYTES> {
    fn as_binding_value<const INLINE_BYTES: usize>(&self) -> TermValue<'a, INLINE_BYTES> {
        TermValue::empty()
    }
}
