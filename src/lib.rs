use core::arch::x86_64::*;

pub fn u64_mul_rust(a: &[u64], b: &[u64], c: &mut [u64]) {
    for ((a, b), c) in a.iter().zip(b).zip(c) {
        *c = *a * *b;
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn u64_mul_auto_avx2(a: &[u64], b: &[u64], c: &mut [u64]) {
    for ((a, b), c) in a.iter().zip(b).zip(c) {
        *c = *a * *b;
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn u64_mul_hand_avx2(a: &[u64], b: &[u64], c: &mut [u64]) {
    for i in 0..a.len() / 4 {
        let a_avx2 = _mm256_loadu_si256(a[4 * i..4 * i + 4].as_ptr() as *const __m256i);
        let b_avx2 = _mm256_loadu_si256(b[4 * i..4 * i + 4].as_ptr() as *const __m256i);
        let r_avx2 = _mm256_mul_epu32(a_avx2, b_avx2);
        _mm256_storeu_si256(c[4 * i..4 * i + 4].as_ptr() as *mut __m256i, r_avx2);
    }
    let o = a.len() / 4 * 4;
    for i in o..a.len() {
        c[i] = a[i] * b[i];
    }
}
