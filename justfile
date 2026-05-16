default: lint test

dev:
    cargo build --workspace

build-release:
    cargo build --workspace --release

test:
    cargo test --workspace --all-features

doc:
    RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links -D rustdoc::missing_crate_level_docs" \
        cargo doc --workspace --no-deps

doc-open: doc
    cargo doc --workspace --no-deps --open

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all --check

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

lint-wiki:
    cargo run -p wiki-link-check

lint: fmt-check clippy doc lint-wiki

no-std:
    cargo build -p uor-prism --target thumbv7em-none-eabihf --no-default-features
    cargo build -p uor-prism-verify --target thumbv7em-none-eabihf --no-default-features
    cargo build -p uor-prism-crypto --target thumbv7em-none-eabihf --no-default-features
    cargo build -p uor-prism-numerics --target thumbv7em-none-eabihf --no-default-features
    cargo build -p uor-prism-tensor --target thumbv7em-none-eabihf --no-default-features
    cargo build -p uor-prism-fhe --target thumbv7em-none-eabihf --no-default-features

deny:
    cargo deny check

# Dry-run publish in ADR-031 dependency-graph order (leaf sub-crates
# first, then tensor → prism → verify). Non-leaf entries pass
# --no-verify because their workspace-path deps aren't on the
# registry; the real release publish does the verify pass. The
# --allow-dirty flag matches the release-workflow pattern (the cargo
# cache restored on CI produces an un-stamped lockfile post-restore).
publish-dry:
    cargo publish --dry-run --allow-dirty -p uor-prism-numerics
    cargo publish --dry-run --allow-dirty -p uor-prism-crypto
    cargo publish --dry-run --allow-dirty -p uor-prism-fhe
    cargo publish --dry-run --allow-dirty --no-verify -p uor-prism-tensor
    cargo publish --dry-run --allow-dirty --no-verify -p uor-prism
    cargo publish --dry-run --allow-dirty --no-verify -p uor-prism-verify

# Tag a release: `just tag-release 0.1.1` creates and pushes the
# `v0.1.1` tag. The Cargo workspace version must already be bumped
# in `[workspace.package]` before tagging — the release workflow's
# tag-validation step rejects mismatches.
tag-release version:
    @if ! grep -q '^version = "{{version}}"' Cargo.toml; then \
      echo "error: workspace version in Cargo.toml is not {{version}}" >&2; \
      grep '^version = ' Cargo.toml >&2; \
      exit 1; \
    fi
    git tag -s "v{{version}}" -m "Release v{{version}}"
    @echo "Created signed tag v{{version}}. Push with: git push origin v{{version}}"
