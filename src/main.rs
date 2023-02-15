use std::arch::x86_64::{__m128, _mm_add_ps, _mm_cvtss_f32, _mm_extract_ps, _mm_set_ps};
use::microbench::*;

#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

fn add_vec3_normal(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3{x:u.x + v.x, y:u.y + v.y, z:u.z + v.z}
}

fn main() {
    let u = Vec3{x:1.0, y:2.0, z:3.0};
    let v = Vec3{x:4.0, y:5.0, z:6.0};
    let w = Vec3{x:0.0, y:0.0, z:0.0};

    let options = Options::default();

    microbench::bench(&options, "add Vec3 normal", || add_vec3_normal(&u, &v));

    let _a = unsafe { _mm_set_ps(u.x, u.y, u.z, 1.0) };
    let _b = unsafe { _mm_set_ps(v.x, v.y, v.z, 1.0) };

    let result = unsafe { (_mm_add_ps(_a, _b)) };
    // let ri = unsafe { _mm_extract_ps::<IMM8>(result) };



    println!("{}", unsafe { _mm_cvtss_f32(result) });

    unsafe { (_mm_add_ps(_a, _b)) };

    println!("{}", unsafe { _mm_cvtss_f32(result) });
}
