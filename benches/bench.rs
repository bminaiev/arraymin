use arraymin::{
    cpp_impl_find_min_for_loop, cpp_impl_simd1, cpp_impl_simd2, find_min_blocks, find_min_for_loop,
    find_min_index_for_loop, find_min_iter, find_min_iter_avx2, gen_array,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn find_min_iter_bench(c: &mut Criterion) {
    let a = gen_array();
    c.bench_function("find min iter", |b| b.iter(|| find_min_iter(black_box(&a))));
}

pub fn find_min_iter_avx2_bench(c: &mut Criterion) {
    let a = gen_array();
    c.bench_function("find min iter avx2", |b| {
        b.iter(|| find_min_iter_avx2(black_box(&a)))
    });
}

pub fn find_min_for_loop_bench(c: &mut Criterion) {
    let a = gen_array();
    c.bench_function("find min for loop", |b| {
        b.iter(|| find_min_for_loop(black_box(&a)))
    });
}

pub fn find_min_index_for_loop_bench(c: &mut Criterion) {
    let a = gen_array();
    c.bench_function("find min index for loop", |b| {
        b.iter(|| find_min_index_for_loop(black_box(&a)))
    });
}

pub fn find_min_blocks_bench(c: &mut Criterion) {
    let a = gen_array();
    c.bench_function("find min blocks", |b| {
        b.iter(|| find_min_blocks(black_box(&a)))
    });
}

pub fn cpp_for_bench(c: &mut Criterion) {
    let a = gen_array();

    let expected = find_min_for_loop(&a);

    c.bench_function("cpp for bench", |b| {
        b.iter(|| {
            let res = cpp_impl_find_min_for_loop(black_box(&a));
            assert_eq!(res, expected);
        })
    });
}

pub fn cpp_simd1_bench(c: &mut Criterion) {
    let a = gen_array();

    let expected = find_min_for_loop(&a);

    c.bench_function("cpp simd1 bench", |b| {
        b.iter(|| {
            let res = cpp_impl_simd1(black_box(&a));
            assert_eq!(res, expected);
        })
    });
}

pub fn cpp_simd2_bench(c: &mut Criterion) {
    let a = gen_array();

    let expected = find_min_for_loop(&a);

    c.bench_function("cpp simd2 bench", |b| {
        b.iter(|| {
            let res = cpp_impl_simd2(black_box(&a));
            assert_eq!(res, expected);
        })
    });
}

criterion_group!(
    benches,
    find_min_iter_bench,
    find_min_iter_avx2_bench,
    find_min_for_loop_bench,
    find_min_index_for_loop_bench,
    find_min_blocks_bench,
    cpp_for_bench,
    cpp_simd1_bench,
    cpp_simd2_bench,
);
criterion_main!(benches);
