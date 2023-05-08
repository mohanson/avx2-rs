bench-avx2-c:
	gcc -mavx2 -O2 -o target/debug/avx2 c/avx2.c
	target/debug/avx2

bench-avx2-rs:
	cargo bench
