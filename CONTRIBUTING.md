# Contributing

The canonical repo definition, conventions, and gating rules live in
[`AGENTS.md`](AGENTS.md). Read it once before making your first change —
it is short and load-bearing.

Quick start:

```sh
just dev          # cargo build --workspace
just test         # cargo test --workspace --all-features
just lint         # fmt + clippy + doc + wiki-link-check
just lint-wiki    # only the wiki backlink validator
```

Every public item must carry the five-block doc structure described in
[`AGENTS.md` § 5.1](AGENTS.md#51-required-structure-for-every-pub-item).
Every wiki backlink must resolve under `cargo run -p wiki-link-check`;
broken backlinks fail CI.

Architecture changes belong in the
[UOR-Framework wiki](https://github.com/UOR-Foundation/UOR-Framework/wiki),
not here. This repository implements; the wiki specifies.
