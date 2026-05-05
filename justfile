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
    cargo clippy --workspace --all-targets -- -D warnings

lint-wiki:
    cargo run -p wiki-link-check

lint: fmt-check clippy doc lint-wiki

no-std:
    cargo build -p uor-prism --target thumbv7em-none-eabihf --no-default-features
    cargo build -p uor-prism-verify --target thumbv7em-none-eabihf --no-default-features

deny:
    cargo deny check

publish-dry:
    cargo publish --dry-run -p uor-prism
    cargo publish --dry-run -p uor-prism-verify
