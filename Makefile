.PHONY: all

all:
	rustc -A dead-code src/rusty_erl.rs
#	cargo build --verbose
