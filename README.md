```sh
$ cargo bench

u64_mul_rust            time:   [115.04 us 115.40 us 115.84 us]
u64_mul_auto_avx2       time:   [112.63 us 112.91 us 113.26 us]
u64_mul_hand_avx2       time:   [113.79 us 114.04 us 114.42 us]

$ make bench-c-avx2

u64_mul_c    3.820440 s
u64_mul_avx2 3.627868 s
```
