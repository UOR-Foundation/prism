# Mathematical Foundation

The algebraic structure underlying PRISM, for readers with background in abstract algebra or formal methods.


## Carrier Set

PRISM operates over the modular ring **Z/(2ⁿ)Z**, where `n = 8 × (quantum + 1)`. Elements are represented as big endian byte tuples of width `quantum + 1`.

## Signature Σ

| Symbol | Arity | Definition | Properties |
|---|---|---|---|
| `neg` | unary | `-x mod 2ⁿ` | involution |
| `bnot` | unary | `~x` (bitwise complement) | involution |
| `xor` | binary | bitwise exclusive or | commutative, associative |
| `and` | binary | bitwise and | commutative, associative |
| `or` | binary | bitwise or | commutative, associative |

Binary operations accept n ary syntax: `xor(a, b, c) ≡ xor(xor(a, b), c)`. Canonicalization flattens and sorts.

## Critical Identity

**Theorem.** For all `x ∈ Z/(2ⁿ)Z`:

```
neg(bnot(x)) = x + 1 mod 2ⁿ
bnot(neg(x)) = x - 1 mod 2ⁿ
```

**Proof.** `bnot(x) = 2ⁿ - 1 - x` (ones complement). Then `neg(bnot(x)) = -(2ⁿ - 1 - x) mod 2ⁿ = x + 1 mod 2ⁿ`. ∎

This defines derived operations: `succ(x) = neg(bnot(x))`, `pred(x) = bnot(neg(x))`.

## Universality Theorem

**Theorem.** No nonempty proper subset `S ⊂ Z/(2ⁿ)Z` is closed under both `neg` and `bnot`.

**Proof.** If `S` is closed under both, then for any `x ∈ S`: `bnot(x) ∈ S` and `neg(bnot(x)) ∈ S`. By the critical identity, `x + 1 ∈ S`. Since `succ` generates the cyclic group and `S` is nonempty, `S = Z/(2ⁿ)Z`. ∎

## Triadic Coordinates

For `x` represented as byte tuple `(b₀, b₁, ..., b_{w-1})`:

* **Datum:** `(b₀, b₁, ..., b_{w-1})`
* **Stratum:** `(popcount(b₀), ..., popcount(b_{w-1}))`, total `|σ(x)| = Σᵢ popcount(bᵢ)`
* **Spectrum:** `(basis(b₀), ..., basis(b_{w-1}))` where `basis(b) = {i ∈ {0..7} : b & (1 << i) ≠ 0}`

**Stratum symmetry:** `|σ(x)| + |σ(bnot(x))| = 8w` for all `x`.

**Stratum distribution (Q0):** Values with total stratum `k` number `C(8, k)` (binomial), giving Pascal's triangle row 8.

## Correlation Metric

```
d(a, b) = |σ(a ⊕ b)|          (Hamming distance)
fidelity(a, b) = 1 - d(a,b)/n  (normalized)
```

This is a proper metric on the carrier set.

## Closure Semantics

| Mode | Definition | Note |
|---|---|---|
| ONE_STEP | `S' = S ∪ {f(x) : f ∈ F, x ∈ S}` | Single pass, does not close under composition |
| FIXED_POINT | Iterate until stable | For `{neg, bnot}`, produces full ring |
| GRAPH_CLOSED | Fixed point + referential integrity verification | For both involutions, requires full enumeration |

## Canonicalization

Terms are normalized by these rewrite rules (confluent and terminating):

1. Involution cancellation: `f(f(x)) → x`
2. Derived expansion: `succ(x) → neg(bnot(x))`
3. Constant reduction: `c → c mod 2ⁿ`
4. AC flatten + sort: nested associative ops flattened, operands sorted
5. Identity elimination: `x ⊕ 0 → x`, `x ∧ mask → x`, `x ∨ 0 → x`
6. Annihilator reduction: `x ∧ 0 → 0`, `x ∨ mask → mask`
7. Self cancellation: `x ⊕ x → 0`
8. Idempotence: `x ∧ x → x`, `x ∨ x → x`

Not normalized: absorption, distributivity, general equational equivalence.

## Content Addressing

Derivation IDs are computed as:

```
SHA256(canonical_serialize(term) + "=" + result_iri)[:16]
```

prefixed with `urn:uor:derivation:sha256:`. Canonical serialization uses fixed width hex at the correct quantum width.

## IRI Encoding

Bytes map to Unicode Braille codepoints: `codepoint(byte) = U+2800 + byte`. The Braille block U+2800..U+28FF provides a bijection with byte values, and each character visually encodes the active bit positions as raised dots.
