use rand::prelude::random;

const SIZE: usize = 1024 * 32;

#[test]
fn test_u64_mul() {
    let mut lhs = [0u64; SIZE];
    let mut rhs = [0u64; SIZE];
    let mut ret = [0u64; SIZE];
    for i in 0..SIZE {
        lhs[i] = random::<u32>() as u64;
        rhs[i] = random::<u32>() as u64;
    }
    avx2_rs::u64_mul_rust(&lhs, &rhs, &mut ret);
    for i in 0..SIZE {
        assert_eq!(ret[i], lhs[i] * rhs[i]);
    }
    avx2_rs::u64_mul_auto_avx2(&lhs, &rhs, &mut ret);
    for i in 0..SIZE {
        assert_eq!(ret[i], lhs[i] * rhs[i]);
    }
    avx2_rs::u64_mul_hand_avx2(&lhs, &rhs, &mut ret);
    for i in 0..SIZE {
        assert_eq!(ret[i], lhs[i] * rhs[i]);
    }
}
