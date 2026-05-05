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
- **Architecture decision records** ADR-001 through ADR-017 —
  see wiki page 09

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
    ├── ci.yml                         # PR + push: fmt, clippy, test, doc, no_std, MSRV, wiki-links, deny
    ├── release.yml                    # tag-driven cargo publish
    ├── docs.yml                       # rustdoc → GitHub Pages
    └── wiki-drift.yml                 # weekly cron: wiki-link-check against wiki HEAD
```

## 4. Toolchain

- **Rust edition**: 2021
- **MSRV**: 1.83 (matches `uor-foundation` v0.3.0; pinned via `rust-toolchain.toml`)
- **`uor-foundation`**: `^0.3`, `default-features = false`, `no_std`-clean
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
| `msrv`       | `cargo +1.83 build --workspace`                                            | MSRV not silently regressed       |
| `doc`        | `RUSTDOCFLAGS='-D rustdoc::broken_intra_doc_links -D rustdoc::missing_crate_level_docs' cargo doc --workspace --no-deps` | C4 view stays linkable          |
| `wiki-links` | `cargo run -p wiki-link-check`                                             | All wiki backlinks resolve        |
| `deny`       | `cargo deny check`                                                         | Licenses + advisories + sources   |

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

## 11. Out of scope (explicit)

- Implementing the full Prism runtime. This file defines the *infrastructure*
  for that work; the runtime is built incrementally in subsequent changes,
  each landing through the gates above.
- Modifying or republishing `uor-foundation`. Issues found in `uor-foundation`
  are filed against `UOR-Foundation/UOR-Framework`.
- Operating any author-side service or registry (forbidden by TC-06).
