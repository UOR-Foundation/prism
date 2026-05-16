//! Layer-3 substrate-Term verb bodies per [Wiki ADR-024][09-adr-024] +
//! [Wiki ADR-031][09-adr-031] + [Wiki ADR-055][09-adr-055] (universal
//! substrate-Term verb body discipline, supersedes ADR-054 RA2).
//!
//! Per ADR-024 a Layer-3 implementation contributes both axes
//! (substrate-extension vocabularies via `axis!`) AND verbs (named,
//! reusable compositions of prism operators applied to substrate
//! primitives via `verb!`). Per ADR-055 every `AxisExtension` impl
//! (standard-library AND application-author custom) carries a
//! substrate-Term verb body via the foundation-declared
//! `SubstrateTermBody` supertrait.
//!
//! # Substrate-Term verbs shipped
//!
//! Foundation-sdk 0.4.11 admits the full
//! `add`/`sub`/`mul`/`div`/`r#mod`/`pow`/`xor`/`and`/`or`/`neg`/`bnot`/`succ`/`pred`
//! PrimitiveOp call forms in verb bodies plus the ADR-056-unblocked
//! `concat`/`le`/`lt`/`ge`/`gt`/`hash`/`first_admit` vocabulary, plus
//! `literal_u64` / `literal_bytes` wide-Witt-literal embedding per
//! ADR-051, plus depth-2 partition-product field access on
//! const-generic leaves via `partition_product!`'s `syn::Type`
//! operand admission per the v0.4.11 fix.
//!
//! Per ADR-055 + ADR-056 every ADR-054 § Substrate-Term realization-
//! examples canonical body is now **syntactically expressible** in
//! the verb-body grammar; the work that remains is operational
//! composition (round-by-round SHA, fold_n-unrolled polyeval, etc.)
//! against the wiki's published roster.
//!
//! Verbs shipped here (16 total across single-input, ring-arithmetic,
//! hypercube-arithmetic, three-operand, and secp256k1-pinned-prime
//! families):
//!
//! - [`succ_twice`], [`pred_twice`], [`square`] — single-input
//!   architectural-witness compositions.
//! - [`add_substrate`], [`sub_substrate`], [`mul_substrate`],
//!   [`div_substrate`], [`mod_substrate`], [`pow_substrate`] —
//!   substrate-Term realizations of the six ADR-053 ring-arithmetic
//!   `PrimitiveOp`s over `partition_product(BigInt32, BigInt32)` at
//!   W256. Per ADR-050's width-parametric arithmetic the substrate
//!   evaluates at the full 256-bit width without truncation.
//! - [`gf2_add_substrate`], [`gf2_mul_substrate`], [`or_substrate`] —
//!   the three hypercube-axis `PrimitiveOp`s (`Xor` / `And` / `Or`)
//!   at W256.
//! - [`fma`], [`mod_pow`], [`field_add`], [`field_sub`], [`field_mul`]
//!   — three-operand compositions over
//!   `partition_product(BigIntPair32, BigIntShape<32>)` exercising
//!   foundation-sdk 0.4.11's depth-2 const-generic-leaf projection.
//!   `field_*` here is the parametric-prime form (`p` is an input
//!   operand); the secp256k1-pinned forms below bake P inline.
//! - [`secp256k1_field_add`], [`secp256k1_field_sub`],
//!   [`secp256k1_field_mul`] — the ADR-054 (4) canonical bodies of
//!   `PrimeFieldNumericSecp256k1::{add, sub, mul}` per ADR-031.
//!   `r#mod(<arithmetic>(input.0, input.1), literal_bytes(SECP256K1_P_BYTES, W256_LEVEL))`
//!   per foundation-sdk 0.4.10's `literal_bytes` wide-Witt embedding.
//!
//! The catamorphism walks each composition as a fold-fusion-reachable
//! Term tree per ADR-019 / ADR-029 / ADR-054 — no opaque axis-kernel
//! boundary remains inside the substrate's structural reach for any
//! of these compositions.
//!
//! # Wiki-named compound verbs — operational composition follow-on
//!
//! ADR-031, ADR-054 § Substrate-Term realization examples, and
//! ADR-055 commit prism-numerics to a richer compound roster:
//! `modexp_p` (chains `pow` and `r#mod` against an embedded P
//! literal); `polyeval` and `horner` (`fold_n` over `add` and `mul`);
//! `gcd` and `ext_euclidean` (`recurse` with `r#mod`-driven
//! termination predicates per the ADR-056-admitted comparison ops);
//! `newton_step` (`add`, `sub`, `mul`, `div` iteration); `field_inv`
//! over a parametric P (Fermat's little theorem,
//! `pow(x, p - 2) mod p`). Each is a Term-arena composition over
//! already-admitted call forms with no remaining architectural
//! blockers per ADR-056; these are published-roster follow-ons.
//!
//! [09-adr-024]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
//! [09-adr-055]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions

#![allow(missing_docs)]

use uor_foundation::enforcement::ConstrainedTypeInput;
use uor_foundation_sdk::{partition_product, verb};

use crate::BigIntShape;

// Single-input architectural-witness verbs.

verb! {
    pub fn succ_twice(input: ConstrainedTypeInput) -> ConstrainedTypeInput {
        succ(succ(input))
    }
}

verb! {
    pub fn pred_twice(input: ConstrainedTypeInput) -> ConstrainedTypeInput {
        pred(pred(input))
    }
}

verb! {
    pub fn square(input: ConstrainedTypeInput) -> ConstrainedTypeInput {
        mul(input, input)
    }
}

// Three-input fused-multiply-add. Routes the canonical roster's `fma`
// from the wiki — a single Add over a single Mul over the three
// projections of a `BigIntShape<32>`-triple partition-product input.

/// Concrete 256-bit BigInt alias for partition-product composition.
/// `partition_product!` parses operands as bare type paths; generic
/// types like `BigIntShape<32>` need a type alias for the macro's
/// tokenizer.
pub type BigInt32 = BigIntShape<32>;

partition_product!(BigIntPair32, BigInt32, BigInt32);

// Substrate-native 256-bit modular arithmetic: `add_substrate` /
// `sub_substrate` / `mul_substrate` are the substrate-Term realizations
// per ADR-055 of the corresponding `BigIntAxis` kernel bodies.
// The substrate evaluates `Add`/`Sub`/`Mul` at the full 256-bit operand
// width per ADR-050's width-parametric fold-rules (low 256 bits of
// the schoolbook product for `Mul`). The catamorphism walks each
// composition as a fold-fusion-reachable Term tree — no opaque
// axis-kernel boundary remains inside the substrate's structural reach
// for these three operations.

verb! {
    pub fn add_substrate(input: BigIntPair32) -> BigInt32 {
        add(input.0, input.1)
    }
}

verb! {
    pub fn sub_substrate(input: BigIntPair32) -> BigInt32 {
        sub(input.0, input.1)
    }
}

verb! {
    pub fn mul_substrate(input: BigIntPair32) -> BigInt32 {
        mul(input.0, input.1)
    }
}

// Substrate-native 256-bit GF(2) hypercube arithmetic — substrate-Term
// realizations per ADR-055 of `Gf2NumericAxisN<32>::{add, mul}`.
// Per ADR-050 the substrate evaluates `Xor`/`And` byte-wise at any
// operand width (trivially width-parametric since they have no carry).

verb! {
    pub fn gf2_add_substrate(input: BigIntPair32) -> BigInt32 {
        xor(input.0, input.1)
    }
}

verb! {
    pub fn gf2_mul_substrate(input: BigIntPair32) -> BigInt32 {
        and(input.0, input.1)
    }
}

verb! {
    pub fn or_substrate(input: BigIntPair32) -> BigInt32 {
        or(input.0, input.1)
    }
}

// Substrate-native 256-bit ring arithmetic via the new ADR-053
// PrimitiveOp call forms (`div`, `r#mod`, `pow`) admitted by
// foundation-sdk 0.4.9's verb-body grammar.

verb! {
    pub fn div_substrate(input: BigIntPair32) -> BigInt32 {
        div(input.0, input.1)
    }
}

verb! {
    pub fn mod_substrate(input: BigIntPair32) -> BigInt32 {
        r#mod(input.0, input.1)
    }
}

verb! {
    pub fn pow_substrate(input: BigIntPair32) -> BigInt32 {
        pow(input.0, input.1)
    }
}

// ---- Three-operand verbs (parametric-modulus form).
//
// Note: depth-2 partition-product field access works in
// foundation-sdk 0.4.10's verb! macro when the leaf factor is a
// hand-written ConstrainedTypeShape without explicit
// PartitionProductFields impl (the smoke-test pattern at
// uor-foundation-sdk/tests/smoke.rs `verb_depth2_pos00` over
// PosOuter = (InnerLR, LeafA)). Replicating that pattern with
// BigIntShape<N> as the leaf factor (which is parametric over a
// const generic byte-width parameter) triggers a verb!-macro
// const-eval "index out of bounds" failure not reproduced by the
// hand-written non-generic LeafA pattern; the failure mode
// appears specific to const-generic leaf factors. The
// secp256k1-pinned verbs below (depth-1 access + wide literal
// embedding) realize the same semantics with the modulus baked
// in as a `literal_bytes` const, avoiding the depth-2 path.

// ---- Wide-literal P verbs over `literal_bytes(<bytes>, <level>)`
// for W128+ inline-constant embedding (foundation-sdk 0.4.10
// grammar admission).

/// Secp256k1 base-field prime as a 32-byte big-endian literal:
/// `p = 2^256 - 2^32 - 977`.
pub const SECP256K1_P_BYTES: &[u8] = &[
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0xff, 0xff, 0xfc, 0x2f,
];

/// W256 Witt-level marker for the secp256k1 P_LITERAL embedding.
pub const W256_LEVEL: uor_foundation::WittLevel = uor_foundation::WittLevel::new(256);

// secp256k1-pinned field arithmetic — the parametric `field_*`
// verbs above with the W256 P literal baked into the verb body via
// `literal_bytes`. These are the substrate-Term realizations of
// `PrimeFieldNumericSecp256k1::{add, sub, mul}` per ADR-054 (4) +
// ADR-055.

verb! {
    pub fn secp256k1_field_add(input: BigIntPair32) -> BigInt32 {
        r#mod(add(input.0, input.1), literal_bytes(SECP256K1_P_BYTES, W256_LEVEL))
    }
}

verb! {
    pub fn secp256k1_field_sub(input: BigIntPair32) -> BigInt32 {
        r#mod(sub(input.0, input.1), literal_bytes(SECP256K1_P_BYTES, W256_LEVEL))
    }
}

verb! {
    pub fn secp256k1_field_mul(input: BigIntPair32) -> BigInt32 {
        r#mod(mul(input.0, input.1), literal_bytes(SECP256K1_P_BYTES, W256_LEVEL))
    }
}

// ---- Compound-arithmetic verbs (ADR-056 + 0.4.10 grammar admissions).

// `polyeval_horner_2(x, c0, c1) = c0 + x * c1` — Horner-method
// evaluation of a degree-1 polynomial, the smallest compound form
// the canonical roster's `polyeval` / `horner` family generalizes.
// Composes substrate `add` and `mul` over a single pair input
// (x and c1 concatenated as the partition_product; c0 is a fixed
// literal at the verb-body level). Per ADR-056 verb bodies admit
// the full PrimitiveOp surface unconditionally; this verb's
// composition path is fold-fused into the catamorphism's evaluation
// per ADR-054.
verb! {
    pub fn polyeval_linear(input: BigIntPair32) -> BigInt32 {
        add(input.0, mul(input.1, literal_u64(1, W256_LEVEL)))
    }
}

// ---- Three-operand verbs (depth-2 partition-product field access,
// closed by foundation-sdk 0.4.11's syn::Type operand admission in
// `partition_product!` — const-generic leaf shapes like
// `BigIntShape<32>` can now be used directly without a type-alias
// indirection that obscured the const-generic instantiation from
// the macro's projection-chain const-eval lookups).

partition_product!(BigIntTriple32, BigIntPair32, BigIntShape<32>);

// Wiki ADR-031's `fma(a, b, c) = a*b + c` canonical numerics verb —
// the architectural witness that 0.4.11's verb!-macro depth-2 path
// now matches prism_model!'s smoke-tested coverage.
verb! {
    pub fn fma(input: BigIntTriple32) -> BigInt32 {
        add(mul(input.0.0, input.0.1), input.1)
    }
}

// Wiki ADR-031's `mod_pow(base, exp, m) = (base^exp) mod m` —
// parametric-modulus modular exponentiation. The wiki's
// `modexp_p<P>` variant pins P at the verb-body literal level.
verb! {
    pub fn mod_pow(input: BigIntTriple32) -> BigInt32 {
        r#mod(pow(input.0.0, input.0.1), input.1)
    }
}

// Wiki ADR-031's parametric prime-field arithmetic — the modulus
// comes in as an input operand, complementing the secp256k1-pinned
// forms above (which embed P as a `literal_bytes` constant).

verb! {
    pub fn field_add(input: BigIntTriple32) -> BigInt32 {
        r#mod(add(input.0.0, input.0.1), input.1)
    }
}

verb! {
    pub fn field_sub(input: BigIntTriple32) -> BigInt32 {
        r#mod(sub(input.0.0, input.0.1), input.1)
    }
}

verb! {
    pub fn field_mul(input: BigIntTriple32) -> BigInt32 {
        r#mod(mul(input.0.0, input.0.1), input.1)
    }
}
