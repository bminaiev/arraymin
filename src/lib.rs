use std::time::Instant;

use crate::rand::Random;

pub mod rand;

pub fn gen_array() -> Vec<i32> {
    const N: usize = 1_000_000;
    let mut rnd = Random::new(787788);
    (0..N).map(|_| (rnd.gen_u64() & 0xfffffff) as i32).collect()
    // (0..N as i32).rev().collect()
}

#[link(name = "c_impl", kind = "static")]
extern "C" {
    // fn foo();
    fn cpp_min_for_loop(a: *const i32, len: usize) -> i32;
    fn argmin_simd1(a: *const i32, len: usize) -> i32;
    fn argmin_simd2(a: *const i32, len: usize) -> i32;
}

pub fn cpp_impl_find_min_for_loop(a: &[i32]) -> i32 {
    unsafe { cpp_min_for_loop(a.as_ptr(), a.len()) }
}

pub fn cpp_impl_simd1(a: &[i32]) -> i32 {
    unsafe { argmin_simd1(a.as_ptr(), a.len()) }
}

pub fn cpp_impl_simd2(a: &[i32]) -> i32 {
    unsafe { argmin_simd2(a.as_ptr(), a.len()) }
}

pub fn find_min_for_loop(a: &[i32]) -> i32 {
    let mut min = a[0];
    for &x in a.iter() {
        if x < min {
            min = x;
        }
    }
    min
}

pub fn find_min_index_for_loop(a: &[i32]) -> i32 {
    let mut min = a[0];
    for i in 0..a.len() {
        if a[i] < min {
            min = a[i];
        }
    }
    min
}

pub fn find_min_iter(a: &[i32]) -> i32 {
    *a.iter().min().unwrap()
}

pub fn find_min_iter_avx2(a: &[i32]) -> i32 {
    #[target_feature(enable = "avx2")]
    pub unsafe fn run(a: &[i32]) -> i32 {
        find_min_for_loop(a)
    }
    unsafe { run(a) }
}

pub fn find_min_blocks(a: &[i32]) -> i32 {
    const BLOCK_SIZE: usize = 4;
    let mut mins = [i32::MAX; BLOCK_SIZE];
    for block in a.chunks_exact(BLOCK_SIZE) {
        for i in 0..BLOCK_SIZE {
            if block[i] < mins[i] {
                mins[i] = block[i];
            }
        }
    }
    let mut min = mins[0];
    for &x in a.chunks_exact(BLOCK_SIZE).remainder() {
        if x < min {
            min = x;
        }
    }
    for &x in mins.iter() {
        if x < min {
            min = x;
        }
    }
    min
}
