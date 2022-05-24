
all: cpp

.PHONY: rustplugin
rustplugin:
	cargo build --manifest-path=rustplugin/Cargo.toml

.PHONY: rustcwrapper
rustcwrapper: rustplugin
	cargo build --manifest-path=rustcwrapper/Cargo.toml

.PHONY: cppcwrapper
cppcwrapper: rustcwrapper
	make -C cppcwrapper/

.PHONY: cpp
cpp: cppcwrapper
	make -C cpp/
