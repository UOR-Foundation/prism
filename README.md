# Prism

Source repository and publishing pipeline for the **Prism standard
library** — the implementation of the **Prism** system specified by
the [UOR-Framework wiki][wiki]. Per wiki [ADR-031][adr-031], `prism` IS
the standard library: a façade re-exporting the `uor-foundation`
substrate together with the Layer-3 sub-crates that contribute the
built-in axes and built-in types it surfaces.

| Cargo package         | Library (import) name | Role                                                                                                          |
|-----------------------|-----------------------|---------------------------------------------------------------------------------------------------------------|
| `uor-prism`           | `prism`               | Standard-library façade. Re-exports foundation + SDK macros + every Layer-3 sub-crate (wiki ADR-031)          |
| `uor-prism-verify`    | `prism_verify`        | Replay façade for verifiers (wiki ADR-005)                                                                    |
| `uor-prism-crypto`    | `prism_crypto`        | Layer-3: `HashAxis` (SHA-2/3, BLAKE3, Keccak) + `CurveAxis` + `SignatureAxis` + `CommitmentAxis`              |
| `uor-prism-numerics`  | `prism_numerics`      | Layer-3: `BigIntAxis` + `FixedPointAxis` + `FieldAxis` (secp256k1 base) + `RingAxis` (GF(2))                  |
| `uor-prism-tensor`    | `prism_tensor`        | Layer-3: `TensorAxis` + `ActivationAxis` (CPU integer-precision reference impls)                              |
| `uor-prism-fhe`       | `prism_fhe`           | Layer-3: `FheAxis` (one-time-pad reference impl)                                                              |

Per ADR-031's façade commitment, application authors depend on
`uor-prism` alone — every standard-library axis and SDK macro is
reachable through `use prism::*`. Examples:

```rust
use prism::pipeline::{prism_model, run_route};
use prism::crypto::Sha256Hasher;
use prism::numerics::BigInt256Numeric;
```

The substrate crates [`uor-foundation`](https://crates.io/crates/uor-foundation)
and [`uor-foundation-sdk`](https://crates.io/crates/uor-foundation-sdk)
are consumed unmodified as normal crates.io dependencies.

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
[adr-031]: https://github.com/UOR-Foundation/UOR-Framework/wiki/09-Architecture-Decisions
