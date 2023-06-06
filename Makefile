all:
	cd mtgcombo && RUST_BACKTRACE=1 cargo run

release:
	cd mtgcombo && cargo run --release
