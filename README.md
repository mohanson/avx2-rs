```sh
$ make bench-avx2-c

u64_mul_c    5.517003 s
u64_mul_avx2 5.526147 s

$ make bench-avx2-rs

u64_mul_rust            time:   [301.41 µs 310.94 µs 320.99 µs]
u64_mul_auto_avx2       time:   [209.14 µs 217.62 µs 226.95 µs]
u64_mul_hand_avx2       time:   [201.07 µs 206.37 µs 212.10 µs]
```
