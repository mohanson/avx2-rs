bench-c-avx2:
	gcc -mavx2 -o /tmp/avx2 c/avx2.c
	/tmp/avx2
