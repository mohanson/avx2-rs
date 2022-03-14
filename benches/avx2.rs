use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::random;

const SIZE: usize = 1024 * 128;

pub fn u64_mul_rust(c: &mut Criterion) {
    let mut lhs = [0u64; SIZE];
    let mut rhs = [0u64; SIZE];
    let mut ret = [0u64; SIZE];
    for i in 0..SIZE {
        lhs[i] = random::<u32>() as u64;
        rhs[i] = random::<u32>() as u64;
    }
    c.bench_function("u64_mul_rust", |b| {
        b.iter(|| black_box(rs_avx2::u64_mul_rust(&lhs, &rhs, &mut ret)))
    });
}

pub fn u64_mul_auto_avx2(c: &mut Criterion) {
    let mut lhs = [0u64; SIZE];
    let mut rhs = [0u64; SIZE];
    let mut ret = [0u64; SIZE];
    for i in 0..SIZE {
        lhs[i] = random::<u32>() as u64;
        rhs[i] = random::<u32>() as u64;
    }
    c.bench_function("u64_mul_auto_avx2", |b| {
        b.iter(|| black_box(rs_avx2::u64_mul_auto_avx2(&lhs, &rhs, &mut ret)))
    });
}

pub fn u64_mul_hand_avx2(c: &mut Criterion) {
    let mut lhs = [0u64; SIZE];
    let mut rhs = [0u64; SIZE];
    let mut ret = [0u64; SIZE];
    for i in 0..SIZE {
        lhs[i] = random::<u32>() as u64;
        rhs[i] = random::<u32>() as u64;
    }
    c.bench_function("u64_mul_hand_avx2", |b| {
        b.iter(|| black_box(rs_avx2::u64_mul_hand_avx2(&lhs, &rhs, &mut ret)))
    });
}

criterion_group!(benches, u64_mul_rust, u64_mul_auto_avx2, u64_mul_hand_avx2);
criterion_main!(benches);
