#include <immintrin.h>
#include <stdint.h>
#include <stdio.h>
#include <time.h>

// #define ARRAY_LENGTH 8

void u64_mul_c(uint64_t size, const uint64_t *a, const uint64_t *b,
               uint64_t *c) {
  for (uint64_t i = 0; i < size; i++) {
    c[i] = a[i] * b[i];
  }
}

void u64_mul_avx2(uint64_t size, const uint64_t *a, const uint64_t *b,
                  uint64_t *c) {
  for (uint64_t i = 0; i < size / 4; i++) {
    __m256i a_avx2 = _mm256_load_si256((__m256i *)&a[4 * i]);
    __m256i b_avx2 = _mm256_load_si256((__m256i *)&b[4 * i]);
    __m256i c_avx2 = _mm256_mul_epu32(a_avx2, b_avx2);
    _mm256_store_si256((__m256i *)&c[4 * i], c_avx2);
  }
  uint64_t o = size / 4 * 4;
  for (uint64_t i = o; i < size; i++) {
    c[i] = a[i] * b[i];
  }
}

#define SIZE 1024 * 32
#define LOOP 1024 * 32

int main(int argc, char *argv[]) {
  uint64_t a[SIZE] __attribute__((aligned(256))) = {};
  uint64_t b[SIZE] __attribute__((aligned(256))) = {};
  uint64_t c[SIZE] __attribute__((aligned(256))) = {};

  for (uint64_t i = 0; i < SIZE; i++) {
    a[i] = rand();
    b[i] = rand();
  }

  clock_t tic = clock();
  for (uint64_t i = 0; i < LOOP; i++) {
    u64_mul_c(SIZE, &a[0], &b[0], &c[0]);
  }
  clock_t toc = clock();
  printf("u64_mul_c    %f s\n", (double)(toc - tic) / CLOCKS_PER_SEC);

  tic = clock();
  for (uint64_t i = 0; i < LOOP; i++) {
    u64_mul_avx2(SIZE, &a[0], &b[0], &c[0]);
  }
  toc = clock();
  printf("u64_mul_avx2 %f s\n", (double)(toc - tic) / CLOCKS_PER_SEC);

  for (uint64_t i = 0; i < SIZE; i++) {
    if (c[i] != a[i] * b[i]) {
      printf("fail\n");
      return 1;
    }
  }
  printf("done\n");

  return 0;
}
