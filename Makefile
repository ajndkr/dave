.ONESHELL:
.SHELLFLAGS = -ec
.PHONY: help ci install bump clean

help:		## help menu
	grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

init:		## initialize project for local development
	echo "\033[1mchecking rust, rustup and cargo installation\033[0m"
	which rustc > /dev/null || (echo "rust is not installed. Please install rust from https://rustup.rs/"; exit 1)
	rustup --version > /dev/null || (echo "rustup is not installed. Please install rust from https://rustup.rs/"; exit 1)
	cargo --version > /dev/null || (echo "cargo is not installed. Please install rust from https://rustup.rs/"; exit 1)

	echo "\033[1minstalling additional dependencies\033[0m"
	rustup component add rustfmt
	rustup component add clippy
	cargo install cargo-audit
	cargo install cargo-edit

ci:			## run ci checks (formatting, compilation checks, linting and security checks)
	cargo fmt --all
	cargo check --all-targets --all-features
	cargo clippy --all-targets --all-features -- -D warnings
	cargo audit

install:	## install project
		cargo install --path .

bump:       ## bump version
	if [ "$(type)" != "major" ] && [ "$(type)" != "minor" ] && [ "$(type)" != "patch" ]; then \
		echo "Usage: make bump type=(major|minor|patch)"; \
		exit 1; \
	fi
	old_version=$$(cargo pkgid | cut -d# -f2); \
	cargo set-version --bump $(type); \
	new_version=$$(cargo pkgid | cut -d# -f2); \
	git commit -am "bump: $$old_version → $$new_version"

clean:		## clean project
	cargo clean
