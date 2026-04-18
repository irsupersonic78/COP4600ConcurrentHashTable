all:
	cargo build --release
	cp target/release/chash ./chash

clean:
	cargo clean
	rm -f chash