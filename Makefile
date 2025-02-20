# Copyright (c) 2024-2025, Decisym, LLC
# Licensed under the BSD 3-Clause License (see LICENSE file in the project root).

lint:
	cargo install  cargo-machete
	cargo fmt --check
	cargo machete
	cargo clippy --benches --tests --bins --no-deps --all-features

build:
	cargo build

test:
	cargo test

presubmit: lint test
