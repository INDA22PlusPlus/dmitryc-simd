use std::arch::x86_64::{__m128, _mm_add_ps, _mm_cvtss_f32, _mm_extract_ps, _mm_set_ps};
use::microbench::*;

#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Vec3Optimized {
    pub register: __m128
}

fn add_vec3(u: &Vec3, v: &Vec3, result: &mut Vec3) {
    result.x = u.x + v.x;
    result.y = u.y + v.y;
    result.z = u.z + v.z;
}

fn add_vec3_optimized(u: &Vec3Optimized, v: &Vec3Optimized, result: &mut Vec3Optimized){
    result.register = unsafe {_mm_add_ps(u.register, v.register)};
}

fn add_float(a: __m128, b: __m128) ->__m128 {
    return unsafe {_mm_add_ps(a, b)};
}

fn main() {
    // Init default options for both benchmarks
    let options = Options::default();

    // Normal vectors
    let u = Vec3{x:1.0, y:2.0, z:3.0};
    let v = Vec3{x:4.0, y:5.0, z:6.0};
    let mut add_result_normal = Vec3{x:0.0, y:0.0, z:0.0};

    // Normal vectors benchmark
    microbench::bench(&options, "add Vec3 normal", || add_vec3(&u, &v, &mut add_result_normal));

    // Should print: "w: (5, 7, 9)"
    println!("w: ({}, {}, {})", add_result_normal.x, add_result_normal.y, add_result_normal.z);

    let a_reg = unsafe {_mm_set_ps(0.0, 3.0, 2.0, 1.0)};
    let b_reg = unsafe {_mm_set_ps(0.0, 6.0, 5.0, 4.0)};

    // Optimized vectors. Data represented as f32x4 in a _m128, where only the first 3 floats are
    // of interest, so the last one can is simply ignored. The order is flipped, the last value
    // is the most significant one.
    let a = Vec3Optimized{register: a_reg};
    let b = Vec3Optimized{register: b_reg};
    let mut add_result_optimized = Vec3Optimized{register: unsafe {_mm_set_ps(0.0, 0.0, 0.0, 0.0)}};

    // Normal vectors benchmark
    microbench::bench(&options, "add Vec3 optimized", || add_vec3_optimized(&a, &b, &mut add_result_optimized));

    microbench::bench(&options, "add pure floats in __m128", || add_float(a_reg, b_reg));
}
