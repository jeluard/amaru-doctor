build:
	cargo build --release

build-pi:
	rustup target add aarch64-unknown-linux-gnu
ifeq ($(shell uname -s),Linux)
	cargo build --release --target aarch64-unknown-linux-gnu
else ifeq ($(shell uname -s),Darwin)
	cargo zigbuild --release --target aarch64-unknown-linux-gnu
endif

run:
	cargo run --release

run-test:
	cd resources/db-creator && cargo run --release
	AMARU_LEDGER_DB=resources/db-creator/ledger.db AMARU_CHAIN_DB=resources/db-creator/chain.db cargo run --release