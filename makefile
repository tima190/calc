all: clean build

build:
	cargo build --release
run:
	cargo run --release

clean:
	cargo clean
	rm -rf target/
