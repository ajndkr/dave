.PHONY: help ci dev install clean

help:		## help menu
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

ci:			## run ci checks (formatting, compilation checks, linting and security checks)
	cargo fmt --all
	cargo check --all-targets --all-features
	cargo clippy --all-targets --all-features -- -D warnings
	cargo audit

dev:		## build and run project
	cargo run -- $(filter-out $@,$(MAKECMDGOALS))

%:
	@:

install:	## install project
	cargo install --path .

clean:		## uninstall and clean project
	cargo uninstall
	cargo clean
