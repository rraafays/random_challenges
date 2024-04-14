default: build

build:
	cargo build
	# cp target/debug/chipeight chipeight
clean:
	cargo clean
release:
	cargo build --release
	# cp target/release/chipeight chipeight
dependencies:
	cargo tree
