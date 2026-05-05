# Prism

Source repository and publishing pipeline for the **`uor-prism`** and
**`uor-prism-verify`** Rust crates — the implementation of the **Prism**
system specified by the [UOR-Framework wiki][wiki].

| Cargo package      | Library (import) name | Role                                                                                         |
|--------------------|-----------------------|----------------------------------------------------------------------------------------------|
| `uor-prism`        | `prism`               | Pipeline runtime, three sealed Prism-mechanism types, replay machinery, operation vocabulary |
| `uor-prism-verify` | `prism_verify`        | Replay façade for verifiers                                                                  |

The substrate crate [`uor-foundation`](https://crates.io/crates/uor-foundation)
is consumed unmodified as a normal crates.io dependency.

The architecture is defined at the [UOR-Framework wiki][wiki] in arc42 + C4
form, and is normative. Every public item in this codebase backlinks to
the wiki section that defines it; the rustdoc surface forms the C4 view of
the system. Wiki backlinks are mechanically validated in CI by the
workspace tool at [`tools/wiki-link-check`](tools/wiki-link-check/) — a
broken anchor fails the build.

## Repository definition

The canonical definition of this repository (layout, toolchain, CI gates,
release pipeline, documentation conventions) lives in
[`AGENTS.md`](AGENTS.md). Anything not described there is out of scope.

## License

MIT — see [`LICENSE`](LICENSE).

[wiki]: https://github.com/UOR-Foundation/UOR-Framework/wiki
