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

fn add_vec3(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3{x:u.x + v.x, y:u.y + v.y, z:u.z + v.z}
}

fn add_vec3_optimized(u: &Vec3Optimized, v: &Vec3Optimized) -> Vec3Optimized {
    Vec3Optimized {register: unsafe {_mm_add_ps(u.register, v.register)}}
}

fn main() {
    // Init default options for both benchmarks
    let options = Options::default();

    // Normal vectors
    let u = Vec3{x:1.0, y:2.0, z:3.0};
    let v = Vec3{x:4.0, y:5.0, z:6.0};
    // let w = Vec3{x:0.0, y:0.0, z:0.0};

    // Normal vectors benchmark
    microbench::bench(&options, "add Vec3 normal", || add_vec3(&u, &v));

    // Optimized vectors. Data represented as f32x4 in a _m128, where only the first 3 floats are
    // of interest, so the last one can is simply ignored. The order is flipped, the last value
    // is the most significant one.
    let a = Vec3Optimized{register: unsafe {_mm_set_ps(0.0, 3.0, 2.0, 1.0)}};
    let b = Vec3Optimized{register: unsafe {_mm_set_ps(0.0, 6.0, 5.0, 4.0)}};
    // let c = Vec3Optimized{register: unsafe {_mm_set_ps(0.0, 3.0, 2.0, 1.0)}};

    // Normal vectors benchmark
    microbench::bench(&options, "add Vec3 optimized", || add_vec3_optimized(&a, &b));

}
