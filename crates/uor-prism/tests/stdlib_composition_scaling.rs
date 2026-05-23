//! Scaling V&V for the ADR-061 composition shapes and the
//! decentralized publication-graph shapes — the falsification suite for
//! the claim that these shapes **scale arbitrarily, with no arbitrary
//! limit**.
//!
//! # The falsifiable claim
//!
//! `G2ProductShape<N>`, `F4QuotientShape<N>`, `E6FiltrationShape<N>`,
//! `E7AugmentationShape<N>`, `E8EmbeddingShape<N>`, `RouteShape<…>`, and
//! `RevocationShape<…>` impose **no ceiling** on their component-label
//! byte widths. Their `SITE_COUNT` is a pure parametric function of the
//! const-generic widths (`2×N` for the binary G₂ product, `N` for the
//! operand-preserving unary operations F₄/E₇/E₈, `N + 1` for the
//! structure-preserving E₆ filtration's one-byte degree-partition tag
//! per wiki ADR-061 §(2), the sum of the per-component widths for the
//! route/revocation shapes), and admission through
//! `uor-foundation`'s constrained-type path is **independent of
//! `SITE_COUNT`**.
//!
//! # Why there is no width ceiling (structural argument)
//!
//! `validate_constrained_type` runs exactly two preflight checks —
//! `preflight_feasibility(T::CONSTRAINTS)` and
//! `preflight_package_coherence(T::CONSTRAINTS)`. Both inspect only
//! `T::CONSTRAINTS`; **neither reads `T::SITE_COUNT`**. Per ADR-017's
//! closure rule every shape here carries `CONSTRAINTS = &[]`, so the
//! preflight walk is empty and admission is unconditional — at width 1,
//! at a σ-axis-canonical 71, or at 16 MiB. There is no clamp, no
//! `MAX_SITES`, no rejection branch keyed on the width. ADR-060 already
//! removed the foundation's fictional byte-width caps; this suite is the
//! conformance witness that nothing reintroduced one at the shape layer.
//!
//! # The one documented, non-arbitrary bound: ADR-032 `CYCLE_SIZE`
//!
//! The only width-dependent quantity is `CYCLE_SIZE`, defined per
//! ADR-032 as `256^SITE_COUNT` evaluated with **saturating** arithmetic
//! (`256u64.saturating_pow(SITE_COUNT)`). For `SITE_COUNT ≥ 8` the true
//! value exceeds `u64::MAX`, so it saturates to `u64::MAX` — a graceful,
//! documented saturation shared with `FixedSites<N>` / `Bytes<N>`, not
//! an arbitrary cap on the shape. This suite pins both regimes: exact
//! `256^k` below the saturation threshold, `u64::MAX` at and above it.
//!
//! # Method
//!
//! The const-generic width is a compile-time parameter, so the spread is
//! enumerated as explicit instantiations rather than a runtime loop. The
//! parametric-exactness checks are `const _: () = assert!(…)` items —
//! they hold at **compile time** (TC-01), so a regression fails the
//! build, not merely the test run. The spread is geometric and spans
//! eight orders of magnitude: degenerate (`0`, `1`), the σ-axis-canonical
//! κ-label widths (`71`/`73`/`74`), and progressively larger widths up to
//! `16_777_216` (16 MiB), well beyond any plausible "arbitrary limit"
//! while staying within `usize`/`u32` arithmetic so `2×N` and the
//! `saturating_pow` exponent cast are exact.
//!
//! See [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
//! for the standard-type-library contract and the wiki's
//! [02 Architecture Constraints § Substitution Axes][02-axes] for the
//! arbitrary-scaling commitment these shapes inherit.
//!
//! [02-axes]: https://github.com/UOR-Foundation/UOR-Framework/wiki/02-Architecture-Constraints

#![allow(clippy::unwrap_used, clippy::expect_used)]

use prism::pipeline::{validate_constrained_type, ConstrainedTypeShape};
use prism::std_types::{
    E6FiltrationShape, E7AugmentationShape, E8EmbeddingShape, F4QuotientShape, G2ProductShape,
    RevocationShape, RouteShape,
};

// ---- Parametric exactness across eight orders of magnitude ----
//
// `SITE_COUNT` is a pure function of the width with no clamp. Each arm
// is a compile-time assertion: the binary product is exactly `2×N`, the
// operand-preserving unary operations (F₄, E₇, E₈) are exactly `N`,
// and the structure-preserving E₆ filtration is exactly `N + 1` (the
// one-byte degree-partition tag per wiki ADR-061 §(2)), at every width
// in the spread.

/// Assert `G2ProductShape<N>::SITE_COUNT == 2×N`, the operand-
/// preserving unary shapes (F₄, E₇, E₈) `SITE_COUNT == N`, and the
/// structure-preserving E₆ filtration `SITE_COUNT == N + 1` (the one-
/// byte degree-partition tag per wiki ADR-061 §(2)), at compile time,
/// for each width.
macro_rules! assert_arity_exact {
    ($($n:literal),* $(,)?) => {$(
        const _: () = assert!(
            <G2ProductShape<$n> as ConstrainedTypeShape>::SITE_COUNT == 2 * $n
        );
        const _: () = assert!(
            <F4QuotientShape<$n> as ConstrainedTypeShape>::SITE_COUNT == $n
        );
        const _: () = assert!(
            <E6FiltrationShape<$n> as ConstrainedTypeShape>::SITE_COUNT == $n + 1
        );
        const _: () = assert!(
            <E7AugmentationShape<$n> as ConstrainedTypeShape>::SITE_COUNT == $n
        );
        const _: () = assert!(
            <E8EmbeddingShape<$n> as ConstrainedTypeShape>::SITE_COUNT == $n
        );
    )*};
}

// Degenerate, σ-axis-canonical, and progressively larger widths.
assert_arity_exact!(
    0, 1, 2, 3, 4, 7, 8, 16, 71, 73, 74, 80, 256, 4096, 65536, 1_048_576, 16_777_216,
);

#[test]
fn site_count_scales_linearly_with_no_clamp() {
    // A monotone width sequence must produce a strictly monotone
    // SITE_COUNT for the unary shapes and `2×` that for the product —
    // i.e. the formula never plateaus at a hidden ceiling.
    const W: [usize; 6] = [71, 256, 4096, 65536, 1_048_576, 16_777_216];
    const G: [usize; 6] = [
        <G2ProductShape<71> as ConstrainedTypeShape>::SITE_COUNT,
        <G2ProductShape<256> as ConstrainedTypeShape>::SITE_COUNT,
        <G2ProductShape<4096> as ConstrainedTypeShape>::SITE_COUNT,
        <G2ProductShape<65536> as ConstrainedTypeShape>::SITE_COUNT,
        <G2ProductShape<1_048_576> as ConstrainedTypeShape>::SITE_COUNT,
        <G2ProductShape<16_777_216> as ConstrainedTypeShape>::SITE_COUNT,
    ];
    const F: [usize; 6] = [
        <F4QuotientShape<71> as ConstrainedTypeShape>::SITE_COUNT,
        <F4QuotientShape<256> as ConstrainedTypeShape>::SITE_COUNT,
        <F4QuotientShape<4096> as ConstrainedTypeShape>::SITE_COUNT,
        <F4QuotientShape<65536> as ConstrainedTypeShape>::SITE_COUNT,
        <F4QuotientShape<1_048_576> as ConstrainedTypeShape>::SITE_COUNT,
        <F4QuotientShape<16_777_216> as ConstrainedTypeShape>::SITE_COUNT,
    ];
    const E6: [usize; 6] = [
        <E6FiltrationShape<71> as ConstrainedTypeShape>::SITE_COUNT,
        <E6FiltrationShape<256> as ConstrainedTypeShape>::SITE_COUNT,
        <E6FiltrationShape<4096> as ConstrainedTypeShape>::SITE_COUNT,
        <E6FiltrationShape<65536> as ConstrainedTypeShape>::SITE_COUNT,
        <E6FiltrationShape<1_048_576> as ConstrainedTypeShape>::SITE_COUNT,
        <E6FiltrationShape<16_777_216> as ConstrainedTypeShape>::SITE_COUNT,
    ];

    for i in 0..W.len() {
        assert_eq!(
            F[i], W[i],
            "operand-preserving unary SITE_COUNT (F₄/E₇/E₈) must equal the width exactly"
        );
        assert_eq!(G[i], 2 * W[i], "product SITE_COUNT must equal 2× the width");
        assert_eq!(
            E6[i],
            W[i] + 1,
            "structure-preserving E₆ filtration SITE_COUNT must equal width + 1"
        );
        if i > 0 {
            assert!(F[i] > F[i - 1], "SITE_COUNT must stay strictly monotone");
            assert!(G[i] > G[i - 1], "no hidden plateau / clamp at scale");
            assert!(
                E6[i] > E6[i - 1],
                "E₆ SITE_COUNT must stay strictly monotone"
            );
        }
    }
}

// ---- Admission is independent of SITE_COUNT (no width ceiling) ----

/// Admit every shape at width `$n` through the foundation's
/// constrained-type path. Because admission inspects only `CONSTRAINTS`
/// (empty here), success is independent of how large `$n` is.
macro_rules! admit_all_shapes_at {
    ($($n:literal),* $(,)?) => {$(
        validate_constrained_type(G2ProductShape::<$n>)
            .expect(concat!("G2ProductShape<", stringify!($n), "> admits"));
        validate_constrained_type(F4QuotientShape::<$n>)
            .expect(concat!("F4QuotientShape<", stringify!($n), "> admits"));
        validate_constrained_type(E6FiltrationShape::<$n>)
            .expect(concat!("E6FiltrationShape<", stringify!($n), "> admits"));
        validate_constrained_type(E7AugmentationShape::<$n>)
            .expect(concat!("E7AugmentationShape<", stringify!($n), "> admits"));
        validate_constrained_type(E8EmbeddingShape::<$n>)
            .expect(concat!("E8EmbeddingShape<", stringify!($n), "> admits"));
    )*};
}

#[test]
fn composition_shapes_admit_at_every_scale() {
    // The crux of "no arbitrary limit": admission succeeds from a single
    // site up to 16 MiB-class widths, because the admission path never
    // reads SITE_COUNT. A reintroduced width ceiling anywhere in the
    // tower would make one of these `expect`s panic.
    admit_all_shapes_at!(1, 71, 256, 4096, 65536, 1_048_576, 16_777_216);
}

#[test]
fn route_and_revocation_shapes_admit_at_every_scale() {
    // The publication-graph shapes carry five / six independent widths;
    // each admits regardless of the per-component magnitudes.
    validate_constrained_type(RouteShape::<1, 1, 1, 1, 1>).expect("minimal route admits");
    validate_constrained_type(RouteShape::<71, 32, 32, 71, 71>).expect("canonical route admits");
    validate_constrained_type(RouteShape::<65536, 65536, 65536, 65536, 65536>)
        .expect("64 KiB-per-component route admits");
    validate_constrained_type(RouteShape::<1_048_576, 4096, 256, 1_048_576, 1_048_576>)
        .expect("MiB-class route admits");

    validate_constrained_type(RevocationShape::<1, 1, 1, 1, 1, 1>)
        .expect("minimal revocation admits");
    validate_constrained_type(RevocationShape::<71, 32, 32, 71, 71, 71>)
        .expect("canonical revocation admits");
    validate_constrained_type(
        RevocationShape::<1_048_576, 4096, 256, 1_048_576, 1_048_576, 1_048_576>,
    )
    .expect("MiB-class revocation admits");
}

// ---- Route / revocation parametric sums at scale ----

#[test]
fn route_site_count_is_exact_parametric_sum_at_scale() {
    // SITE_COUNT is the exact sum of the five widths — at canonical
    // widths and at MiB-class widths alike, with no saturation in the
    // sum itself (saturation lives only in CYCLE_SIZE).
    const CANON: usize = <RouteShape<71, 32, 32, 71, 71> as ConstrainedTypeShape>::SITE_COUNT;
    const BIG: usize =
        <RouteShape<1_048_576, 4096, 256, 1_048_576, 1_048_576> as ConstrainedTypeShape>::SITE_COUNT;
    assert_eq!(CANON, 71 + 32 + 32 + 71 + 71);
    assert_eq!(BIG, 1_048_576 + 4096 + 256 + 1_048_576 + 1_048_576);
}

#[test]
fn revocation_extends_route_by_revoked_width_at_scale() {
    // The route → revocation structural relationship (revocation =
    // route + revoked-label width) holds at any scale.
    const ROUTE_BIG: usize =
        <RouteShape<1_048_576, 4096, 256, 1_048_576, 1_048_576> as ConstrainedTypeShape>::SITE_COUNT;
    const REV_BIG: usize = <RevocationShape<1_048_576, 4096, 256, 1_048_576, 1_048_576, 65536>
        as ConstrainedTypeShape>::SITE_COUNT;
    assert_eq!(REV_BIG, ROUTE_BIG + 65536);
}

// ---- ADR-032 CYCLE_SIZE: exact below threshold, saturated above ----
//
// The single width-dependent quantity. `256^k = 2^(8k)` overflows
// `u64` once `8k ≥ 64`, i.e. `k ≥ 8`, where ADR-032 saturates to
// `u64::MAX`. Below the threshold the value is exact. This is graceful,
// documented saturation — not an arbitrary ceiling on the shape — and
// computing it at MiB-class widths neither overflows nor panics
// (saturating exponentiation is logarithmic, short-circuiting once
// saturated).

#[test]
fn cycle_size_is_exact_below_the_saturation_threshold() {
    // Operand-preserving unary shapes: SITE_COUNT = N, so
    // CYCLE_SIZE = 256^N exact for N < 8.
    assert_eq!(<F4QuotientShape<0> as ConstrainedTypeShape>::CYCLE_SIZE, 1);
    assert_eq!(
        <F4QuotientShape<1> as ConstrainedTypeShape>::CYCLE_SIZE,
        256
    );
    assert_eq!(
        <F4QuotientShape<2> as ConstrainedTypeShape>::CYCLE_SIZE,
        65_536
    );
    assert_eq!(
        <E8EmbeddingShape<7> as ConstrainedTypeShape>::CYCLE_SIZE,
        72_057_594_037_927_936, // 256^7 = 2^56
    );

    // Structure-preserving E₆ filtration: SITE_COUNT = N + 1, so
    // CYCLE_SIZE = 256^(N+1) exact for N < 7 (saturates one width
    // earlier than the operand-preserving unaries).
    assert_eq!(
        <E6FiltrationShape<0> as ConstrainedTypeShape>::CYCLE_SIZE,
        256
    ); // 256^1 — the degree-partition tag alone
    assert_eq!(
        <E6FiltrationShape<1> as ConstrainedTypeShape>::CYCLE_SIZE,
        65_536
    ); // 256^2
    assert_eq!(
        <E6FiltrationShape<6> as ConstrainedTypeShape>::CYCLE_SIZE,
        72_057_594_037_927_936, // 256^7 = 2^56
    );

    // Binary product: SITE_COUNT = 2N, so the exact regime is N < 4.
    assert_eq!(<G2ProductShape<0> as ConstrainedTypeShape>::CYCLE_SIZE, 1);
    assert_eq!(
        <G2ProductShape<1> as ConstrainedTypeShape>::CYCLE_SIZE,
        65_536
    ); // 256^2
    assert_eq!(
        <G2ProductShape<3> as ConstrainedTypeShape>::CYCLE_SIZE,
        281_474_976_710_656, // 256^6 = 2^48
    );
}

#[test]
fn cycle_size_saturates_at_and_above_the_threshold() {
    // At SITE_COUNT ≥ 8 the value saturates to u64::MAX (ADR-032), and
    // stays there for every larger width — including MiB-class — without
    // overflow or panic.
    assert_eq!(
        <F4QuotientShape<8> as ConstrainedTypeShape>::CYCLE_SIZE,
        u64::MAX
    );
    // E₆ saturates at COMPONENT_LABEL_BYTES = 7 (SITE_COUNT = 8) —
    // one width earlier than the operand-preserving unaries because
    // of its structure-preserving N + 1 width.
    assert_eq!(
        <E6FiltrationShape<7> as ConstrainedTypeShape>::CYCLE_SIZE,
        u64::MAX
    );
    assert_eq!(
        <E6FiltrationShape<71> as ConstrainedTypeShape>::CYCLE_SIZE,
        u64::MAX
    );
    assert_eq!(
        <E8EmbeddingShape<16_777_216> as ConstrainedTypeShape>::CYCLE_SIZE,
        u64::MAX,
    );
    // Binary product saturates from N = 4 (SITE_COUNT = 8) upward.
    assert_eq!(
        <G2ProductShape<4> as ConstrainedTypeShape>::CYCLE_SIZE,
        u64::MAX
    );
    assert_eq!(
        <G2ProductShape<1_048_576> as ConstrainedTypeShape>::CYCLE_SIZE,
        u64::MAX,
    );
}

// ---- Arity scales by iteration, not by a per-shape arity cap ----
//
// ADR-061 fixes each shape's arity by its operation's algebra (G₂
// binary, the rest unary). Wider compositions are not blocked — they
// **iterate** via `ConstraintRef::Recurse` per ADR-057: a balanced
// composition tree over K leaf κ-labels has K−1 internal binary G₂
// nodes, each a `G2ProductShape<N>`. The arity bound is the
// application's declared recursion-descent depth, not a ceiling baked
// into the shape. This models that the node arithmetic scales with K
// for arbitrarily large K.

/// Internal binary-product nodes needed to compose `k` leaf κ-labels:
/// `k − 1` for any `k ≥ 1` (a binary tree with `k` leaves has `k − 1`
/// internal nodes).
const fn internal_g2_nodes(k: usize) -> usize {
    k - 1
}

#[test]
fn arbitrary_arity_decomposes_into_iterated_binary_products() {
    // Each internal node is the *same* shape — G2ProductShape<N> — so a
    // composition of any arity reuses one admitted typed-input shape; the
    // arity lives in the tree, not in a widening shape parameter.
    const N: usize = 71;
    validate_constrained_type(G2ProductShape::<N>).expect("the binary product node admits once");

    // The node count scales linearly with arity, unbounded: arity 2
    // needs 1 node, arity 3 needs 2, arity one-million needs 999_999.
    assert_eq!(internal_g2_nodes(2), 1);
    assert_eq!(internal_g2_nodes(3), 2);
    assert_eq!(internal_g2_nodes(8), 7);
    assert_eq!(internal_g2_nodes(1_000_000), 999_999);

    // Strict monotonicity over a wide arity spread — no plateau implying
    // a hidden maximum arity at the shape layer.
    let arities = [2usize, 3, 8, 64, 4096, 1_000_000];
    let mut prev = internal_g2_nodes(arities[0]);
    for &k in &arities[1..] {
        let nodes = internal_g2_nodes(k);
        assert_eq!(nodes, k - 1);
        assert!(nodes > prev, "node count must grow with arity, never clamp");
        prev = nodes;
    }
}

// ---- Distinctness is preserved across the full scale range ----

#[test]
fn distinct_widths_remain_distinct_shapes_at_scale() {
    // Identity-via-(IRI, SITE_COUNT, CONSTRAINTS) per ADR-017: same IRI,
    // adjacent widths ⇒ distinct SITE_COUNT, even at MiB scale.
    assert_ne!(
        <F4QuotientShape<16_777_216> as ConstrainedTypeShape>::SITE_COUNT,
        <F4QuotientShape<16_777_215> as ConstrainedTypeShape>::SITE_COUNT,
    );
    // E₆'s structure-preserving N + 1 formula also gives distinct
    // widths at adjacent COMPONENT_LABEL_BYTES, even at MiB scale.
    assert_ne!(
        <E6FiltrationShape<16_777_216> as ConstrainedTypeShape>::SITE_COUNT,
        <E6FiltrationShape<16_777_215> as ConstrainedTypeShape>::SITE_COUNT,
    );
    // E₆ at width N produces SITE_COUNT = N + 1, structurally
    // distinct from F₄/E₇/E₈ at the same N (SITE_COUNT = N), so the
    // four unary shapes at the same component-label width are not all
    // numerically equal — E₆ is one byte wider.
    assert_ne!(
        <E6FiltrationShape<71> as ConstrainedTypeShape>::SITE_COUNT,
        <F4QuotientShape<71> as ConstrainedTypeShape>::SITE_COUNT,
    );
    // The shared closure IRI is width-invariant — it identifies the
    // family, not the instance — across the whole range.
    assert_eq!(
        <G2ProductShape<1> as ConstrainedTypeShape>::IRI,
        <G2ProductShape<16_777_216> as ConstrainedTypeShape>::IRI,
    );
    assert_eq!(
        <E6FiltrationShape<1> as ConstrainedTypeShape>::IRI,
        <E6FiltrationShape<16_777_216> as ConstrainedTypeShape>::IRI,
    );
    assert_eq!(
        <RouteShape<1, 1, 1, 1, 1> as ConstrainedTypeShape>::IRI,
        <RouteShape<1_048_576, 4096, 256, 1_048_576, 1_048_576> as ConstrainedTypeShape>::IRI,
    );
}
