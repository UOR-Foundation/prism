//! Prism standard-library numerics sub-crate.
//!
//! `prism-numerics` realizes the numerics Layer-3 of the standard
//! library named in [Wiki ADR-031][09-adr-031]: it declares the
//! arithmetic-domain axis traits (`BigIntAxis`, `FixedPointAxis`,
//! `FieldAxis`, `RingAxis`) through the [`axis!`][09-adr-030] SDK
//! macro and supplies canonical reference impls per the wiki's
//! ADR-031 roster.
//!
//! ## Scope
//!
//! - **`BigIntAxis`** — arbitrary-precision integer arithmetic with
//!   a foundation-fixed maximum byte width. Reference impl:
//!   [`BigInt256Numeric`] — 256-bit fixed-width modular arithmetic.
//! - **`FixedPointAxis`** — Q-format fixed-point arithmetic.
//!   Reference impl: [`FixedPointQ32_32Numeric`] (Q32.32, 64-bit).
//! - **`FieldAxis`** — prime-field arithmetic. Reference impl:
//!   [`PrimeFieldNumericSecp256k1`] — the secp256k1 base field
//!   (`p = 2^256 - 2^32 - 977`).
//! - **`RingAxis`** — finite-ring arithmetic. Reference impl:
//!   [`Gf2NumericAxis`] — the binary field GF(2) (per-bit XOR / AND).
//!
//! Each axis lives in its own module so the per-method `KERNEL_*` ids
//! the `axis!` macro emits scope to a single axis (the axes share
//! method names like `add` / `mul`, which would otherwise collide).
//!
//! ## Closure under uor-foundation (ADR-013)
//!
//! Every axis trait declared here has `::uor_foundation::pipeline::AxisExtension`
//! as a supertrait (enforced by `axis!`), and every concrete impl is
//! registered for `AxisExtension` via the companion macro
//! `axis_extension_impl_for_<axis>!` per ADR-030.
//!
//! ## See also
//!
//! - [Wiki: 09 Architecture Decisions § ADR-030 — `axis!` SDK macro][09-adr-030]
//! - [Wiki: 09 Architecture Decisions § ADR-031 — `prism` is the standard library][09-adr-031]
//! - [Wiki: 12 Glossary § Numerics][12-glossary]
//!
//! [09-adr-030]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [12-glossary]: https://github.com/UOR-Foundation/UOR-Framework/wiki/12-Glossary

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

use uor_foundation::enforcement::ShapeViolation;

pub mod bigint;
pub mod field;
pub mod fixed_point;
pub mod ring;

pub use bigint::{BigInt256Numeric, BigIntAxis};
pub use field::{FieldAxis, PrimeFieldNumericSecp256k1};
pub use fixed_point::{FixedPointAxis, FixedPointQ32_32Numeric};
pub use ring::{Gf2NumericAxis, RingAxis};

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
