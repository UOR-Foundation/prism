//! `PolynomialShape<MAX_DEGREE, COEFF_BYTES>` — ADR-031 named
//! numerics shape carrier.
//!
//! Per [Wiki ADR-031][09-adr-031], prism-numerics ships
//! `Polynomial<MaxDegree, Coeff>` as one of its canonical shape
//! declarations alongside `BigInt<MaxBits>`, `FixedPoint<I, F>`, and
//! `FieldElement<P>`.
//!
//! This carrier is `MAX_DEGREE + 1` coefficient slots of `COEFF_BYTES`
//! each, packed big-endian — coefficient `c_i` lives at byte offset
//! `i * COEFF_BYTES`. Total site count: `(MAX_DEGREE + 1) * COEFF_BYTES`.
//!
//! Per ADR-017's closure rule the IRI is the foundation's shared
//! `ConstrainedType` class; instance identity flows through
//! `(SITE_COUNT, CONSTRAINTS)`. The shape carries no admission
//! constraints; application authors that need degree-bound, sparsity,
//! or coefficient-range admission discipline declare additional
//! `ConstraintRef` predicates in a newtype wrapper.
//!
//! Horner evaluation (`prism::numerics`'s `horner` verb in the ADR-031
//! roster) and polynomial multiplication compose this shape with
//! substrate `PrimitiveOp::{Add, Mul}` per ADR-050's width-parametric
//! evaluation discipline.
//!
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

use uor_foundation::enforcement::GroundedShape;
use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef, IntoBindingValue, TermValue};

/// Parametric `ConstrainedTypeShape` for a degree-`MAX_DEGREE` polynomial
/// with `COEFF_BYTES`-wide coefficients (big-endian).
///
/// Carries `MAX_DEGREE + 1` coefficient slots — index `i` holds the
/// `x^i` coefficient. The shape's identity flows through its byte-site
/// count per ADR-017.
#[derive(Debug, Clone, Copy)]
pub struct PolynomialShape<const MAX_DEGREE: usize, const COEFF_BYTES: usize>;

impl<const MAX_DEGREE: usize, const COEFF_BYTES: usize> Default
    for PolynomialShape<MAX_DEGREE, COEFF_BYTES>
{
    fn default() -> Self {
        Self
    }
}

impl<const MAX_DEGREE: usize, const COEFF_BYTES: usize> ConstrainedTypeShape
    for PolynomialShape<MAX_DEGREE, COEFF_BYTES>
{
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = (MAX_DEGREE + 1) * COEFF_BYTES;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
    #[allow(clippy::cast_possible_truncation)]
    const CYCLE_SIZE: u64 = 256u64.saturating_pow(((MAX_DEGREE + 1) * COEFF_BYTES) as u32);
}

impl<const MAX_DEGREE: usize, const COEFF_BYTES: usize> uor_foundation::pipeline::__sdk_seal::Sealed
    for PolynomialShape<MAX_DEGREE, COEFF_BYTES>
{
}
impl<const MAX_DEGREE: usize, const COEFF_BYTES: usize> GroundedShape
    for PolynomialShape<MAX_DEGREE, COEFF_BYTES>
{
}
impl<'a, const MAX_DEGREE: usize, const COEFF_BYTES: usize> IntoBindingValue<'a>
    for PolynomialShape<MAX_DEGREE, COEFF_BYTES>
{
    fn as_binding_value<const INLINE_BYTES: usize>(&self) -> TermValue<'a, INLINE_BYTES> {
        TermValue::empty()
    }
}

/// Degree-7 polynomial with 32-byte (256-bit) coefficients — canonical
/// shape for cryptographic polynomial commitments operating in a
/// `Z/(2^256)Z` coefficient ring.
pub type Polynomial7Mod256 = PolynomialShape<7, 32>;

/// Degree-15 polynomial with 32-byte coefficients — the depth-4
/// Merkle / KZG canonical commitment width.
pub type Polynomial15Mod256 = PolynomialShape<15, 32>;
