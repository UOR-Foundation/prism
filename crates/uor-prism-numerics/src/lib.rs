//! Prism standard-library numerics sub-crate.
//!
//! `prism-numerics` realizes the numerics Layer-3 of the standard
//! library named in [Wiki ADR-031][09-adr-031]: it declares the
//! arithmetic-domain axis traits (`BigIntAxis`, `FixedPointAxis`,
//! `FieldAxis`, `RingAxis`) through the [`axis!`][09-adr-030] SDK
//! macro and supplies parametric reference impls plus matching
//! ConstrainedTypeShape carriers per the wiki's ADR-031 roster.
//!
//! ## Scope
//!
//! Every axis kernel takes `(input: &[u8], out: &mut [u8])` per
//! ADR-030's signature contract. Axis impls are generic in their
//! natural axis (byte-width, Q-format split) so applications can
//! instantiate the impl their model needs without re-rolling the
//! kernel body.
//!
//! - **`BigIntAxis`** — `(a + b) / (a - b) / (a * b) mod 2^(8*N)`.
//!   Parametric: [`BigIntModularNumeric<BYTES>`] for any `BYTES ≥ 1`
//!   (no width ceiling). Aliases: [`BigInt64Numeric`],
//!   [`BigInt128Numeric`], [`BigInt256Numeric`], [`BigInt512Numeric`].
//!   Shape: [`BigIntShape<BYTES>`].
//! - **`FixedPointAxis`** — Q-format arithmetic on a 64-bit container.
//!   Parametric: [`FixedPointQNumeric<INT_BITS, FRAC_BITS>`].
//!   Aliases: [`FixedPointQ16_16Numeric`], [`FixedPointQ32_32Numeric`],
//!   [`FixedPointQ1_31Numeric`], [`FixedPointQ48_16Numeric`].
//!   Shape: [`FixedPointShape<I, F>`].
//! - **`FieldAxis`** — prime-field arithmetic. The reference impl
//!   [`PrimeFieldNumericSecp256k1`] fixes the modulus at
//!   `p = 2^256 - 2^32 - 977`; alternative primes are operational
//!   policy per ADR-031. Shape: [`FieldElementShape<BYTES>`].
//! - **`RingAxis`** — finite-ring arithmetic. Parametric:
//!   [`Gf2NumericAxisN<BYTES>`] for GF(2) over `N` bytes (bitwise
//!   XOR / AND). Aliases: [`Gf2NumericAxis`], [`Gf2NumericAxis128`],
//!   [`Gf2NumericAxis512`]. Shape: [`Gf2RingShape<BYTES>`].
//!
//! ## ConstrainedTypeShape declarations
//!
//! Per ADR-031's shape-declaration commitment (`BigInt<MaxBits>`,
//! `FixedPoint<I, F>`, `FieldElement<P>`, ...), each axis has a
//! matching `ConstrainedTypeShape` carrier so downstream
//! `prism_model!` invocations can use the shape as `Input` / `Output`
//! through the SDK macros. Every shape is `GroundedShape +
//! IntoBindingValue`-bound for use as a model `Output` per ADR-027.
//! Per ADR-017's closure rule, shape identity flows through
//! `(SITE_COUNT, CONSTRAINTS)` — distinct parametric instantiations
//! with the same site count content-address identically.
//!
//! ## Closure under uor-foundation (ADR-013)
//!
//! Every axis trait has `::uor_foundation::pipeline::AxisExtension` as
//! a supertrait (enforced by `axis!`). Parametric axis impls
//! hand-write their `AxisExtension` impl since the `axis!`-emitted
//! companion macro takes `:ident` and cannot apply to generic types
//! (the hand-written impls replicate the companion macro's dispatch
//! arms verbatim).
//!
//! ## See also
//!
//! - [Wiki: 09 Architecture Decisions § ADR-027 — `output_shape!` SDK macro][09-adr-027]
//! - [Wiki: 09 Architecture Decisions § ADR-030 — `axis!` SDK macro][09-adr-030]
//! - [Wiki: 09 Architecture Decisions § ADR-031 — `prism` is the standard library][09-adr-031]
//! - [Wiki: 12 Glossary § Numerics][12-glossary]
//!
//! [09-adr-027]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-030]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [12-glossary]: https://github.com/UOR-Foundation/UOR-Framework/wiki/12-Glossary

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

use uor_foundation::enforcement::ShapeViolation;

pub mod bigint;
pub mod field;
pub mod fixed_point;
pub mod polynomial;
pub mod ring;
pub mod verbs;

pub use bigint::{
    BigInt128Numeric, BigInt256Numeric, BigInt512Numeric, BigInt64Numeric, BigIntAxis,
    BigIntModularNumeric, BigIntShape,
};
pub use field::{FieldAxis, FieldElementShape, PrimeFieldNumericSecp256k1};
pub use fixed_point::{
    FixedPointAxis, FixedPointQ16_16Numeric, FixedPointQ1_31Numeric, FixedPointQ32_32Numeric,
    FixedPointQ48_16Numeric, FixedPointQNumeric, FixedPointShape,
};
pub use polynomial::{Polynomial15Mod256, Polynomial7Mod256, PolynomialShape};
pub use ring::{
    Gf2NumericAxis, Gf2NumericAxis128, Gf2NumericAxis512, Gf2NumericAxisN, Gf2RingShape, RingAxis,
};

/// Wiki ADR-031 standard-library version banner.
pub const STANDARD_LIBRARY_VERSION: &str = env!("CARGO_PKG_VERSION");

fn arity_violation(constraint: &'static str) -> ShapeViolation {
    ShapeViolation {
        shape_iri: "https://uor.foundation/axis/NumericAxisShape",
        constraint_iri: constraint,
        property_iri: "https://uor.foundation/axis/inputBytes",
        expected_range: "https://uor.foundation/axis/NumericInputArity",
        min_count: 0,
        max_count: 0,
        kind: uor_foundation::ViolationKind::ValueCheck,
    }
}

pub(crate) fn split_pair(
    input: &[u8],
    operand_bytes: usize,
) -> Result<(&[u8], &[u8]), ShapeViolation> {
    if input.len() != 2 * operand_bytes {
        return Err(arity_violation(
            "https://uor.foundation/axis/NumericAxisShape/operandPair",
        ));
    }
    Ok((&input[..operand_bytes], &input[operand_bytes..]))
}

pub(crate) fn check_output(out: &[u8], bytes: usize) -> Result<(), ShapeViolation> {
    if out.len() < bytes {
        return Err(arity_violation(
            "https://uor.foundation/axis/NumericAxisShape/outputBuffer",
        ));
    }
    Ok(())
}
