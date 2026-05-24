# Repo definition — `UOR-Foundation/prism`

This document is the canonical definition of this repository. Anything in
the working tree that contradicts this file is a bug in the working tree;
anything missing from this file is out of scope.

## 1. Purpose

This repository is the source and publishing pipeline for the **Prism
standard library** (wiki ADR-031): a façade crate (`uor-prism`) that
re-exports the `uor-foundation` substrate together with the built-in
axes and built-in types its Layer-3 sub-crates declare, plus a
replay-only sibling (`uor-prism-verify`). Together these realize the
**Prism** system specified by the [UOR-Framework wiki][wiki]:

| Cargo package         | Library (import) name | Role                                                                                                                 |
|-----------------------|-----------------------|----------------------------------------------------------------------------------------------------------------------|
| `uor-prism`           | `prism`               | Standard-library façade. Re-exports foundation substrate + SDK macros + every Layer-3 sub-crate (wiki ADR-031)       |
| `uor-prism-verify`    | `prism_verify`        | Replay façade for verifiers (wiki ADR-005)                                                                           |
| `uor-prism-crypto`    | `prism_crypto`        | Layer-3 sub-crate: `HashAxis` + `CurveAxis` + `SignatureAxis` + `CommitmentAxis` per wiki ADR-031                    |
| `uor-prism-numerics`  | `prism_numerics`      | Layer-3 sub-crate: `BigIntAxis` + `FixedPointAxis` + `FieldAxis` + `RingAxis` per wiki ADR-031                       |
| `uor-prism-tensor`    | `prism_tensor`        | Layer-3 sub-crate: `TensorAxis` + `ActivationAxis` per wiki ADR-031                                                  |
| `uor-prism-fhe`       | `prism_fhe`           | Layer-3 sub-crate: `FheAxis` per wiki ADR-031                                                                        |

The `uor-` prefix on the package names is forced because the bare name
`prism` on crates.io is already occupied by an unrelated crate. Inside
Rust source, the import path and module names track wiki nomenclature
exactly: `use prism::pipeline::run;`, `use prism::crypto::Sha256Hasher;`,
`use prism_verify::certify_from_trace;`.

Per wiki ADR-031's façade commitment, application authors depend on
`uor-prism` alone — the four Layer-3 sub-crates are re-exported through
`prism::crypto`, `prism::numerics`, `prism::tensor`, `prism::fhe` so
they reach every standard-library axis without adding additional deps.

The substrate crates `uor-foundation` and `uor-foundation-sdk` are
consumed unmodified as normal crates.io dependencies. This repository
does not fork or vendor them.

[wiki]: https://github.com/UOR-Foundation/UOR-Framework/wiki

## 2. Authoritative architecture

The architecture is defined externally at the [UOR-Framework wiki][wiki]
in arc42 + C4 form. The wiki is normative; this repository is its
implementation. Code in this repository must satisfy:

- **Architecture constraints** TC-01 through TC-06
  (zero-cost runtime, sealing, singular principal data path,
  bilateral compile-time enforcement, replayability without deciders or
  hashing, no application-author infrastructure) — see wiki page 02
- **Quality scenarios** QS-01 through QS-05 — see wiki page 10
- **Architecture decision records** ADR-001 through ADR-055 —
  see wiki page 09. The most architecturally load-bearing recent
  additions: **ADR-018** (`HostBounds` capacity completeness — third
  substitution axis); **ADR-019** (foundation is a closed signature
  endofunctor, `Term` is its initial algebra, `pipeline::run` is the
  catamorphism); **ADR-020** (`PrismModel` is the application author's
  typed-iso contract, sealed by foundation, derived by the
  `prism_model!` macro from `uor-foundation-sdk`); **ADR-022**
  (`PrismModel` implementation surface decisions, including `run_route`
  as the canonical model-execution entry point); **ADR-023**
  (`M::Input`/`M::Output` value flow into the `CompileUnit` binding
  table via `IntoBindingValue`); **ADR-024** (three-layer algebraic
  closure: substrate, prism, implementation — verbs and axes are the
  Layer-3 surface); **ADR-030** (the `axis!` SDK macro is the
  universal substrate-extension declaration mechanism replacing the
  prior single `Hasher` lane); **ADR-031** (**`prism` IS the
  standard library** — a façade re-exporting `uor-foundation` plus
  Layer-3 sub-crates `prism-crypto`, `prism-numerics`, `prism-tensor`,
  `prism-fhe`); **ADR-032** (`CYCLE_SIZE` associated const on
  `ConstrainedTypeShape` for compile-time domain-cardinality
  introspection); **ADR-035** (canonical ψ-pipeline plus ψ-chain
  `Term` variants and ψ-residuals discipline); **ADR-036**
  (`ResolverTuple` substrate parameter on `PrismModel`/`run_route`
  carrying the eight categorical-machinery resolvers — Nerve,
  ChainComplex, HomologyGroup, CochainComplex, CohomologyGroup,
  Postnikov, HomotopyGroup, KInvariant — with `NullResolverTuple` as
  the default); **ADR-037** (`HostBounds`-parametric capacity bounds
  completing ADR-018's commitment); **ADR-043** (iterative-resolution
  discipline for resolver-internal bounded-search convergence);
  **ADR-044** (`PartitionProductFields` trait for product-shape field
  metadata); **ADR-045** (`Grounded::tag::<NewTag>()` zero-cost
  re-tagging); **ADR-047** (σ-projection hardening U1–U6 axioms on
  canonical-hash axes); **ADR-040** (closed 7-individual `BoundShape`
  catalog with `type:LexicographicLessEqBound` for byte-sequence
  observables — 1:1 correspondence with the foundation-published
  `ObservablePredicate` impl surface of ADR-049); **ADR-048**
  (`TypedCommitment` substrate as the 5th model-declaration
  parameter — zero-cost typed-bandwidth admission composition;
  closed three-impl set `EmptyCommitment` / `SingletonCommitment<P>` /
  `AndCommitment<A, B>` plus canonical
  `TargetCommitment = SingletonCommitment<LexicographicLessEqThreshold>`
  alias); **ADR-049** (five foundation-published typed UOR observable
  primitives — `Stratum<P>`, `WalshHadamardParity`,
  `UltrametricCloseTo<P>`, `AffineParity`, `LexicographicLessEqThreshold` —
  closing the catalog correspondence with ADR-040 via four taxonomy
  subclasses of ADR-038's closed `observable:Observable` catalog:
  `StratumObservable`, `SpectralObservable`, `MetricObservable`,
  `ValueThresholdObservable`); **ADR-050**
  (width-parametric arithmetic fold-rules — the catamorphism
  evaluates `PrimitiveOp::{Add, Sub, Mul, Neg, Bnot, Succ, Pred, Xor,
  And, Or, Div, Mod, Pow}` at the full Witt tower, no longer
  truncating wide operands to u64); **ADR-051** (`Term::Literal`
  `value` is now a `TermValue` byte-sequence per wide-Witt-level
  literals — see [`prism::operation::TermValue::from_u64_be`]);
  **ADR-052** (the `axis!` SDK macro emits a `@generic` companion
  form, so parametric Layer-3 axes inherit the
  `AxisExtension::dispatch_kernel` body from the macro instead of
  duplicating it as hand-written impls); **ADR-053** (`PrimitiveOp`
  catalog gains `Div`, `Mod`, `Pow` as substrate primitives — see
  the doctest in [`prism::operation`] for the updated
  exhaustive-match); **ADR-054** (the Fold-Fusion Principle — every
  prism transformation is a folding operation; the catamorphism
  fuses composed folds by universal property); **ADR-055**
  (universal substrate-Term verb body discipline — the
  foundation-declared `SubstrateTermBody` supertrait on
  `AxisExtension` makes the substrate-Term verb body discipline
  apply to every axis impl, not just standard-library canonical
  impls per the previous ADR-054 RA2 carve-out); **ADR-056**
  (ψ-residuals discipline scope refinement — the discipline
  applies to the route body's syntactic surface ONLY; verb bodies
  and axis impl bodies admit the full substrate vocabulary
  including `concat`, `le`/`lt`/`ge`/`gt`, `hash(...)` axis
  invocation, and `first_admit`, unblocking canonical decompositions
  for SHA padding, HMAC, Merkle tree construction, and tensor
  saturation per ADR-054 + ADR-055); **ADR-057** (bounded recursive
  structural typing via `ConstraintRef::Recurse { shape_iri,
  descent_bound }` + the foundation `shape_iri_registry` module
  surface — `RegisteredShape`, `ShapeRegistryProvider`,
  `EmptyShapeRegistry`, `lookup_shape`, `lookup_shape_in` — plus the
  `register_shape!` SDK macro and the `partition_product!` /
  `partition_coproduct!` operand grammar admitting
  `recurse[(<bound>)]:T`. A `Recurse`-bearing shape declares
  `CYCLE_SIZE = u64::MAX` (saturation per ADR-032). The
  registry-aware nerve / Betti substrate primitives shipped in
  foundation 0.4.15 — `primitive_simplicial_nerve_betti_in::<T, R>`,
  `primitive_cartesian_nerve_betti_in::<S, R>`, and
  `expand_constraints_in::<R>` — walk `Recurse` entries through `R`'s
  registry plus foundation's built-in registry, giving the
  structurally-correct nerve / Betti reading of recursively-expanded
  constraint sets; the non-`_in` siblings
  (`primitive_simplicial_nerve_betti::<T>`,
  `primitive_cartesian_nerve_betti::<S>`) are also surfaced through the
  prism façade. Wire-format trace events gain a `Recurse` discriminant;
  `TRACE_REPLAY_FORMAT_VERSION` bumps 9 → 10); **ADR-058** (κ-derivation
  — the eight-resolver ψ-pipeline composed with the ψ_9 σ-projection —
  **is** the framework's compression-to-canonical-form operator; the
  three-tier closure-lossless taxonomy T1 byte-identical ⇒ T2
  κ-label-identical ⇒ T3 outcome-coarse-equivalent. Conceptual-reading
  commitment over existing constructs — no new substrate surface);
  **ADR-059** (the operator-geometry codomain of κ-derivation is the
  Atlas image inside E₈, coarsely stratified by the **Hopf convergence
  tower** — the foundation's `kernel::convergence` namespace: four
  `ConvergenceLevel` instances R / C / H / O at division-algebra
  dimensions {1, 2, 4, 8} with Hopf fibers S⁰ / S¹ / S³ / S⁷ and
  characteristic identities existence / feedback / choice /
  self-reference. Conceptual-reading commitment over the foundation's
  **existing** `kernel::convergence` substrate vocabulary — no new
  substrate surface; the prism façade surfaces it through the
  `prism::convergence` module so application authors building ADR-059
  codomain-typed `TypedCommitment` predicates reach it through `prism`
  per ADR-031); **ADR-060** (source-polymorphic value carrier —
  `TermValue` becomes the const-generic enum
  `TermValue<'a, INLINE_BYTES> { Inline, Borrowed, Stream(&dyn ChunkSource) }`
  admitting bounded inline, zero-copy borrowed, and unbounded streamed
  payloads; the 12 fictional byte-width caps and the foundation-provided
  `DefaultHostBounds` are **removed**; `HostBounds` shrinks 26 → 14
  associated consts and per-carrier widths derive from the
  application's declared structural-count primitives via foundation
  `const fn`s (`carrier_inline_bytes::<B>()`, the per-ψ-stage
  `*_carrier_bytes::<B>()`). `Term`, `CompileUnitBuilder`, `Grounded`,
  `Sinking`, and `run` gain the `INLINE_BYTES` const parameter. Wire
  format byte-identical — `TRACE_REPLAY_FORMAT_VERSION` stays 10, no
  shim. **There is no default `HostBounds`: every application — including
  prism's own test suite — declares its own impl** (the standard library
  re-exports the `HostBounds` trait but provides no concrete impl, per
  ADR-060's "no default that hides a choice"). Foundation 0.5.1
  completes the input side: `IntoBindingValue` gains a `'a` lifetime and
  replaces the `MAX_BYTES` const + `into_binding_bytes` writer with
  `as_binding_value<INLINE_BYTES>(&self) -> TermValue<'a, INLINE_BYTES>`,
  so an input returns the source-polymorphic carrier directly and
  `run_route` consumes it with no byte-width cap. prism's stdlib shapes
  are zero-sized markers, so each `as_binding_value` returns
  `TermValue::empty()`; `PrismModel` / `Grounded` / `run_route` gain the
  same `'a` lifetime); **ADR-061** (operational composition surface for
  κ-labels — each of ADR-059's five categorical operations on the Atlas
  image inside E₈ **is** a `ConstrainedTypeShape` in prism's standard
  type library: `G2ProductShape<N>` (binary product, `SITE_COUNT = 2×N`),
  the operand-preserving unary shapes `F4QuotientShape<N>` /
  `E7AugmentationShape<N>` / `E8EmbeddingShape<N>` (`SITE_COUNT = N`),
  and the structure-preserving unary `E6FiltrationShape<N>`
  (`SITE_COUNT = N + 1` — the one-byte degree-partition tag prepended
  to operand bytes per wiki ADR-061 §(2)),
  parametric over the component-label byte width. Arity is fixed by the
  operation's algebra, not an application const; arity > 2 iterates via
  `ConstraintRef::Recurse` per ADR-057 (T = 3 / O = 8 bounds per
  ADR-025). Compositions are content-addressing realizations — the
  composed κ-label is itself a κ-label, recursively composable, closed
  under the Atlas; no new substrate primitive. Prism-runtime-level: no
  foundation/SDK changes — the shapes satisfy the standard-type-library
  inclusion criteria per ADR-031, the canonicalize verb is
  realization-architect work. The same change adds the decentralized
  publication-graph shapes `RouteShape<5 widths>` / `RevocationShape<6
  widths>` (route-declaration use case per ADR-061's consequences),
  empty-`CONSTRAINTS` typed-distinction markers whose `SITE_COUNT` is the
  sum of their per-component κ-label/endpoint/time-pair byte widths).

Substitution axes (the only permitted variation points per ADR-007 /
ADR-030 / ADR-036 / ADR-048): `HostTypes`, `HostBounds`, `AxisTuple`,
`ResolverTuple`, `TypedCommitment`. Per ADR-060 the foundation supplies
no `DefaultHostBounds`; the application declares its `HostBounds` impl
explicitly (prism re-exports the trait, not a default).

**Input-size discipline (ADR-060, completed in foundation 0.5.1).**
Per ADR-060 the byte width of a value carrier is an application
concern; **large inputs are content-addressed by their hash, not
materialized**. Foundation 0.5.1 completed the input path:
`IntoBindingValue::as_binding_value<INLINE_BYTES>` returns the
source-polymorphic `TermValue<'a, INLINE_BYTES>` carrier
(`Inline` within the derived inline width, `Borrowed` zero-copy for
larger in-memory values, `Stream` for unbounded sources), and
`run_route` consumes it directly with **no `INLINE_BYTES` cap** (the
pre-0.5.1 `MAX_BYTES`-overflow rejection is gone). So an input shape
whose `as_binding_value` returns `Borrowed`/`Stream` flows through the
convenience `prism_model!` path unbounded (model-weight container
formats, multi-GB tensor-data sections, large canonical-JSON).
Independently, a large input can be content-addressed by hash and
bound directly: stream-hash it through the application's `Hasher`
(`fold_bytes`, chunk-by-chunk, never materialized), set the
leading-8-byte digest as an input-slot `Binding`'s `content_address`,
and drive `run` over a `CompileUnitBuilder` whose root term is
`Term::Variable { name_index: 0 }`. `Binding` is re-exported through
`prism::vocabulary`; the replay-verified worked example is
`crates/uor-prism/tests/large_input_grounding.rs`.

## 3. Layout

Per wiki ADR-031 (`prism` is the standard library), the `prism`
façade crate sits alongside the standard-library Layer-3 sub-crates
that contribute the built-in axes and built-in types it re-exports.

```
.
├── AGENTS.md                          # this file (canonical repo definition)
├── CONTRIBUTING.md                    # human-facing pointer to AGENTS.md
├── Cargo.toml                         # workspace manifest, shared profile + lints
├── LICENSE                            # MIT
├── README.md                          # public-facing overview
├── crates
│   ├── uor-prism                      # the standard-library façade (wiki ADR-031)
│   │   ├── Cargo.toml                 # package = uor-prism, lib.name = prism
│   │   └── src/lib.rs                 # re-exports foundation + every Layer-3 sub-crate
│   ├── uor-prism-verify               # replay-only façade (wiki ADR-005)
│   │   ├── Cargo.toml                 # package = uor-prism-verify, lib.name = prism_verify
│   │   └── src/lib.rs
│   ├── uor-prism-crypto               # Layer-3 sub-crate (wiki ADR-031)
│   │   ├── Cargo.toml                 # package = uor-prism-crypto, lib.name = prism_crypto
│   │   ├── src/                       # HashAxis + CurveAxis + SignatureAxis + CommitmentAxis
│   │   └── tests/conformance.rs       # FIPS-180-4 + FIPS-202 + BLAKE3 vectors
│   ├── uor-prism-numerics             # Layer-3 sub-crate (wiki ADR-031)
│   │   ├── Cargo.toml                 # package = uor-prism-numerics, lib.name = prism_numerics
│   │   ├── src/                       # BigIntAxis + FixedPointAxis + FieldAxis + RingAxis
│   │   └── tests/conformance.rs
│   ├── uor-prism-tensor               # Layer-3 sub-crate (wiki ADR-031)
│   │   ├── Cargo.toml                 # package = uor-prism-tensor, lib.name = prism_tensor
│   │   ├── src/                       # TensorAxis + ActivationAxis
│   │   └── tests/conformance.rs
│   └── uor-prism-fhe                  # Layer-3 sub-crate (wiki ADR-031)
│       ├── Cargo.toml                 # package = uor-prism-fhe, lib.name = prism_fhe
│       ├── src/                       # FheAxis + reference one-time-pad impl
│       └── tests/conformance.rs
├── tools
│   └── wiki-link-check                # internal CI binary, publish = false
│       ├── Cargo.toml
│       └── src
│           ├── main.rs                # CLI entrypoint
│           ├── slug.rs                # github-slugger algorithm
│           ├── scan.rs                # source/markdown URL scanner
│           └── wiki.rs                # wiki repo cloner + header parser
├── docs/                              # C4 diagrams, assets referenced from rustdoc
├── deny.toml                          # cargo-deny config
├── justfile                           # task runner shortcuts
├── rust-toolchain.toml                # pinned to MSRV 1.83 stable
├── rustfmt.toml
└── .github/workflows/
    ├── ci.yml                         # PR + push: fmt, clippy, test, doc, no_std, wiki-links, deny
    ├── release.yml                    # tag-driven cargo publish
    ├── docs.yml                       # rustdoc → GitHub Pages
    └── wiki-drift.yml                 # weekly cron: wiki-link-check against wiki HEAD
```

## 4. Toolchain

- **Rust edition**: 2021
- **MSRV**: 1.83 (matches the `rust-version` declared by every
  released `uor-foundation` since v0.3.1 — currently still 1.83 in
  v0.4.6 — which corrected v0.3.0's stale 1.81 declaration).
  Pinned via `rust-toolchain.toml`, which the Rust toolchain enforces
  on every cargo invocation in this workspace. Per
  [TR-09](https://github.com/UOR-Foundation/UOR-Framework/wiki/11-Technical-Risks#tr-09--prism-version-pin-lag-against-uor-foundation),
  `prism`'s pin on `uor-foundation` may lag the latest published
  version; updates to this repo are demand-driven (a needed surface
  change) rather than calendar-driven.
- **`uor-foundation`**: `^0.5` (effective floor 0.5.2 — generalizes the
  resolver/pipeline tower over the fingerprint width `FP_MAX`: the
  `AxisTuple` blanket impl is now
  `impl<INLINE_BYTES, FP_MAX, H: Hasher<FP_MAX>>`, the eight ψ-stage
  resolver traits take an unbounded `H`, and `run` / `run_route` /
  `Grounded` / `PrismModel` / `certify_from_trace` carry `FP_MAX` as a
  const parameter (`Hasher<const FP_MAX = 32>`). 0.5.1 had pinned the
  whole tower to `Hasher<32>`, so a 64-byte-fingerprint hasher
  (`Sha512Hasher: Hasher<64>`) could not flow through the pipeline at
  all — fixed in 0.5.2 (regression-tested in
  `crates/uor-prism/tests/wide_hasher_pipeline.rs`). Earlier floor 0.5.1
  completed the ADR-060 input path: `IntoBindingValue` gains a `'a`
  lifetime and replaces the `MAX_BYTES` const + `into_binding_bytes`
  writer with
  `as_binding_value<INLINE_BYTES>(&self) -> TermValue<'a, INLINE_BYTES>`,
  returning the source-polymorphic carrier directly so `run_route`
  admits arbitrarily large inputs with no byte-width cap;
  `PrismModel` / `Grounded` / `run_route` gain the `'a`. Earlier
  floor 0.5.0 introduced the ADR-060 source-polymorphic value carrier:
  `TermValue` becomes the const-generic enum
  `TermValue<'a, INLINE_BYTES>` with
  `Inline`/`Borrowed`/`Stream(&dyn ChunkSource)` variants; the 12
  byte-width capacity caps and the foundation-provided
  `DefaultHostBounds` are removed; `HostBounds` shrinks 26 → 14
  associated consts; per-carrier widths derive from the application's
  structural-count primitives via `carrier_inline_bytes::<B>()` and the
  per-ψ-stage `*_carrier_bytes::<B>()` const fns; `Term`,
  `CompileUnitBuilder`, `Grounded`, `Sinking`, and `run` gain the
  `INLINE_BYTES` const parameter. Wire format byte-identical
  (`TRACE_REPLAY_FORMAT_VERSION` stays 10); MSRV stays 1.83. Earlier
  floors: 0.4.15 shipped the complete ADR-057 registry-aware
  substrate-primitive surface:
  `enforcement::expand_constraints_in::<R>`,
  `enforcement::primitive_simplicial_nerve_betti_in::<T, R>`, and
  `pipeline::primitive_cartesian_nerve_betti_in::<S, R>` walk
  `ConstraintRef::Recurse` entries through `R`'s registry plus
  foundation's built-in registry, closing the ADR-057 nerve / Betti
  reading of recursively-expanded constraint sets;
  0.4.14 shipped the foundational ADR-057 surface — the
  `ConstraintRef::Recurse { shape_iri, descent_bound }` variant + the
  `pipeline::shape_iri_registry` module surface (`RegisteredShape`,
  `ShapeRegistryProvider`, `EmptyShapeRegistry`, `lookup_shape`,
  `lookup_shape_in`) + the `TRACE_REPLAY_FORMAT_VERSION` bump 9 → 10
  with the wire-format `Recurse` discriminant.
  Earlier floors: 0.4.12 shipped the ADR-049 5th
  `ObservablePredicate` impl `LexicographicLessEqThreshold` plus its
  `observable:ValueThresholdObservable` taxonomy subclass realizing
  ADR-040's `type:LexicographicLessEqBound` catalog primitive, and
  the ADR-048 canonical search-cost commitment alias
  `TargetCommitment = SingletonCommitment<LexicographicLessEqThreshold>`;
  0.4.11 shipped the `partition_product!` macro's
  `syn::Type` operand admission closing the const-generic-leaf
  depth-2 verb!-macro projection-chain gap (the last remaining
  structural blocker for the three-operand canonical numerics roster
  `fma` / `mod_pow` / parametric `field_*`); 0.4.10 shipped the
  ADR-056 ψ-residuals scope refinement plus `literal_u64`/`literal_bytes`
  wide-Witt embedding plus depth-2 partition-product field access
  for hand-rolled leaves; 0.4.9 admitted `div`/`r#mod`/`pow` as
  verb-body call forms plus the `axis!` `body = |input| { … };`
  clause grammar per ADR-053 + ADR-055;
  Earlier floors: the `SubstrateTermBody` supertrait on
  `AxisExtension` per ADR-055 (0.4.8 floor);
  width-parametric arithmetic fold-rules per ADR-050; wide-value
  carrier on `Term::Literal` per ADR-051; `PrimitiveOp::{Div, Mod,
  Pow}` per ADR-053; `C: TypedCommitment` on
  `PrismModel`/`run_route` per ADR-048; `CYCLE_SIZE` on
  `ConstrainedTypeShape` per ADR-032; `R: ResolverTuple` substrate
  parameter per ADR-035/036; new
  `PrimitiveOp::{Le, Lt, Ge, Gt, Concat}` per ADR-026;
  `Output: IntoBindingValue` per ADR-023 value-flow expansion.
  `default-features = false`, `no_std`-clean.
- **`uor-foundation-sdk`**: `^0.5` (effective floor 0.5.2 — tracks
  the foundation 0.5.2 `FP_MAX` tower generalization; the
  `axis!` / `verb!` / `prism_model!` / `partition_product!` /
  `register_shape!` macro names and grammar are unchanged, but the
  `prism_model!`-emitted impls now carry the `FP_MAX` const parameter
  alongside `INLINE_BYTES` and the `IntoBindingValue<'a>` lifetime
  (0.5.1). Earlier floors: 0.4.15 added the optional
  `resolver!` macro `shape_registry: MyRegistry` clause that wires an
  application's `ShapeRegistryProvider` marker into the emitted
  `ResolverTuple` impl as the `ShapeRegistry` associated type;
  `prism::pipeline` does not invoke `resolver!`, so the new clause is
  purely a downstream-application surface. Earlier floors: 0.4.14
  shipped ADR-057's `register_shape!(Registry, S1, S2, …)` macro
  emitting a `ShapeRegistryProvider` impl with a const-aggregated
  `REGISTRY` slice, plus the `partition_product!` /
  `partition_coproduct!` operand grammar admitting
  `recurse[(<bound>)]:T` markers that lower to
  `ConstraintRef::Recurse` instead of inlining the target's
  CONSTRAINTS — closing the const-eval cycle for recursive shapes.
  Earlier floors: 0.4.12 tracked the foundation 0.4.12 release that
  closes the ADR-040 / ADR-048 / ADR-049 catalog correspondence
  (purely additive at 0.4.12, no macro grammar changes);
  0.4.11 shipped the `partition_product!` macro's `syn::Type`
  operand admission per the const-generic-leaf depth-2 verb!-macro
  projection-chain fix; 0.4.10 shipped the ADR-056 ψ-residual scope
  refinement (admitting `concat`/`hash`/ordered-comparison ops in
  verb/axis bodies) and `literal_u64`/`literal_bytes` wide-Witt
  embedding; 0.4.9 admitted
  `div`/`r#mod`/`pow` as verb-body call forms plus the `axis!`
  `body` clause grammar; 0.4.8 declared the `SubstrateTermBody`
  supertrait;
  the `axis!` macro's `@generic` companion-emission form per ADR-052;
  the SDK macros `prism_model!`, `verb!`, `axis!`, `resolver!`,
  `output_shape!`, `use_verbs!`, `product_shape!`, `coproduct_shape!`,
  `cartesian_product_shape!`, `partition_product!`,
  `partition_coproduct!` per ADR-031. Re-exported through
  `prism::pipeline` so application authors reach the canonical SDK
  macro surface through the single `prism` dep.
- **Backing crates for standard-library Layer-3 sub-crates** (per
  ADR-031's `prism-crypto` roster of canonical impls):
  `sha2 = "0.10"`, `sha3 = "0.10"`, `blake3 = "1.5"` (pinned to
  the 1.5 line; the 1.8+ line transitively requires
  `constant_time_eq 0.4` which needs Rust edition 2024 / MSRV 1.85
  per Cargo.lock).
- **Workspace resolver**: `"2"`
- **Release profile** (per QS-01): `opt-level = 3`, `lto = true`, `codegen-units = 1`
- **`#![no_std]` posture**: default for both crates; `std` and `alloc`
  are opt-in features, mirroring `uor-foundation`

## 5. Documentation-driven, behavior-driven development

The rustdoc surface IS the C4 view of the system. To make that load-bearing:

### 5.1 Required structure for every `pub` item

Every **first-class** public item in `uor-prism` and `uor-prism-verify`
(modules, constants, types, functions, traits declared in this
repository) carries:

1. **One-line brief** as the first paragraph (rustdoc summary line).
2. **`# See also`** section with at least one verified backlink to the
   precise wiki page and section anchor that defines the item.
3. **`# Constraints`** section listing every applicable normative
   identifier (`TC-0X`, `QS-0X`, `ADR-NNN`).
4. **`# C4 placement`** at module scope: which C4 level and component
   the module realizes, mirroring wiki page 05 (Building Block View)
   one-to-one.
5. **`# Behavior`** doctest framed as Given / When / Then comments
   inside a ` ```rust ` block. Doctests are the executable behavior
   spec — they run in CI as part of `cargo test --workspace`.

**Re-exports** of `uor-foundation` items inherit the foundation's
rustdoc verbatim — they do not get a second copy of the five-block
structure here. The structure attaches to the **module** that re-exports
them, which describes which wiki section the re-exports realize and
why each one is included. This matches ADR-013 (closure of `prism`
under `uor-foundation`): the substrate is the source of truth for the
items themselves, and `prism` is the source of truth for their
architectural placement.

### 5.2 Module hierarchy ↔ wiki components

Module names mirror the Level 2 components named in
[wiki page 05 § Whitebox `prism`][05-prism] and
[wiki page 05 § Whitebox `prism-verify`][05-verify].

In `prism` the modules are:
`pipeline`, `seal`, `replay`, `operation`, `std_types`,
`vocabulary` (foundation re-exports), plus the standard-library
Layer-3 sub-crate re-exports introduced by wiki ADR-031:
`crypto`, `numerics`, `tensor`, `fhe`. Adding a top-level module
that has no counterpart in the wiki is forbidden.

Each Layer-3 sub-crate (`uor-prism-crypto`, `uor-prism-numerics`,
`uor-prism-tensor`, `uor-prism-fhe`) declares its axis traits via
the `axis!` SDK macro per ADR-030; the axis trait declaration and
all its concrete impls live in a single module per Rust's
proc-macro-emitted `#[macro_export]` constraint (issue rust-lang
#52234 — companion macros are reachable only at the call site of
the original `axis!` invocation).

[05-prism]: https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism
[05-verify]: https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism-verify

### 5.3 Wiki backlink format

```
//! # See also
//!
//! - [Wiki: 05 Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
```

Anchors are computed by the GitHub anchor algorithm
(see § 6 below). The CI gate `wiki-link-check` rejects any URL whose
page or anchor does not exist in the wiki source repository.

## 6. Wiki backlink validation (`tools/wiki-link-check`)

A workspace binary (not published to crates.io) that enforces every
wiki backlink in the repository points at content that actually exists
in the wiki source.

### 6.1 What it does

1. Acquires a copy of the wiki source. By default `git clone --depth=1`
   from `https://github.com/UOR-Foundation/UOR-Framework.wiki.git` into
   a cache directory. Pinnable via `--wiki-rev <SHA>` or
   `PRISM_WIKI_REV` env var; usable against an already-cloned tree via
   `--wiki-path <DIR>`.
2. Walks the repository tree (default: current directory) and extracts
   every URL matching the pattern
   `https://github.com/UOR-Foundation/UOR-Framework/wiki/<page>(#<anchor>)?`
   from `*.rs`, `*.md`, and `*.toml` files.
3. For each URL:
   - Verifies that `<page>.md` exists in the wiki source.
   - If a `#<anchor>` is present, parses every ATX header
     (`# `, `## `, `### `, …) from that page, slugifies each header
     using the GitHub algorithm (§ 6.2), and confirms the anchor matches.
4. Exits 0 on full success; 1 on any broken link, with a report of
   `(file:line, broken_url, suggestion)` triples.

### 6.2 GitHub anchor algorithm (locked-in)

Identical to `github-slugger` (Ruby/JS). Given a header text:

1. Lowercase the text.
2. Remove every character that is not a Unicode word character,
   `-`, or ` `.
3. Replace each ` ` with `-`.
4. If the resulting slug has already appeared earlier in document order
   on the same page, append `-1`, `-2`, … until unique.

This is implemented in [`tools/wiki-link-check/src/slug.rs`](tools/wiki-link-check/src/slug.rs)
with golden-file unit tests against real wiki headers.

### 6.3 Where it runs

| Trigger                      | Workflow                | Behavior                              |
|------------------------------|-------------------------|---------------------------------------|
| Pull request and push        | `.github/workflows/ci.yml` (`wiki-links` job) | Hard-fails the build on broken backlinks |
| Weekly Mon 07:00 UTC + manual| `.github/workflows/wiki-drift.yml`            | Detects drift introduced by wiki edits, even when no PR is open |
| Local development            | `cargo run -p wiki-link-check` (or `just lint-wiki`) | Optional pre-commit hook |

## 7. CI gates (`.github/workflows/ci.yml`)

Every gate is required to merge.

| Gate         | Command                                                                   | Enforces                          |
|--------------|---------------------------------------------------------------------------|-----------------------------------|
| `fmt`        | `cargo fmt --all --check`                                                 | Formatting consistency            |
| `clippy`     | `cargo clippy --workspace --all-targets -- -D warnings`                   | Lint cleanliness                  |
| `test`       | `cargo test --workspace --all-features`                                   | Unit + doctests (BDD specs)       |
| `no-std`     | `cargo build -p uor-prism --target thumbv7em-none-eabihf --no-default-features` | Deployment view, `#![no_std]`     |
|              | `cargo build -p uor-prism-verify --target thumbv7em-none-eabihf --no-default-features` |                                   |
| `doc`        | `RUSTDOCFLAGS='-D rustdoc::broken_intra_doc_links -D rustdoc::missing_crate_level_docs' cargo doc --workspace --no-deps` | C4 view stays linkable          |
| `wiki-links` | `cargo run -p wiki-link-check`                                             | All wiki backlinks resolve        |
| `deny`       | `cargo deny check`                                                         | Licenses + advisories + sources   |

MSRV is enforced implicitly by `rust-toolchain.toml` (every cargo
command in the workspace runs against the pinned channel), not by a
dedicated CI gate.

## 8. Release pipeline (`.github/workflows/release.yml`)

Tag-driven on `v*`. Mirrors the `UOR-Foundation/UOR-Framework`
release pipeline (single `release` job, `dtolnay/rust-toolchain@stable`
toolchain action, `actions/cache@v4` cargo cache, `softprops/action-gh-release@v2`
GitHub Release creation, direct crates.io HTTP API for index-propagation
wait, `CARGO_REGISTRY_TOKEN` secret). Steps, in order:

1. **Tag validation**: the tag (`vX.Y.Z`) must match the workspace
   version pinned in `[workspace.package]`. Mismatched tags fail
   early before any CI work.
2. **CI matrix**: `cargo fmt --check`, `cargo clippy --workspace
   --all-targets --all-features -- -D warnings`, `cargo test
   --workspace --all-features`, `cargo build --target
   thumbv7em-none-eabihf --no-default-features` per crate (all six),
   `wiki-link-check`, `cargo doc -D warnings`. Any failure aborts.
3. **Per-crate dry-run** (`cargo publish --dry-run --allow-dirty`).
   Leaf crates (`numerics`, `crypto`, `fhe`) verify against direct
   deps. Non-leaf crates (`tensor`, `prism`, `prism-verify`) pass
   `--no-verify` since their workspace-path deps aren't yet on the
   registry; the actual publish step is the authoritative
   verification, gated by wait-for-index.
4. **GitHub Release**: `softprops/action-gh-release@v2` creates the
   release page with auto-generated notes plus a manifest of the six
   crates that will be published.
5. **Publish in dependency order** (per wiki ADR-031's layered
   graph):
   - `cargo publish -p uor-prism-numerics`
   - `cargo publish -p uor-prism-crypto`
   - `cargo publish -p uor-prism-fhe`
   - Wait for leaf sub-crates to appear on crates.io (direct HTTP
     query against `crates.io/api/v1/crates/<pkg>/<version>`,
     bypassing the runner's cached cargo registry which would be
     stale).
   - `cargo publish -p uor-prism-tensor` (depends on numerics).
   - Wait for tensor to appear.
   - `cargo publish -p uor-prism` (depends on all four sub-crates).
   - Wait for prism to appear.
   - `cargo publish -p uor-prism-verify` (depends on prism).

Secret: `CARGO_REGISTRY_TOKEN` is provided at the
`UOR-Foundation` GitHub-org level and inherits into this repo
automatically. No per-repo secret minting is required — the same
token that publishes `uor-foundation` and `uor-foundation-sdk`
also publishes the six `uor-prism*` crates. (Confirm the
crates.io account behind the token has publish rights to each
`uor-prism*` crate before the first tag.)

Permissions: `contents: write` on the workflow (required for
`softprops/action-gh-release@v2` to upload the release notes).

### 8.1 SemVer policy

The six `uor-prism*` crates share the workspace version and follow
[Cargo SemVer](https://doc.rust-lang.org/cargo/reference/semver.html)
for a `0.x` series: the **minor** component is the breaking-change
axis. Any release that removes or renames a `pub` item, removes a
re-export, or otherwise breaks source compatibility for a downstream
pinned to the prior `0.MINOR` bumps the **minor** (e.g. `0.1.4 →
0.2.0`); additive-only releases (new re-exports, new axes, new
shapes, foundation-floor bumps that don't change the prism surface)
bump the **patch** (e.g. `0.1.3 → 0.1.4`). The breaking change is
recorded in the release commit body. Precedent: `0.2.0` removed the
`prism::tensor` convenience aliases (`CpuI8Tensor4x4Matmul` etc.) and
the `MAX_TENSOR_DIM` / `MAX_ACTIVATION_LEN` caps in favor of the
ADR-037 `HostBounds` discipline — a `pub`-item removal, hence the
minor bump.

## 9. Documentation hosting (`.github/workflows/docs.yml`)

On push to `main`:

1. `cargo doc --workspace --no-deps`
2. Inject a redirect at `target/doc/index.html` pointing to `prism/index.html`
3. Publish to GitHub Pages

`docs.rs` will additionally publish per-version rustdoc on each release.

## 10. Hard rules

- No `unsafe` anywhere. The workspace forbids it via `unsafe_code = "forbid"`.
- No `todo!()`, no `unimplemented!()`, no `panic!("not implemented")`. If a
  feature is incomplete, do not merge it.
- No top-level module in `uor-prism` or `uor-prism-verify` without a
  matching wiki Level 2 component.
- No public item without the five-block doc structure from § 5.1.
- No wiki backlink that has not been validated by `wiki-link-check`.
- `Cargo.lock` is committed.

## 11. Standard type library policy

Per wiki ADR-031, the **Prism standard library** is realized as the
`prism` façade plus the Layer-3 sub-crates published from this
repository (`uor-prism-crypto`, `uor-prism-numerics`,
`uor-prism-tensor`, `uor-prism-fhe`). Each sub-crate's conformance
discipline is governed by ADR-031 itself (application-neutral within
domain, built on foundation primitives + lower sub-crates,
content-addressed per ADR-017, conformance-tested against canonical
reference vectors, compile-time stable, `#![no_std]`-clean) — that
discipline is enforced by the `axis!` SDK macro at proc-macro
expansion and the conformance test suites in each sub-crate's
`tests/conformance.rs`.

This section §11 covers a narrower sub-policy: the **baseline
primitive type catalog** in `prism::std_types`, which realizes the
wiki's [Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
component named "standard type library". Per ADR-017 the catalog is
content-addressed and evolves *operationally* — the wiki defines the
catalog's purpose and identity rules, not its specific contents. The
catalog exists so that `prism` consumers do not have to derive common
patterns from first principles every time they author a
`ConstrainedTypeShape`.

### 11.1 Inclusion criteria

A type belongs in `prism::std_types` if and only if all of the
following hold:

1. **Built on foundation primitives.** Its body uses only
   foundation-supplied vocabulary (`ConstrainedTypeShape`,
   `ConstraintRef`, the closed `PrimitiveOp` set, `pipeline` admission
   functions). No new traits, no operation logic, no resolver
   implementations.
2. **Application-neutral.** Reusable across multiple unrelated
   downstream applications. No type carries a single domain's
   assumptions (cryptocurrency, JSON-RPC, an organization's internal
   protocol, etc.).
3. **Content-addressed per closure (ADR-017 + § 11.3).** The
   `(IRI, SITE_COUNT, CONSTRAINTS)` triple deterministically encodes
   the shape's identity. Empty-`CONSTRAINTS` baseline types share the
   foundation's `https://uor.foundation/type/ConstrainedType` class IRI
   per the closure rule; future types with non-empty constraint
   declarations adopt the IRI dictated by their constraint structure
   under the same rule (never a prism-claimed sub-namespace, never
   derived from the Rust type name).
4. **Compile-time stable.** All admission decisions resolve at compile
   time via the const path (`validate_compile_unit_const`,
   `validate_constrained_type_const`); no runtime allocation, no runtime
   trait dispatch.
5. **`#![no_std]`-clean.** Compiles on `thumbv7em-none-eabihf` without
   `alloc` or `std`.

### 11.2 Exclusion criteria

The following are explicitly out of scope and remain downstream concerns:

- **Operation libraries** (ADR-014). Pre-implemented resolvers,
  deciders, computation strategies, or DSL macros.
- **Cryptographic substrates.** Concrete `Hasher` impls (BLAKE3,
  SHA-256, …). The `Hasher` trait is the third substitution axis per
  ADR-007; choosing one is the application's prerogative.
- **Domain-specific shapes.** Anything tied to a single application
  domain — a Bitcoin block-header shape, an Ethereum transaction
  shape, etc. These belong in domain crates that consume
  `uor-prism::std_types` as building blocks.
- **Speculative additions.** Types added to anticipate future demand
  without an observed downstream consumer.

### 11.3 IRI rule (closure under `uor-foundation`)

The wiki's
[Concepts § Closure Under uor-foundation](https://github.com/UOR-Foundation/UOR-Framework/wiki/08-Concepts#closure-under-uor-foundation)
states the rule directly: *"The IRI of every type `prism` ships is
content-deterministic in its constraint declaration — derived from
`uor-foundation`'s vocabulary, not from the Rust type name."* ADR-017's
**rejected alternative 1** reinforces this: prism does **not** claim a
separate IRI namespace; closure makes IRIs derivative, not
namespace-claimed.

The concrete consequence for `prism::std_types`:

- The IRI is **determined by the constraint declaration**, not by the
  Rust type name. Two stdlib types with identical
  `(SITE_COUNT, CONSTRAINTS)` shape ⇒ identical IRI ⇒ identical UOR
  content-address.
- Every prism stdlib type with empty `CONSTRAINTS` therefore shares the
  IRI `https://uor.foundation/type/ConstrainedType` — the foundation's
  ontology class for `ConstrainedTypeShape` instances. Instance
  identity flows through `(SITE_COUNT, CONSTRAINTS)`.
- The Rust type name is for the **developer**: `use prism::U32` is
  self-documenting. The IRI is for **content-addressing**: `U32` and
  `I32` have the same content-address because they have the same
  constraint declaration. Schema-import tools that emit `prism::Bytes32`
  produce traces that address consistently with any author-declared
  shape carrying the same constraints (ADR-017's closure clause).
- Rust types with **distinct** constraint declarations (different
  `SITE_COUNT` or non-empty `CONSTRAINTS`) produce distinct
  content-addresses through that constraint declaration, even when they
  share the IRI.

### 11.4 Growth policy

There are two growth tracks, distinguished by whether the type is a
*baseline primitive* every implementor reaches for or a more
specialized addition.

**Baseline primitives** are admissible without per-type demonstrated
demand because every implementor reaches for them — withholding them
would force every downstream to re-derive the same trivial boilerplate.
The baseline set is fixed at:

- The byte-paired integer family `U8`/`I8` through `U256`/`I256` (the
  complete set of byte-aligned widths up to 32 bytes).
- The IEEE float widths `F32` and `F64`.
- `Bool`.
- `Bytes<const N: usize>` and `Char`.
- `FixedSites<const N: usize>` (the structural building block under
  every other typed primitive).

Any addition outside this set follows the **specialized track** and
requires:

1. **Demonstrated need.** At least one downstream consumer that would
   author the same boilerplate from first principles in its absence.
   Speculation alone is not sufficient. Per
   [TR-08](https://github.com/UOR-Foundation/UOR-Framework/wiki/11-Technical-Risks#tr-08--vocabulary-insufficiency-in-uor-foundation-forces-cross-repo-amendment-cadence),
   if the demand exposes a vocabulary insufficiency in `uor-foundation`
   itself (e.g., a needed `ConstraintRef` variant the foundation does
   not yet ship), file the gap upstream rather than papering over it
   with a prism-side workaround.
2. **Inclusion criteria satisfied** (§ 11.1).
3. **PR contents:** the new type with the five-block doc structure
   (§ 5.1), an integration test exercising the type end-to-end through
   `pipeline::run` and `certify_from_trace`, and verified wiki
   backlinks.
4. **Catalog entry** added to § 11.6 below.

Stdlib types are stable from inclusion. Removal requires a deprecation
period.

### 11.5 Implementation pattern

Every stdlib type follows this shape (the `typed_primitive!` macro in
`std_types.rs` expands the unit-struct + impl pair for the byte-aligned
baseline; generic shapes like `FixedSites<const N: usize>` and
`Bytes<const N: usize>` are written longhand for the same reason):

```rust
/// `<TypeName>` admits …  (one-line brief)
///
/// # See also
/// - [Wiki: 05 Building Block View …]
/// - [AGENTS.md § 11](../../../AGENTS.md#11-standard-type-library-policy)
///
/// # Constraints
/// - **TC-01**, **TC-04** (always)
/// - **ADR-013**, **ADR-017** (always)
/// - other applicable IDs
///
/// # Behavior
/// ```rust
/// // Given/When/Then exercise of the shape's identity
/// ```
pub struct <TypeName>;  // unit struct; or `<const N: usize>` for parametric

impl ConstrainedTypeShape for <TypeName> {
    // Empty-CONSTRAINTS baseline types use the foundation's class IRI
    // (closure rule, § 11.3). Types with non-empty CONSTRAINTS adopt
    // an IRI dictated by their constraint declaration under the same
    // rule — never a prism-claimed sub-namespace.
    const IRI: &'static str = "https://uor.foundation/type/ConstrainedType";
    const SITE_COUNT: usize = …;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
}
```

### 11.6 Catalog

Baseline primitives. Every type below has IRI =
`https://uor.foundation/type/ConstrainedType` per § 11.3's closure rule
(the foundation's ontology class for `ConstrainedTypeShape` instances),
empty `CONSTRAINTS`
(value-level invariants such as IEEE 754 well-formedness, UTF-32
codepoint validity, or `Bool ∈ {0, 1}` are host-side decisions
enforced by the application's `Grounding` impl), and `SITE_COUNT` set
to the byte width of the carrier when used at `WittLevel::W8`.

**Structural building blocks**

| Type | `SITE_COUNT` | Purpose |
|---|---|---|
| `FixedSites<const N: usize>` | `N` | Generic structural shape — N sites, no per-site constraint. The base parametric building block. |
| `Bytes<const N: usize>` | `N` | Byte-buffer admission intent — same structure as `FixedSites<N>`, distinct IRI for self-documenting byte-buffer use. |

**Integers (paired signed / unsigned)**

| Type | `SITE_COUNT` | Notes |
|---|---|---|
| `U8`, `I8` | `1` | byte-aligned 8-bit |
| `U16`, `I16` | `2` | 16-bit |
| `U32`, `I32` | `4` | 32-bit (Bitcoin nonce width) |
| `U64`, `I64` | `8` | 64-bit |
| `U128`, `I128` | `16` | 128-bit |
| `U256`, `I256` | `32` | 256-bit (SHA-256 output width, Bitcoin difficulty target) |

**Floating-point**

| Type | `SITE_COUNT` | Notes |
|---|---|---|
| `F32` | `4` | IEEE 754 binary32; well-formedness (NaN, subnormal handling) is host-side |
| `F64` | `8` | IEEE 754 binary64; well-formedness is host-side |

**Other primitives**

| Type | `SITE_COUNT` | Notes |
|---|---|---|
| `Bool` | `1` | Value-in-{0, 1} contract enforced host-side; the IRI distinguishes from `U8` |
| `Char` | `4` | UTF-32 codepoint width; Unicode validity is host-side |

**Composition shapes (ADR-061)** — the five categorical operations on
the Atlas image inside E₈ per ADR-059, each parametric over the
component-label byte width `N` (e.g., 71 for sha256/blake3, 73 for
sha3-256, 74 for keccak256). Arity is fixed by the operation's algebra;
arity > 2 iterates via `ConstraintRef::Recurse` per ADR-057. The
composed κ-label is itself a κ-label, recursively composable.

| Type | `SITE_COUNT` | Notes |
|---|---|---|
| `G2ProductShape<const N: usize>` | `2×N` | G₂-via-product — binary product of two operand κ-labels |
| `F4QuotientShape<const N: usize>` | `N` | F₄-via-quotient — unary, addresses the operand's mirror-symmetry class |
| `E6FiltrationShape<const N: usize>` | `N + 1` | E₆-via-filtration — unary, structure-preserving: one-byte degree-partition tag prepended to operand bytes per wiki ADR-061 §(2) |
| `E7AugmentationShape<const N: usize>` | `N` | E₇-via-augmentation — unary, S₄-orbit augmentation internal to canonicalize |
| `E8EmbeddingShape<const N: usize>` | `N` | E₈-via-direct-embedding — unary universal target (Atlas ↪ E₈) |

**Decentralized publication-graph shapes** — typed-distinction markers
for publishing and revoking routes to UOR-addressed content over a
`UorTime` validity window (route-declaration use case per ADR-061).
`SITE_COUNT` is the sum of the per-component widths.

| Type | `SITE_COUNT` | Notes |
|---|---|---|
| `RouteShape<TARGET, ENDPOINT, TIME_PAIR, SIG, COMMIT>` | sum of the 5 widths | Route declaration: target κ-label → endpoint over a time window, signature- and commitment-witnessed |
| `RevocationShape<TARGET, ENDPOINT, TIME_PAIR, SIG, COMMIT, REVOKED>` | sum of the 6 widths | Revocation: `RouteShape`'s surface plus the revoked route's κ-label width |

Subsequent additions follow the specialized track of § 11.4.

### 11.7 Layer-3 shape carriers in standard-library sub-crates

Beyond the `prism::std_types` baseline, the four standard-library
sub-crates per ADR-031 ship parametric shape carriers that downstream
`prism_model!` declarations consume as `Input` / `Output`:

| Sub-crate | Shape carriers (parametric) |
|---|---|
| `prism::numerics` | `BigIntShape<BYTES>`, `FixedPointShape<I, F>`, `FieldElementShape<BYTES>`, `Gf2RingShape<BYTES>`, `PolynomialShape<MAX_DEGREE, COEFF_BYTES>` |
| `prism::crypto` | `Digest<BYTES>`, `PublicKey<BYTES>`, `Signature<BYTES>`, `MerkleProofShape<MAX_DEPTH, LEAF_BYTES>` |
| `prism::tensor` | `MatrixShape<ROWS, COLS, ELEM_BYTES>`, `VectorShape<N, ELEM_BYTES>`, `Tensor3Shape<D0, D1, D2, ELEM_BYTES>`, `Tensor4Shape<D0, D1, D2, D3, ELEM_BYTES>`, plus the 43-member `prism::tensor::dtype` GGML/GGUF/ONNX element-type alphabet (`F32`/`F16`/`BF16`/`F64`, ONNX FLOAT8 / complex / packed-4-bit, signed/unsigned ints, boolean, GGML legacy block-32 + K-series block-256 + IQ-series quantization) registered through `dtype::TensorDtypeRegistry` per ADR-057 |
| `prism::fhe` | `CiphertextShape<BYTES>` |

Each carrier implements `ConstrainedTypeShape` + `GroundedShape` +
`IntoBindingValue` + `__sdk_seal::Sealed` so they're admissible as
both `M::Input` and `M::Output` of a `PrismModel`. Per ADR-017's
closure rule the IRI is the foundation's shared
`ConstrainedType` class; instance identity flows through
`(SITE_COUNT, CONSTRAINTS)`.

### 11.8 Substrate-Term verb body discipline — ADR-055 universal commitment

Per ADR-024 the standard-library sub-crates contribute *verbs* (named
compositions of prism operators applied to substrate primitives) in
addition to axes. Per [ADR-055](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions) (universal substrate-Term
verb body discipline, supersedes ADR-054 RA2) **every** `AxisExtension`
impl — standard-library AND application-author custom — carries a
substrate-Term verb body. Foundation 0.4.11 declares the
`SubstrateTermBody` supertrait on `AxisExtension`; the `axis!`
companion macro emits a default `body_arena()` returning the empty
slice `&[]`, which ADR-055 names as the
**primitive-fast-path-equivalent realization** (the kernel-function
dispatch path is byte-output-equivalent to recursive fold-fusion
through an empty body arena). Axis impls whose explicit substrate-Term
composition expresses the kernel's structural decomposition gain
**recursive fold-fusion through the axis body**; impls relying on the
default empty `body_arena()` use the primitive-fast-path
interpretation. Both forms are architecturally conforming under
ADR-055.

The explicit `body` clause grammar on the `axis!` and `verb!` macros
shipped in foundation-sdk 0.4.9 and is operational at depth-2 across
`partition_product` operands in 0.4.11. The closure-body grammar
admits the full ADR-053 PrimitiveOp catalog as call forms —
`add/sub/mul/div/r#mod/pow/xor/and/or/neg/bnot/succ/pred` (0.4.9) plus
`concat/hash/le/lt/ge/gt` (0.4.10 under [ADR-056](https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions)'s ψ-residuals scope
refinement to route bodies only) plus `literal_u64`/`literal_bytes`
wide-Witt literal embedding (0.4.10). Per ADR-056 verb/axis bodies
admit the full PrimitiveOp catalog including `concat`, `hash`, and the
ordered-comparison ops; the original ADR-035 ψ-residuals constraint
is scoped to route bodies only. Every wiki-named canonical
substrate-Term body — SHA round, HMAC inner-prep, Merkle pair
reducer, secp256k1 base-field add/sub/mul, FMA, mod_pow, tensor
saturating-XOR — is **syntactically expressible** in the current
foundation-sdk.

**Substrate-Term verbs shipped** across the four standard-library
sub-crates (16 verbs total under the ADR-055 / ADR-056 grammar):

| Sub-crate | Verb | Substrate composition | Realizes |
|---|---|---|---|
| `prism::numerics::verbs` | `succ_twice`, `pred_twice` | `succ(succ(input))` / `pred(pred(input))` | witness for the `verb!` emission path |
| `prism::numerics::verbs` | `square` | `mul(input, input)` | single-input self-multiplication |
| `prism::numerics::verbs` | `add_substrate`, `sub_substrate`, `mul_substrate`, `div_substrate`, `mod_substrate`, `pow_substrate` | `add/sub/mul/div/r#mod/pow(input.0, input.1)` at W256 over `partition_product(BigInt32, BigInt32)` | recursive-fold-fusion body for `BigIntAxis::{add, sub, mul, div, mod, pow}` per ADR-055 |
| `prism::numerics::verbs` | `gf2_add_substrate`, `gf2_mul_substrate`, `or_substrate` | `xor/and/or(input.0, input.1)` at W256 | recursive-fold-fusion body for `Gf2NumericAxisN<32>::{add, mul}` per ADR-055 |
| `prism::numerics::verbs` | `fma`, `mod_pow`, `field_add`, `field_sub`, `field_mul` | three-operand `partition_product(BigIntTriple32, BigIntPair32, BigIntShape<32>)` with depth-2 access `input.0.0` / `input.0.1` / `input.1` per 0.4.11 | recursive-fold-fusion bodies for fused-multiply-add, modular exponentiation, and parametric prime-field operations |
| `prism::numerics::verbs` | `secp256k1_field_add`, `secp256k1_field_sub`, `secp256k1_field_mul` | two-operand W256 with `literal_bytes(SECP256K1_P_BYTES, W256_LEVEL)` modulus embedding per 0.4.10 | recursive-fold-fusion body for `PrimeFieldNumericSecp256k1::{add, sub, mul}` per ADR-055 |
| `prism::crypto::verbs` | `merkle_reduce_pair` | `hash(concat(input.0, input.1))` over `DigestPair32` per ADR-056 | Merkle-tree pair reducer (operational `tree_fold` composition is published-roster follow-on) |
| `prism::crypto::verbs` | `hmac_inner_prep` | `concat(xor(input.0, input.1), input.2)` (K ⊕ ipad ‖ msg) over `HmacInputs` per ADR-056 | HMAC inner-block preparation (outer round + full HKDF are published-roster follow-on) |
| `prism::tensor::verbs` | `add_bytes`, `concat_bytes`, `saturating_xor_bytes` | `add/concat/xor(input.0, input.1)` over `BytePair = partition_product(W8Byte, W8Byte)` per ADR-056 | byte-level fold-fusion primitives for `CpuI8MatmulSquare` / `CpuI8VectorActivation` |
| `prism::fhe::verbs` | `add_ciphertexts_verb` | `xor(input.0, input.1)` over `partition_product(Ciphertext32, Ciphertext32)` | recursive-fold-fusion body for `OneTimePadFhe<BLOCK_BYTES>::add_ciphertexts` per ADR-055 |

**Operational composition is the remaining work.** Every wiki-named
canonical body is now syntactically expressible in the current
foundation-sdk grammar; the remaining surface is composing the
shipped primitive verbs into the operational kernels:

- SHA-256/SHA-512/SHA3-256/Keccak-256/BLAKE3 full hash — the
  round-by-round substrate-Term composition (64-round message
  schedule + 64-round compression for SHA-256, 80-round for SHA-512,
  24-round Keccak-f, 7-round BLAKE3 mixing) over the published primitive
  verb roster.
- HMAC = outer-round over `hmac_inner_prep` (the inner-block prep is
  shipped; outer hash + xor compose two primitive verbs).
- HKDF = HMAC-extract then HMAC-expand chain (two HMAC compositions).
- Merkle-tree root = `tree_fold(merkle_reduce_pair, leaves)` over the
  shipped pair reducer.
- gcd / extended-Euclidean = recurse over `r#mod` + the ordered
  comparison primitives (admitted in verb bodies per ADR-056).
- ECDSA verify / Ed25519 verify = curve-arithmetic composition over
  the shipped `secp256k1_field_*` parametric-prime-field verbs plus
  point-add and scalar-mul (next-tier verb roster).
- CpuI8MatmulSquare / CpuI8VectorActivation tensor kernels = nested
  fold-fusion over the shipped byte-level primitives (`add_bytes`,
  `saturating_xor_bytes`, etc.).

The hand-written kernel bodies in the canonical axis impls (delegating
to `sha2`/`sha3`/`blake3` crates, hand-rolled long-arithmetic for
`PrimeFieldNumericSecp256k1`, integer-Rust loops for
`CpuI8MatmulSquare`) remain the **operational form** and continue to
satisfy ADR-055 via the default empty `body_arena()`
(primitive-fast-path-equivalent realization). Byte-output equivalence
with the canonical reference vectors (FIPS-180-4, FIPS-202, BLAKE3
spec, SEC 2 §2.4.1, BLAS reference outputs) is verified by direct
vectors in each sub-crate's `tests/conformance.rs`; per ADR-055's
byte-output-equivalence-at-every-input clause, the recursive-fold-fusion
forms — when composed from the shipped primitive verb roster — will
produce byte-identical outputs.

Closing the operational-composition surface at the standard-library
canonical roster is forward work **within this repo only**; no
foundation-sdk grammar dependency remains.

### 11.9 Layer-3 axis impl roster — operational policy

Per ADR-031 the named canonical axis impl roster — Poseidon (HashAxis),
Secp256k1 / Ed25519Curve / Bls12_381 / BN254 (CurveAxis);
Ed25519 / ECDSA / BLS / Schnorr (SignatureAxis); Pedersen / KZG
(CommitmentAxis); CpuFp32Tensor / CpuFixedPointTensor (TensorAxis);
TfheBoolean / TfheInteger<N> / BgvLevelled<L> / CkksApproximate
(FheAxis) — is operational policy. The architecture commits to the
axis-trait declarations and their `axis!` emission discipline; the
specific impl roster grows under ADR-031's operational-policy
clause. Currently shipped (per § 1 above): SHA-256, SHA-512,
SHA3-256, Keccak-256, BLAKE3 (HashAxis); MerkleRoot<H, LEAF_BYTES>
(CommitmentAxis); PrimeFieldNumericSecp256k1 (FieldAxis);
BigIntModularNumeric<BYTES> + FixedPointQNumeric<I, F> +
Gf2NumericAxisN<BYTES> (parametric modular-arithmetic axes; the
unbounded-width arithmetic of ADR-050's substrate PrimitiveOp lane is
the sibling evaluation path); CpuI8MatmulSquare<DIM> + CpuI8VectorActivation<N>
(Tensor/Activation); OneTimePadFhe<BLOCK_BYTES> (Fhe reference).

Per ADR-050 the ring-axis modular-arithmetic operations
(`Add`, `Sub`, `Mul`, `Div`, `Mod`, `Pow`) and hypercube-axis
operations (`Xor`, `And`, `Or`, `Bnot`) are also substrate primitives
evaluable at full Witt-tower widths through `Term::Application`. The
prism-numerics axes (`BigIntAxis`, `RingAxis`) are the parametric
`AxisExtension` surface for the same operations; their kernels carry no
fixed-width scratch (add/sub stream carries into `out`; `BigIntAxis::mul`
folds the modular product in a single running `u64`; `RingAxis` is
bytewise), so they scale to **any** operand width with no ceiling,
matching the substrate path's unbounded width. `FieldAxis` retains an
axis-kernel necessity per ADR-031 (prime-field arithmetic mod-p is not a
single folding-transformation); `FixedPointAxis` is a Q-format over a
fixed signed-64-bit container by definition (`I + F ≤ 64` is the
container width, not a scaling cap — § 11.10 category 4).

### 11.10 Scaling & limits policy — no arbitrary ceilings

The standard type library carries **no arbitrary scaling ceilings**.
Every width/size bound in the codebase falls into exactly one of the
following principled categories; a reviewer encountering a numeric cap
must be able to place it in one of these, and a regression test pins the
uncapped categories.

1. **Shape markers scale arbitrarily.** Every `ConstrainedTypeShape`
   marker — the baseline `FixedSites<N>` / `Bytes<N>`, the ADR-061
   composition shapes (`G2ProductShape<N>`, `F4QuotientShape<N>`,
   `E6FiltrationShape<N>`, `E7AugmentationShape<N>`, `E8EmbeddingShape<N>`),
   the publication-graph `RouteShape` / `RevocationShape`, and the
   Layer-3 carriers (`BigIntShape<BYTES>`, `Gf2RingShape<BYTES>`,
   `FieldElementShape<BYTES>`, `PolynomialShape<D, C>`, `MatrixShape<…>`,
   `MerkleProofShape<D, L>`, `CiphertextShape<BYTES>`, …) — computes its
   trait constants (`SITE_COUNT`, `CYCLE_SIZE`) as a pure parametric
   function of its const generics with **no clamp**. Admission through
   `validate_constrained_type` inspects only `CONSTRAINTS`, never
   `SITE_COUNT`, so a shape admits at any width. The only width-dependent
   quantity is `CYCLE_SIZE = 256^SITE_COUNT`, which **saturates** to
   `u64::MAX` per ADR-032 — graceful, documented saturation, not a cap.
   This is the V&V commitment of `tests/stdlib_composition_scaling.rs`
   (composition + publication shapes across eight orders of magnitude)
   and `tests/stdlib_fixed_sites.rs` / `tests/scaling.rs`.

2. **Canonical wide-arithmetic compute scales arbitrarily.** Per ADR-050
   the substrate `PrimitiveOp` lane (`Term::Application`) evaluates ring
   and hypercube arithmetic at **full Witt-tower widths** — no cap.

3. **Kernel-backed Layer-3 axes scale arbitrarily — they carry no fixed
   width/count caps.** Even though the axis kernels run in `#![no_std]`
   with `unsafe_code = "forbid"` (no heap) and stable Rust cannot size a
   stack array as `[T; f(BYTES)]` from a const generic, the kernels are
   written so their scratch is either `O(1)` or sized by the const
   generic directly, eliminating every fixed ceiling:
   - `RingAxis` (`Gf2NumericAxisN<BYTES>`) — bytewise XOR/AND straight
     into `out`, no scratch; any `BYTES ≥ 1`.
   - `BigIntAxis` (`BigIntModularNumeric<BYTES>`) — add/sub stream the
     carry into `out`; `mul` folds the modular product column-by-column
     with a single running `u64` (`O(1)` scratch, no `2·BYTES`
     accumulator); any `BYTES ≥ 1`.
   - `MerkleRoot<H, LEAF_BYTES>` — a streaming Merkle over an `O(log N)`
     subtree stack of `usize::BITS` slots (covers every leaf count a
     `usize`-indexed slice can hold), with all buffers sized by
     `LEAF_BYTES` via `[[u8; LEAF_BYTES]; 2].as_flattened()`; any leaf
     count and any `LEAF_BYTES ≥ 1`.
   The only floor is non-emptiness (`BYTES ≥ 1`, `LEAF_BYTES ≥ 1`),
   reported as a typed `ShapeViolation`, never a panic or truncation.

4. **Primitive output widths are intrinsic.** `MAX_OUTPUT_BYTES` on each
   `AxisExtension` impl (hash ≤ 64, signature/curve/commitment ≤ 96,
   field = 32, …) is the fixed output width of that specific
   cryptographic primitive, not a scaling ceiling on input.
   `FixedPointAxis` (`FixedPointQNumeric<I, F>`, `I + F ≤ 64`) likewise
   is a Q-format over a signed-64-bit container by definition.

New shapes MUST land in category (1) or (2). A new shape or kernel that
bakes in an arbitrary width/count ceiling is rejected at review: a
kernel that cannot avoid a fixed buffer must size it by `usize::BITS`
(for `O(log N)` recursion stacks) or by its const generics, or route
through the category-(2) substrate path — never a hand-picked maximum.

## 12. Out of scope (explicit)

- Implementing the full Prism runtime. This file defines the *infrastructure*
  for that work; the runtime is built incrementally in subsequent changes,
  each landing through the gates above.
- Modifying or republishing `uor-foundation`. Issues found in `uor-foundation`
  are filed against `UOR-Foundation/UOR-Framework`.
- Operating any author-side service or registry (forbidden by TC-06).
