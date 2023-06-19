// #pragma GCC target("avx2,bmi,bmi2,popcnt,lzcnt")

#include <immintrin.h>
#include <limits.h>
#include <smmintrin.h>
#include <stddef.h>
#include <stdint.h>

typedef __m256i reg;

int32_t cpp_min_for_loop(int32_t *a, size_t len) {
  int32_t res = a[0];
  for (int i = 1; i < (int)len; i++) {
    if (a[i] < res) {
      res = a[i];
    }
  }
  return res;
}

int argmin_simd1(int *a, int n) {
  return cpp_min_for_loop(a, n);
  // int min = INT_MAX, idx = 0;

  // reg p = _mm256_set1_epi32(min);

  // for (int i = 0; i < n; i += 8) {
  //   reg y = _mm256_load_si256((reg *)&a[i]);
  //   reg mask = _mm256_cmpgt_epi32(p, y);
  //   if (!_mm256_testz_si256(mask, mask)) {
  //     for (int j = i; j < i + 8; j++)
  //       if (a[j] < min) min = a[idx = j];
  //     p = _mm256_set1_epi32(min);
  //   }
  // }

  // return min;
}

int argmin_simd2(int *a, int n) {
  return cpp_min_for_loop(a, n);
  // int min = INT_MAX;

  // reg p = _mm256_set1_epi32(min);

  // for (int i = 0; i < n; i += 8) {
  //   reg y = _mm256_load_si256((reg *)&a[i]);
  //   p = _mm256_min_epi32(p, y);
  // }
  // int *mins = (int *)&p;
  // for (int i = 0; i < 8; i++) {
  //   if (mins[i] < min) {
  //     min = mins[i];
  //   }
  // }

  // return min;
}