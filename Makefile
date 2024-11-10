.PHONY: help check fmt lint all dev clean

help:		## help menu
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

ci:			## run linters
	cargo fmt
	cargo check
	cargo clippy -- -D warnings

dev:		## build and run project
	cargo run

clean:		## clean project
	cargo clean
