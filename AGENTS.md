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
  canonical-hash axes); **ADR-048** (`TypedCommitment` substrate as the
  5th model-declaration parameter — zero-cost typed-bandwidth
  admission composition; `EmptyCommitment` default); **ADR-050**
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
  saturation per ADR-054 + ADR-055).

Substitution axes (the only permitted variation points per ADR-007 /
ADR-030 / ADR-036 / ADR-048): `HostTypes`, `HostBounds`, `AxisTuple`,
`ResolverTuple`, `TypedCommitment`.

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
- **`uor-foundation`**: `^0.4` (effective floor 0.4.11 — required
  for the `partition_product!` macro's `syn::Type` operand admission
  closing the const-generic-leaf depth-2 verb!-macro projection-
  chain gap, the last remaining structural blocker for the
  three-operand canonical numerics roster (`fma`, `mod_pow`,
  parametric `field_*`)). Earlier floors: 0.4.10 shipped the
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
- **`uor-foundation-sdk`**: `^0.4` (effective floor 0.4.11 —
  required for the `partition_product!` macro's `syn::Type` operand
  admission per the const-generic-leaf depth-2 verb!-macro
  projection-chain fix). Earlier floors: 0.4.10 shipped the
  ADR-056 ψ-residual scope refinement and the closure of the three
  Dependencies named in earlier §11.8 revisions; 0.4.9 admitted
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

Subsequent additions follow the specialized track of § 11.4.

### 11.7 Layer-3 shape carriers in standard-library sub-crates

Beyond the `prism::std_types` baseline, the four standard-library
sub-crates per ADR-031 ship parametric shape carriers that downstream
`prism_model!` declarations consume as `Input` / `Output`:

| Sub-crate | Shape carriers (parametric) |
|---|---|
| `prism::numerics` | `BigIntShape<BYTES>`, `FixedPointShape<I, F>`, `FieldElementShape<BYTES>`, `Gf2RingShape<BYTES>`, `PolynomialShape<MAX_DEGREE, COEFF_BYTES>` |
| `prism::crypto` | `Digest<BYTES>`, `PublicKey<BYTES>`, `Signature<BYTES>`, `MerkleProofShape<MAX_DEPTH, LEAF_BYTES>` |
| `prism::tensor` | `MatrixShape<ROWS, COLS, ELEM_BYTES>`, `VectorShape<N, ELEM_BYTES>` |
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
substrate-Term verb body. Foundation 0.4.8 declares the
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

The explicit `body` clause grammar on the `axis!` macro is forthcoming
in a future foundation-sdk release per ADR-055's "the macro's
forthcoming body-clause grammar" framing. Until that lands, the
default empty `body_arena()` (emitted by 0.4.8's companion macro) is
the conformant form. Recursive-fold-fusion bodies are forward work
co-gated on the upstream body-clause grammar and the verb-body
call-form admissions (`div`/`mod`/`pow`/`concat`) ADR-053 added to the
PrimitiveOp catalog.

**Substrate-Term verbs shipped** (covering all 13 in-grammar
`PrimitiveOp` 2-arg call forms after foundation-sdk 0.4.9):

| Sub-crate | Verb | Substrate composition | Realizes |
|---|---|---|---|
| `prism::numerics::verbs` | `succ_twice`, `pred_twice` | `succ(succ(input))` / `pred(pred(input))` | witness for the `verb!` emission path |
| `prism::numerics::verbs` | `square` | `mul(input, input)` | single-input self-multiplication |
| `prism::numerics::verbs` | `add_substrate`, `sub_substrate`, `mul_substrate`, `div_substrate`, `mod_substrate`, `pow_substrate` | `add/sub/mul/div/r#mod/pow(input.0, input.1)` at W256 over a `partition_product(BigInt32, BigInt32)` input | recursive-fold-fusion body for `BigIntAxis::{add, sub, mul, div, mod, pow}` per ADR-055 |
| `prism::numerics::verbs` | `gf2_add_substrate`, `gf2_mul_substrate`, `or_substrate` | `xor(input.0, input.1)`, `and(input.0, input.1)`, `or(input.0, input.1)` at W256 | recursive-fold-fusion body for `Gf2NumericAxisN<32>::{add, mul}` per ADR-055 |
| `prism::fhe::verbs` | `add_ciphertexts_verb` | `xor(input.0, input.1)` over `partition_product(Ciphertext32, Ciphertext32)` | recursive-fold-fusion body for `OneTimePadFhe<32>::add_ciphertexts` per ADR-055 |

**Forward work: richer substrate-Term canonical bodies.** ADR-055's
universal discipline applies to the wiki's full canonical roster:
SHA-256/SHA-512/SHA3-256/Keccak-256/BLAKE3 (prism-crypto's `HashAxis`),
`PrimeFieldNumericSecp256k1::{add, sub, mul}` (prism-numerics'
`FieldAxis`), `CpuI8MatmulSquare`/`CpuI8VectorActivation`
(prism-tensor), plus the compound-verb roster (`modexp_p`, `gcd`,
`ext_euclidean`, HMAC, HKDF, ECDSA, Merkle-tree, etc.). The default
empty `body_arena()` ADR-055 ships satisfies the discipline for every
existing axis impl; explicit substrate-Term decompositions are
forward work co-gated on three remaining infrastructure dependencies:

**Dependency 1 — depth-2 partition-product field access in `verb!`
bodies (foundation-sdk).** Three-operand verbs like `fma(a, b, c)`,
`field_add<P>(a, b, p)`, and `mod_pow(base, exp, p)` need
`input.0.0`/`input.0.1`/`input.1` access on a
`partition_product(Pair, Leaf)`. Foundation-sdk 0.4.9 admits the
syntax but fails the verb!-macro const-eval projection chain with
"index out of bounds: the length is 0 but the index is 0".
`prism_model!` bodies admit this form (smoke-tested at
uor-foundation-sdk/tests/smoke.rs line 1093); the verb!-macro path
needs alignment.

**Dependency 2 — wide-Witt-level `TermValue` literals in verb bodies
(foundation-sdk).** Verbs like `field_*<P>` (where P is the
secp256k1 base-field prime, a 256-bit constant) and `modexp_p`
need to embed wide-Witt literals in the verb body. The closure-body
grammar's `Literal(u64)` form caps at 64-bit values; ADR-051's
wide-`TermValue` carrier exists at the substrate level but isn't yet
surfaced as a verb-body literal-expr form.

**Dependency 3 — architectural ψ-residuals discipline per ADR-035 /
ADR-036.** `concat` (for SHA padding, byte-packing), `hash` (for
HMAC, Merkle-tree's `H(left||right)`), and comparison ops
`le`/`lt`/`ge`/`gt` (for tensor saturation, gcd's branching predicate)
are **architecturally rejected** in verb/axis bodies — this is the
wiki's design constraint per ADR-035 ψ-residuals + ADR-036
resolver-only-hash discipline, not a foundation-sdk gap. Closing
SHA-2/SHA-3/BLAKE3 full hashes (with padding), HMAC, Merkle, and
tensor saturation requires either (a) an ADR-035/036 amendment with
axis-body carveouts, or (b) reframing the canonical decomposition
to avoid these ops, or (c) moving them out of axis bodies into
resolver bodies (which have a different grammar surface).

The closure-body grammar in foundation-sdk 0.4.9
(`emit_term_for_call` at lines 3222-3260) admits
`add/sub/mul/div/r#mod/pow/xor/and/or/neg/bnot/succ/pred` as
PrimitiveOp call forms — the full ADR-053 18-variant catalog minus
the rejected ψ-residuals. The unadmitted forms are:

- `concat` — rejected per ADR-035 ψ-residuals discipline (Dependency
  3 above). Required for SHA's pad-and-finalize composition and for
  tensor sign-extend (`Concat(0x00, operand)` / `Concat(0xff,
  operand)`).
- `hash(...)` — rejected per ADR-036: axis invocation is excluded
  from verb composition; hashes are consumed by resolvers, not
  verb bodies. Required for HMAC's `H(K ⊕ opad || H(K ⊕ ipad ||
  message))` composition and for Merkle-tree's `H(left || right)`
  reducer. (Dependency 3.)
- `le`/`lt`/`ge`/`gt` — rejected per ADR-035. Required for tensor
  saturation (`Match` over `Ge(acc, 0x7fff_W16)`) and `gcd`'s
  branching predicate. (Dependency 3.)

The hand-written kernel bodies in the canonical axis impls (delegating
to `sha2`/`sha3`/`blake3` crates, or to hand-rolled long-arithmetic
for `PrimeFieldNumericSecp256k1`, or to integer-Rust `for`-loops for
`CpuI8MatmulSquare`) are the operational form. Per ADR-055 these
satisfy the universal discipline via the default empty `body_arena()`
(primitive-fast-path-equivalent realization). Each canonical-impl
docstring carries an "ADR-055 substrate-Term verb body — forward
work" section citing the specific upstream grammar dependency for
the richer explicit decomposition. Byte-output equivalence with the
canonical reference vectors (FIPS-180-4, FIPS-202, BLAKE3 spec,
SEC 2 §2.4.1, BLAS reference outputs) is verified by direct vectors
in each sub-crate's `tests/conformance.rs`; per ADR-055's
byte-output-equivalence-at-every-input clause, the substrate-Term
forms — once foundation-sdk's `body` clause grammar admits
`div`/`mod`/`pow`/`concat` and resolves the `hash`-in-axis-body
question — will produce byte-identical outputs.

Closing the richer ADR-055 explicit-body decomposition at the
standard-library canonical surface is forward work split between this
repo (substrate-Term verb bodies for the operations that become
expressible) and upstream `uor-foundation` + `uor-foundation-sdk`
(the `axis!` `body` clause grammar plus the verb-body call-form
admissions for `div`/`mod`/`pow`/`concat`).

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
Gf2NumericAxisN<BYTES> (legacy modular-arithmetic axes preserved
alongside the substrate-native PrimitiveOp evaluation path of
ADR-050); CpuI8MatmulSquare<DIM> + CpuI8VectorActivation<N>
(Tensor/Activation); OneTimePadFhe<BLOCK_BYTES> (Fhe reference).

Per ADR-050 the ring-axis modular-arithmetic operations
(`Add`, `Sub`, `Mul`, `Div`, `Mod`, `Pow`) and hypercube-axis
operations (`Xor`, `And`, `Or`, `Bnot`) are now substrate primitives
evaluable at full Witt-tower widths through `Term::Application`.
The prism-numerics axes that previously hand-coded these
(`BigIntAxis`, `FixedPointAxis`, `RingAxis`) are retained for
back-compatibility and `AxisTuple` parametricity but the wiki's
canonical evaluation path for wide arithmetic is the substrate
PrimitiveOp; only `FieldAxis` retains an axis-kernel necessity per
ADR-031 (prime-field arithmetic mod-p is not a single
folding-transformation).

## 12. Out of scope (explicit)

- Implementing the full Prism runtime. This file defines the *infrastructure*
  for that work; the runtime is built incrementally in subsequent changes,
  each landing through the gates above.
- Modifying or republishing `uor-foundation`. Issues found in `uor-foundation`
  are filed against `UOR-Foundation/UOR-Framework`.
- Operating any author-side service or registry (forbidden by TC-06).
