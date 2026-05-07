# Repo definition — `UOR-Foundation/prism`

This document is the canonical definition of this repository. Anything in
the working tree that contradicts this file is a bug in the working tree;
anything missing from this file is out of scope.

## 1. Purpose

This repository is the source and publishing pipeline for two Rust crates
that together realize the **Prism** system specified by the
[UOR-Framework wiki][wiki]:

| Cargo package      | Library (import) name | Role                                                                                         |
|--------------------|-----------------------|----------------------------------------------------------------------------------------------|
| `uor-prism`        | `prism`               | Pipeline runtime, three sealed Prism-mechanism types, replay machinery, operation vocabulary |
| `uor-prism-verify` | `prism_verify`        | Replay façade for verifiers                                                                  |

The `uor-` prefix on the package names is forced because the bare name
`prism` on crates.io is already occupied by an unrelated crate. Inside
Rust source, the import path and module names track wiki nomenclature
exactly: `use prism::pipeline::run;` and `use prism_verify::certify_from_trace;`.

The substrate crate `uor-foundation` is consumed unmodified as a normal
crates.io dependency. This repository does not fork or vendor it.

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
- **Architecture decision records** ADR-001 through ADR-023 —
  see wiki page 09. The most architecturally load-bearing recent
  additions: **ADR-018** (`HostBounds` capacity completeness — third
  substitution axis); **ADR-019** (foundation is a closed signature
  endofunctor, `Term` is its initial algebra, `pipeline::run` is the
  catamorphism); **ADR-020** (`PrismModel` is the application author's
  typed-iso contract, sealed by foundation, derived by the
  `prism_model!` macro from `uor-foundation-sdk`); **ADR-022**
  (`PrismModel` implementation surface decisions, including `run_route`
  as the canonical model-execution entry point); **ADR-023**
  (`M::Input` value flow into the `CompileUnit` binding table via
  `IntoBindingValue`).

Substitution axes (the only permitted variation points): `HostTypes`,
`HostBounds`, `Hasher`.

## 3. Layout

```
.
├── AGENTS.md                          # this file (canonical repo definition)
├── CONTRIBUTING.md                    # human-facing pointer to AGENTS.md
├── Cargo.toml                         # workspace manifest, shared profile + lints
├── LICENSE                            # MIT
├── README.md                          # public-facing overview
├── crates
│   ├── uor-prism
│   │   ├── Cargo.toml                 # package = uor-prism, lib.name = prism
│   │   └── src/lib.rs
│   └── uor-prism-verify
│       ├── Cargo.toml                 # package = uor-prism-verify, lib.name = prism_verify
│       └── src/lib.rs
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
- **MSRV**: 1.83 (matches `uor-foundation` v0.3.1's declared
  `rust-version`, which corrects v0.3.0's stale 1.81 declaration).
  Pinned via `rust-toolchain.toml`, which the Rust toolchain enforces
  on every cargo invocation in this workspace. Per
  [TR-09](https://github.com/UOR-Foundation/UOR-Framework/wiki/11-Technical-Risks#tr-09--prism-version-pin-lag-against-uor-foundation),
  `prism`'s pin on `uor-foundation` may lag the latest published
  version; updates to this repo are demand-driven (a needed surface
  change) rather than calendar-driven.
- **`uor-foundation`**: `^0.3` (effective floor 0.3.2 — required for
  `PrismModel`/`run_route`/`FoundationClosed`/`IntoBindingValue` per
  ADR-020 + ADR-022 + ADR-023), `default-features = false`,
  `no_std`-clean
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
[wiki page 05 § Whitebox `prism-verify`][05-verify]:
`pipeline`, `seal`, `replay`, `operation`, `std_types`,
`vocabulary` (re-exports). Adding a top-level module that has no
counterpart in the wiki is forbidden.

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

Tag-driven on `v*`. Steps, in order:

1. Re-run the full CI matrix; any failure aborts publish.
2. `cargo publish --dry-run -p uor-prism`
3. `cargo publish --dry-run -p uor-prism-verify`
4. `cargo publish -p uor-prism`
5. Wait for crates.io index propagation.
6. `cargo publish -p uor-prism-verify` (depends on #4)

Secrets required: `CRATES_IO_TOKEN`.

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

`prism::std_types` realizes the wiki's
[Building Block View § Whitebox `prism`](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)
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
   Speculation alone is not sufficient.
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

## 12. Out of scope (explicit)

- Implementing the full Prism runtime. This file defines the *infrastructure*
  for that work; the runtime is built incrementally in subsequent changes,
  each landing through the gates above.
- Modifying or republishing `uor-foundation`. Issues found in `uor-foundation`
  are filed against `UOR-Foundation/UOR-Framework`.
- Operating any author-side service or registry (forbidden by TC-06).
