clean:
	cargo clean

build:
	cargo build --release -j 8
	
test:
	cargo test