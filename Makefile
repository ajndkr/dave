.PHONY: help check fmt lint all dev clean

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

fmt:	## format code
	cargo fmt

lint:	## lint code
	cargo clippy

check:	## check for errors
	cargo check

ci: fmt lint check	## run commit ci checks

dev:	## build and run project
	cargo run

clean:	## clean project
	cargo clean
