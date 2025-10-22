build:
	cargo build --release

build-pi:
	rustup target add aarch64-unknown-linux-gnu
ifeq ($(shell uname -s),Linux)
	cargo build --release --target aarch64-unknown-linux-gnu
else ifeq ($(shell uname -s),Darwin)
	cargo zigbuild --release --target aarch64-unknown-linux-gnu
endif
