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
# registry; the real release publish does the verify pass.
publish-dry:
    cargo publish --dry-run -p uor-prism-numerics
    cargo publish --dry-run -p uor-prism-crypto
    cargo publish --dry-run -p uor-prism-fhe
    cargo publish --dry-run -p uor-prism-tensor --no-verify
    cargo publish --dry-run -p uor-prism --no-verify
    cargo publish --dry-run -p uor-prism-verify --no-verify
